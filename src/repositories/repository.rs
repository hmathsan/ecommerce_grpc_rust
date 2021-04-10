use async_trait::async_trait;
use tokio_postgres::{NoTls, Error, Client};

pub struct Db {}

#[async_trait]
pub trait Repository {
    async fn find_all() -> Result<Self, Error> where Self: Sized;

    async fn find_by_id(id: &str) -> Result<Self, Error> where Self: Sized;
}

impl Db {
    pub async fn db_connect() -> Result<Client, Error> {
        let (client, connection) =
            tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprint!("connection error: {}", e);
            }
        });

        Ok(client)
    }
}