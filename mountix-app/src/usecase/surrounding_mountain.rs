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
                                let distance = search_distance.0.clone();

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
                                    Err(_) => Err(SurroundingMountainFindException::new(
                                        ErrorCode::ServerError,
                                        vec!["山岳情報を検索中にエラーが発生しました。".to_string()],
                                    )),
                                }
                            }
                            Err(error_messages) => Err(SurroundingMountainFindException::new(
                                ErrorCode::InvalidQueryParam,
                                error_messages,
                            )),
                        }
                    }
                    None => Err(SurroundingMountainFindException::new(
                        ErrorCode::ServerError,
                        vec!["山岳情報を検索中にエラーが発生しました。".to_string()],
                    )),
                },
                Err(_) => Err(SurroundingMountainFindException::new(
                    ErrorCode::ServerError,
                    vec!["山岳情報を検索中にエラーが発生しました。".to_string()],
                )),
            },
            Err(_) => Err(SurroundingMountainFindException::new(
                ErrorCode::ServerError,
                vec!["山岳情報を検索中にエラーが発生しました。".to_string()],
            )),
        }
    }
}
