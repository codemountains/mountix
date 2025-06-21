use crate::model::surrounding_mountain::{
    SearchedSurroundingMountain, SearchedSurroundingMountainResult, SurroundingMountainSearchQuery,
};
use mountix_adapter::modules::RepositoriesModuleExt;
use mountix_kernel::model::surrounding_mountain::{
    SurroundingMountainFindException, SurroundingMountainSearchCondition,
    SurroundingMountainSearchDistance,
};
use mountix_kernel::model::ErrorCode;
use mountix_kernel::repository::mountain::MountainRepository;
use mountix_kernel::repository::surrounding_mountain::SurroundingMountainRepository;
use std::sync::Arc;

pub struct SurroundingMountainUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> SurroundingMountainUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn find(
        &self,
        id: String,
        search_query: SurroundingMountainSearchQuery,
    ) -> Result<SearchedSurroundingMountainResult, SurroundingMountainFindException> {
        match id.try_into() {
            Ok(id) => match self.repositories.mountain_repository().get(id).await {
                Ok(mountain) => match mountain {
                    Some(mountain) => {
                        match SurroundingMountainSearchDistance::try_from(search_query) {
                            Ok(search_distance) => {
                                let distance = search_distance.0;

                                let condition = SurroundingMountainSearchCondition::new(
                                    mountain,
                                    search_distance,
                                );

                                match self
                                    .repositories
                                    .surrounding_mountain_repository()
                                    .find(condition)
                                    .await
                                {
                                    Ok(mountains) => {
                                        let searched_mountains: Vec<SearchedSurroundingMountain> =
                                            mountains.into_iter().map(|m| m.into()).collect();

                                        Ok(SearchedSurroundingMountainResult {
                                            mountains: searched_mountains,
                                            distance,
                                        })
                                    }
                                    Err(_) => {
                                        Err(SurroundingMountainFindException::new_with_error_code(
                                            ErrorCode::ServerError,
                                        ))
                                    }
                                }
                            }
                            Err(error_messages) => Err(SurroundingMountainFindException::new(
                                ErrorCode::InvalidQueryParam,
                                error_messages,
                            )),
                        }
                    }
                    None => Err(SurroundingMountainFindException::new_with_error_code(
                        ErrorCode::ServerError,
                    )),
                },
                Err(_) => Err(SurroundingMountainFindException::new_with_error_code(
                    ErrorCode::ServerError,
                )),
            },
            Err(_) => Err(SurroundingMountainFindException::new_with_error_code(
                ErrorCode::ServerError,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::surrounding_mountain::SurroundingMountainSearchQuery;
    use mockall::mock;
    use mountix_kernel::model::mountain::{Mountain, MountainLocation};
    use mountix_kernel::model::surrounding_mountain::{
        SurroundingMountain, SurroundingMountainLocation, SurroundingMountainSearchCondition,
    };
    use mountix_kernel::model::{ErrorCode, Id};
    use mountix_kernel::repository::mountain::MountainRepository;
    use mountix_kernel::repository::surrounding_mountain::SurroundingMountainRepository;
    use std::sync::Arc;

    mock! {
        TestSurroundingMountainRepository {}

        #[async_trait::async_trait]
        impl SurroundingMountainRepository for TestSurroundingMountainRepository {
            async fn find(&self, condition: SurroundingMountainSearchCondition) -> anyhow::Result<Vec<SurroundingMountain>>;
        }
    }

    mock! {
        TestMountainRepository {}

        #[async_trait::async_trait]
        impl MountainRepository for TestMountainRepository {
            async fn get(&self, id: Id<Mountain>) -> anyhow::Result<Option<Mountain>>;
            async fn get_count(&self, search_condition: mountix_kernel::model::mountain::MountainSearchCondition) -> anyhow::Result<u64>;
            async fn find(&self, search_condition: mountix_kernel::model::mountain::MountainSearchCondition) -> anyhow::Result<Vec<Mountain>>;
            async fn find_box(&self, search_condition: mountix_kernel::model::mountain::MountainBoxSearchCondition) -> anyhow::Result<Vec<Mountain>>;
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

    fn create_test_surrounding_mountain() -> SurroundingMountain {
        let id = Id::new(2);
        let location = SurroundingMountainLocation::new(
            35.300000,
            138.800000,
            "https://maps.gsi.go.jp/surrounding".to_string(),
        );
        let data = mountix_kernel::model::surrounding_mountain::SurroundingMountainData {
            name: "周辺の山".to_string(),
            name_kana: "しゅうへんのやま".to_string(),
            area: "関東地方".to_string(),
            prefectures: vec!["静岡県".to_string()],
            elevation: 2500,
            location,
            tags: vec!["二百名山".to_string()],
        };
        SurroundingMountain::new(id, data)
    }

    #[tokio::test]
    async fn test_surrounding_mountain_use_case_find_success() {
        let mut mock_mountain_repo = MockTestMountainRepository::new();
        mock_mountain_repo
            .expect_get()
            .with(mockall::predicate::function(|id: &Id<Mountain>| {
                id.value == 1
            }))
            .times(1)
            .returning(|_| Ok(Some(create_test_mountain())));

        let mut mock_surrounding_repo = MockTestSurroundingMountainRepository::new();
        mock_surrounding_repo
            .expect_find()
            .times(1)
            .returning(|_| Ok(vec![create_test_surrounding_mountain()]));

        let mock_module = MockRepositoriesModule {
            mountain_repository: mock_mountain_repo,
            surrounding_mountain_repository: mock_surrounding_repo,
        };

        let use_case = SurroundingMountainUseCase::new(Arc::new(mock_module));
        let search_query = SurroundingMountainSearchQuery {
            distance: Some("10000".to_string()),
        };
        let result = use_case.find("1".to_string(), search_query).await;

        assert!(result.is_ok());
        let search_result = result.unwrap();
        assert_eq!(search_result.mountains.len(), 1);
        assert_eq!(search_result.distance, 10000);
        assert_eq!(search_result.mountains[0].name, "周辺の山");
    }

    #[tokio::test]
    async fn test_surrounding_mountain_use_case_find_mountain_not_found() {
        let mut mock_mountain_repo = MockTestMountainRepository::new();
        mock_mountain_repo
            .expect_get()
            .with(mockall::predicate::function(|id: &Id<Mountain>| {
                id.value == 999
            }))
            .times(1)
            .returning(|_| Ok(None));

        let mock_surrounding_repo = MockTestSurroundingMountainRepository::new();

        let mock_module = MockRepositoriesModule {
            mountain_repository: mock_mountain_repo,
            surrounding_mountain_repository: mock_surrounding_repo,
        };

        let use_case = SurroundingMountainUseCase::new(Arc::new(mock_module));
        let search_query = SurroundingMountainSearchQuery {
            distance: Some("5000".to_string()),
        };
        let result = use_case.find("999".to_string(), search_query).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.error_code, ErrorCode::ServerError);
    }

    #[tokio::test]
    async fn test_surrounding_mountain_use_case_find_invalid_id() {
        let mock_mountain_repo = MockTestMountainRepository::new();
        let mock_surrounding_repo = MockTestSurroundingMountainRepository::new();

        let mock_module = MockRepositoriesModule {
            mountain_repository: mock_mountain_repo,
            surrounding_mountain_repository: mock_surrounding_repo,
        };

        let use_case = SurroundingMountainUseCase::new(Arc::new(mock_module));
        let search_query = SurroundingMountainSearchQuery {
            distance: Some("5000".to_string()),
        };
        let result = use_case.find("invalid".to_string(), search_query).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.error_code, ErrorCode::ServerError);
    }

    #[tokio::test]
    async fn test_surrounding_mountain_use_case_find_repository_error() {
        let mut mock_mountain_repo = MockTestMountainRepository::new();
        mock_mountain_repo
            .expect_get()
            .with(mockall::predicate::function(|id: &Id<Mountain>| {
                id.value == 1
            }))
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let mock_surrounding_repo = MockTestSurroundingMountainRepository::new();

        let mock_module = MockRepositoriesModule {
            mountain_repository: mock_mountain_repo,
            surrounding_mountain_repository: mock_surrounding_repo,
        };

        let use_case = SurroundingMountainUseCase::new(Arc::new(mock_module));
        let search_query = SurroundingMountainSearchQuery {
            distance: Some("5000".to_string()),
        };
        let result = use_case.find("1".to_string(), search_query).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.error_code, ErrorCode::ServerError);
    }

    #[tokio::test]
    async fn test_surrounding_mountain_use_case_find_surrounding_repository_error() {
        let mut mock_mountain_repo = MockTestMountainRepository::new();
        mock_mountain_repo
            .expect_get()
            .with(mockall::predicate::function(|id: &Id<Mountain>| {
                id.value == 1
            }))
            .times(1)
            .returning(|_| Ok(Some(create_test_mountain())));

        let mut mock_surrounding_repo = MockTestSurroundingMountainRepository::new();
        mock_surrounding_repo
            .expect_find()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Surrounding database error")));

        let mock_module = MockRepositoriesModule {
            mountain_repository: mock_mountain_repo,
            surrounding_mountain_repository: mock_surrounding_repo,
        };

        let use_case = SurroundingMountainUseCase::new(Arc::new(mock_module));
        let search_query = SurroundingMountainSearchQuery {
            distance: Some("5000".to_string()),
        };
        let result = use_case.find("1".to_string(), search_query).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.error_code, ErrorCode::ServerError);
    }

    #[tokio::test]
    async fn test_surrounding_mountain_use_case_find_with_default_distance() {
        let mut mock_mountain_repo = MockTestMountainRepository::new();
        mock_mountain_repo
            .expect_get()
            .with(mockall::predicate::function(|id: &Id<Mountain>| {
                id.value == 1
            }))
            .times(1)
            .returning(|_| Ok(Some(create_test_mountain())));

        let mut mock_surrounding_repo = MockTestSurroundingMountainRepository::new();
        mock_surrounding_repo
            .expect_find()
            .times(1)
            .returning(|_| Ok(vec![create_test_surrounding_mountain()]));

        let mock_module = MockRepositoriesModule {
            mountain_repository: mock_mountain_repo,
            surrounding_mountain_repository: mock_surrounding_repo,
        };

        let use_case = SurroundingMountainUseCase::new(Arc::new(mock_module));
        let search_query = SurroundingMountainSearchQuery {
            distance: None, // Use default distance
        };
        let result = use_case.find("1".to_string(), search_query).await;

        assert!(result.is_ok());
        let search_result = result.unwrap();
        assert_eq!(search_result.mountains.len(), 1);
        assert_eq!(search_result.distance, 5000); // Default distance
    }

    #[tokio::test]
    async fn test_surrounding_mountain_use_case_find_empty_results() {
        let mut mock_mountain_repo = MockTestMountainRepository::new();
        mock_mountain_repo
            .expect_get()
            .with(mockall::predicate::function(|id: &Id<Mountain>| {
                id.value == 1
            }))
            .times(1)
            .returning(|_| Ok(Some(create_test_mountain())));

        let mut mock_surrounding_repo = MockTestSurroundingMountainRepository::new();
        mock_surrounding_repo
            .expect_find()
            .times(1)
            .returning(|_| Ok(vec![])); // No surrounding mountains found

        let mock_module = MockRepositoriesModule {
            mountain_repository: mock_mountain_repo,
            surrounding_mountain_repository: mock_surrounding_repo,
        };

        let use_case = SurroundingMountainUseCase::new(Arc::new(mock_module));
        let search_query = SurroundingMountainSearchQuery {
            distance: Some("1000".to_string()),
        };
        let result = use_case.find("1".to_string(), search_query).await;

        assert!(result.is_ok());
        let search_result = result.unwrap();
        assert_eq!(search_result.mountains.len(), 0);
        assert_eq!(search_result.distance, 1000);
    }
}
