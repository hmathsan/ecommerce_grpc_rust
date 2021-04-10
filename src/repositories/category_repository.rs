use async_trait::async_trait;
use tokio_postgres::Error;
use super::repository::Repository;
use super::repository::Db;

pub struct Category {
    id: String,
    name: String,
    parent_category_id: String
}

#[async_trait]
impl Repository for Category {
    async fn find_all() -> Result<Self, Error> {
        Db::db_connect().await?;
        unimplemented!()
    }

    async fn find_by_id(id: &str) -> Result<Self, Error> {
        Db::db_connect().await?;
        unimplemented!()
    }
}