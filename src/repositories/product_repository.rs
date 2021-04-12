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
        println!("Connecting to database to save Product.");
        let db = Db::db_connect().await?;

        println!("Trying to save Product to database.");
        let quantity = &self.quantity_in_stock;
        db.query("INSERT INTO ecommerce.product (id, name, price, quantity_in_stock, description, category_id)
            VALUES ($1, $2, $3, $4, $5, $6);", &[
                &self.id, &self.name, &self.price, &(quantity.to_owned() as i32), &self.description, &self.category_id
            ]).await?;

        println!("Product saved successfully.");
        Ok(())
    }

    async fn find_by_name(name: String) -> Result<Option<Self>, Error> {
        let db = Db::db_connect().await?;

        let product = db.query("SELECT * FROM ecommerce.product WHERE name = $1", &[&name]).await?;
        if &product.len() <= &0 {
            return Ok(None)
        }

        let id: String = product[0].get(0);
        let name: String = product[0].get(1);
        let price: f64 = product[0].get(2);
        let quantity: i32 = product[0].get(3);
        let description:String = product[0].get(4);
        let category_id: String = product[0].get(5);

        Ok(Some(Product{ id, name, price, quantity_in_stock: quantity as u32, description, category_id }))
    }
}

impl Product {
    pub fn new (id: String, name: String, price: f64, quantity_in_stock: u32, 
        description: String, category_id: String) -> Self {
        Product{ id, name, price, quantity_in_stock, description, category_id }
    }
}