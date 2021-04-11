use async_trait::async_trait;
use tokio_postgres::Error;
use super::repository::{Repository, Db};

pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub quantity_in_stock: u32,
    pub description: String,
    pub category_id: String
}

#[async_trait]
impl Repository for Product {
    async fn find_all() -> Result<Vec<Self>, Error> {
        unimplemented!()
    }

    async fn find_by_id(id: String) -> Result<Option<Self>, Error> {
        unimplemented!()
    }

    async fn save(&self) -> Result<(), Error> {
        unimplemented!()
    }

    async fn find_by_name(name: String) -> Result<Option<Self>, Error> {
        unimplemented!()
    }
}

impl Product {
    pub fn new (id: String, name: String, price: f64, quantity_in_stock: u32, 
        description: String, category_id: String) -> Self {
        Product{ id, name, price, quantity_in_stock, description, category_id }
    }
}