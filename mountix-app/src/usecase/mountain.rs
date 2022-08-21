use crate::model::mountain::{FoundMountains, MountainSearchQuery, SearchedMountain};
use mountix_adapter::modules::RepositoriesModuleExt;
use mountix_kernel::model::mountain::{
    MountainFindException, MountainGetException, MountainSearchCondition,
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
    ) -> Result<FoundMountains, MountainFindException> {
        match MountainSearchCondition::try_from(search_query) {
            Ok(condition) => {
                let offset = condition.skip.clone();
                let condition_limit = condition.limit.clone();

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
                        let mut searched_mountains: Vec<SearchedMountain> = Vec::new();
                        for m in mountains {
                            searched_mountains.push(m.into());
                        }

                        let mut limit: Option<u64> = None;
                        if let Some(limit_value) = condition_limit {
                            if let Some(parsed_limit) = u64::from_i64(limit_value) {
                                limit = Some(parsed_limit);
                            }
                        }

                        Ok(FoundMountains {
                            mountains: searched_mountains,
                            total,
                            offset,
                            limit,
                        })
                    }
                    Err(_) => Err(MountainFindException::new(
                        ErrorCode::ServerError,
                        vec!["山岳情報を検索中にエラーが発生しました。".to_string()],
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
