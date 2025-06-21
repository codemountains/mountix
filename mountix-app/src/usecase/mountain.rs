use crate::model::mountain::{
    MountainBoxSearchQuery, MountainSearchQuery, SearchedBoxMountainResult, SearchedMountain,
    SearchedMountainResult,
};
use mountix_adapter::modules::RepositoriesModuleExt;
use mountix_kernel::model::mountain::{
    MountainBoxSearchCondition, MountainFindException, MountainGetException,
    MountainSearchCondition,
};
use mountix_kernel::model::ErrorCode;
use mountix_kernel::repository::mountain::MountainRepository;
use num::FromPrimitive;
use std::sync::Arc;

pub struct MountainUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> MountainUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn get(&self, id: String) -> Result<Option<SearchedMountain>, MountainGetException> {
        match id.try_into() {
            Ok(id) => match self.repositories.mountain_repository().get(id).await {
                Ok(mountain) => match mountain {
                    Some(mountain) => Ok(Some(mountain.into())),
                    None => Ok(None),
                },
                Err(_) => Err(MountainGetException::new(ErrorCode::ServerError)),
            },
            Err(error_code) => Err(MountainGetException::new(error_code)),
        }
    }

    pub async fn find(
        &self,
        search_query: MountainSearchQuery,
    ) -> Result<SearchedMountainResult, MountainFindException> {
        match MountainSearchCondition::try_from(search_query) {
            Ok(condition) => {
                let offset = condition.skip;
                let condition_limit = condition.limit;

                let mut total = 0u64;
                if let Ok(count) = self
                    .repositories
                    .mountain_repository()
                    .get_count(condition.clone())
                    .await
                {
                    total = count;
                }

                match self
                    .repositories
                    .mountain_repository()
                    .find(condition)
                    .await
                {
                    Ok(mountains) => {
                        let searched_mountains: Vec<SearchedMountain> =
                            mountains.into_iter().map(|m| m.into()).collect();

                        let mut limit: Option<u64> = None;
                        if let Some(limit_value) = condition_limit {
                            if let Some(parsed_limit) = u64::from_i64(limit_value) {
                                limit = Some(parsed_limit);
                            }
                        }

                        Ok(SearchedMountainResult {
                            mountains: searched_mountains,
                            total,
                            offset,
                            limit,
                        })
                    }
                    Err(_) => Err(MountainFindException::new_with_error_code(
                        ErrorCode::ServerError,
                    )),
                }
            }
            Err(error_messages) => Err(MountainFindException::new(
                ErrorCode::InvalidQueryParam,
                error_messages,
            )),
        }
    }

    pub async fn find_box(
        &self,
        search_query: MountainBoxSearchQuery,
    ) -> Result<SearchedBoxMountainResult, MountainFindException> {
        match MountainBoxSearchCondition::try_from(search_query) {
            Ok(condition) => {
                match self
                    .repositories
                    .mountain_repository()
                    .find_box(condition)
                    .await
                {
                    Ok(mountains) => {
                        let searched_mountains: Vec<SearchedMountain> =
                            mountains.into_iter().map(|m| m.into()).collect();
                        let total = searched_mountains.len() as u64;

                        Ok(SearchedBoxMountainResult {
                            mountains: searched_mountains,
                            total,
                        })
                    }
                    Err(_) => Err(MountainFindException::new_with_error_code(
                        ErrorCode::ServerError,
                    )),
                }
            }
            Err(error_messages) => Err(MountainFindException::new(
                ErrorCode::InvalidQueryParam,
                error_messages,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::mountain::{MountainBoxSearchQuery, MountainSearchQuery};
    use mockall::mock;
    use mountix_kernel::model::mountain::{
        Mountain, MountainBoxSearchCondition, MountainLocation, MountainSearchCondition,
    };
    use mountix_kernel::model::{ErrorCode, Id};
    use mountix_kernel::repository::mountain::MountainRepository;
    use std::sync::Arc;

    mock! {
        TestMountainRepository {}

        #[async_trait::async_trait]
        impl MountainRepository for TestMountainRepository {
            async fn get(&self, id: Id<Mountain>) -> anyhow::Result<Option<Mountain>>;
            async fn get_count(&self, search_condition: MountainSearchCondition) -> anyhow::Result<u64>;
            async fn find(&self, search_condition: MountainSearchCondition) -> anyhow::Result<Vec<Mountain>>;
            async fn find_box(&self, search_condition: MountainBoxSearchCondition) -> anyhow::Result<Vec<Mountain>>;
        }
    }

    mock! {
        TestSurroundingMountainRepository {}

        #[async_trait::async_trait]
        impl mountix_kernel::repository::surrounding_mountain::SurroundingMountainRepository for TestSurroundingMountainRepository {
            async fn find(&self, condition: mountix_kernel::model::surrounding_mountain::SurroundingMountainSearchCondition) -> anyhow::Result<Vec<mountix_kernel::model::surrounding_mountain::SurroundingMountain>>;
        }
    }

    struct MockRepositoriesModule {
        mountain_repository: MockTestMountainRepository,
        surrounding_mountain_repository: MockTestSurroundingMountainRepository,
    }

    impl RepositoriesModuleExt for MockRepositoriesModule {
        type MountainRepo = MockTestMountainRepository;
        type SurroundingMountainRepo = MockTestSurroundingMountainRepository;

        fn mountain_repository(&self) -> &Self::MountainRepo {
            &self.mountain_repository
        }

        fn surrounding_mountain_repository(&self) -> &Self::SurroundingMountainRepo {
            &self.surrounding_mountain_repository
        }
    }

    fn create_test_mountain() -> Mountain {
        let id = Id::new(1);
        let location =
            MountainLocation::new(35.360556, 138.727778, "https://maps.gsi.go.jp".to_string());
        let data = mountix_kernel::model::mountain::MountainData {
            name: "富士山".to_string(),
            name_kana: "ふじさん".to_string(),
            area: "関東地方".to_string(),
            prefectures: vec!["静岡県".to_string(), "山梨県".to_string()],
            elevation: 3776,
            location,
            tags: vec!["百名山".to_string()],
        };
        Mountain::new(id, data)
    }

    #[tokio::test]
    async fn test_mountain_use_case_get_success() {
        let mut mock_repo = MockTestMountainRepository::new();
        mock_repo
            .expect_get()
            .with(mockall::predicate::function(|id: &Id<Mountain>| {
                id.value == 1
            }))
            .times(1)
            .returning(|_| Ok(Some(create_test_mountain())));

        let mock_module = MockRepositoriesModule {
            mountain_repository: mock_repo,
            surrounding_mountain_repository: MockTestSurroundingMountainRepository::new(),
        };

        let use_case = MountainUseCase::new(Arc::new(mock_module));
        let result = use_case.get("1".to_string()).await;

        assert!(result.is_ok());
        let mountain = result.unwrap();
        assert!(mountain.is_some());
        let mountain = mountain.unwrap();
        assert_eq!(mountain.id, 1);
        assert_eq!(mountain.name, "富士山");
    }

    #[tokio::test]
    async fn test_mountain_use_case_get_not_found() {
        let mut mock_repo = MockTestMountainRepository::new();
        mock_repo
            .expect_get()
            .with(mockall::predicate::function(|id: &Id<Mountain>| {
                id.value == 999
            }))
            .times(1)
            .returning(|_| Ok(None));

        let mock_module = MockRepositoriesModule {
            mountain_repository: mock_repo,
            surrounding_mountain_repository: MockTestSurroundingMountainRepository::new(),
        };

        let use_case = MountainUseCase::new(Arc::new(mock_module));
        let result = use_case.get("999".to_string()).await;

        assert!(result.is_ok());
        let mountain = result.unwrap();
        assert!(mountain.is_none());
    }

    #[tokio::test]
    async fn test_mountain_use_case_get_invalid_id() {
        let mock_repo = MockTestMountainRepository::new();
        let mock_module = MockRepositoriesModule {
            mountain_repository: mock_repo,
            surrounding_mountain_repository: MockTestSurroundingMountainRepository::new(),
        };

        let use_case = MountainUseCase::new(Arc::new(mock_module));
        let result = use_case.get("invalid".to_string()).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.error_code, ErrorCode::InvalidId);
    }

    #[tokio::test]
    async fn test_mountain_use_case_get_repository_error() {
        let mut mock_repo = MockTestMountainRepository::new();
        mock_repo
            .expect_get()
            .with(mockall::predicate::function(|id: &Id<Mountain>| {
                id.value == 1
            }))
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let mock_module = MockRepositoriesModule {
            mountain_repository: mock_repo,
            surrounding_mountain_repository: MockTestSurroundingMountainRepository::new(),
        };

        let use_case = MountainUseCase::new(Arc::new(mock_module));
        let result = use_case.get("1".to_string()).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.error_code, ErrorCode::ServerError);
    }

    #[tokio::test]
    async fn test_mountain_use_case_find_success() {
        let mut mock_repo = MockTestMountainRepository::new();
        mock_repo.expect_get_count().times(1).returning(|_| Ok(1));
        mock_repo
            .expect_find()
            .times(1)
            .returning(|_| Ok(vec![create_test_mountain()]));

        let mock_module = MockRepositoriesModule {
            mountain_repository: mock_repo,
            surrounding_mountain_repository: MockTestSurroundingMountainRepository::new(),
        };

        let use_case = MountainUseCase::new(Arc::new(mock_module));
        let search_query = MountainSearchQuery {
            name: None,
            prefecture: None,
            tag: None,
            offset: None,
            limit: None,
            sort: None,
        };
        let result = use_case.find(search_query).await;

        assert!(result.is_ok());
        let search_result = result.unwrap();
        assert_eq!(search_result.mountains.len(), 1);
        assert_eq!(search_result.total, 1);
        assert_eq!(search_result.mountains[0].name, "富士山");
    }

    #[tokio::test]
    async fn test_mountain_use_case_find_with_invalid_params() {
        let mock_repo = MockTestMountainRepository::new();
        let mock_module = MockRepositoriesModule {
            mountain_repository: mock_repo,
            surrounding_mountain_repository: MockTestSurroundingMountainRepository::new(),
        };

        let use_case = MountainUseCase::new(Arc::new(mock_module));
        let search_query = MountainSearchQuery {
            name: None,
            prefecture: Some("invalid".to_string()),
            tag: None,
            offset: None,
            limit: None,
            sort: None,
        };
        let result = use_case.find(search_query).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.error_code, ErrorCode::InvalidQueryParam);
    }

    #[tokio::test]
    async fn test_mountain_use_case_find_repository_error() {
        let mut mock_repo = MockTestMountainRepository::new();
        mock_repo.expect_get_count().times(1).returning(|_| Ok(1));
        mock_repo
            .expect_find()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let mock_module = MockRepositoriesModule {
            mountain_repository: mock_repo,
            surrounding_mountain_repository: MockTestSurroundingMountainRepository::new(),
        };

        let use_case = MountainUseCase::new(Arc::new(mock_module));
        let search_query = MountainSearchQuery {
            name: None,
            prefecture: None,
            tag: None,
            offset: None,
            limit: None,
            sort: None,
        };
        let result = use_case.find(search_query).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.error_code, ErrorCode::ServerError);
    }

    #[tokio::test]
    async fn test_mountain_use_case_find_box_success() {
        let mut mock_repo = MockTestMountainRepository::new();
        mock_repo
            .expect_find_box()
            .times(1)
            .returning(|_| Ok(vec![create_test_mountain()]));

        let mock_module = MockRepositoriesModule {
            mountain_repository: mock_repo,
            surrounding_mountain_repository: MockTestSurroundingMountainRepository::new(),
        };

        let use_case = MountainUseCase::new(Arc::new(mock_module));
        let search_query = MountainBoxSearchQuery {
            box_coordinates: "(139.0,35.0),(140.0,36.0)".to_string(),
            name: None,
            tag: None,
            sort: None,
        };
        let result = use_case.find_box(search_query).await;

        assert!(result.is_ok());
        let search_result = result.unwrap();
        assert_eq!(search_result.mountains.len(), 1);
        assert_eq!(search_result.total, 1);
        assert_eq!(search_result.mountains[0].name, "富士山");
    }

    #[tokio::test]
    async fn test_mountain_use_case_find_box_with_invalid_coordinates() {
        let mock_repo = MockTestMountainRepository::new();
        let mock_module = MockRepositoriesModule {
            mountain_repository: mock_repo,
            surrounding_mountain_repository: MockTestSurroundingMountainRepository::new(),
        };

        let use_case = MountainUseCase::new(Arc::new(mock_module));
        let search_query = MountainBoxSearchQuery {
            box_coordinates: "invalid_format".to_string(),
            name: None,
            tag: None,
            sort: None,
        };
        let result = use_case.find_box(search_query).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.error_code, ErrorCode::InvalidQueryParam);
    }

    #[tokio::test]
    async fn test_mountain_use_case_find_box_repository_error() {
        let mut mock_repo = MockTestMountainRepository::new();
        mock_repo
            .expect_find_box()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let mock_module = MockRepositoriesModule {
            mountain_repository: mock_repo,
            surrounding_mountain_repository: MockTestSurroundingMountainRepository::new(),
        };

        let use_case = MountainUseCase::new(Arc::new(mock_module));
        let search_query = MountainBoxSearchQuery {
            box_coordinates: "(139.0,35.0),(140.0,36.0)".to_string(),
            name: None,
            tag: None,
            sort: None,
        };
        let result = use_case.find_box(search_query).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.error_code, ErrorCode::ServerError);
    }
}
