use async_trait::async_trait;
use tokio_postgres::Error;
use super::repository::{Repository, Db};

pub struct Category {
    pub id: String,
    pub name: String,
}

#[async_trait]
impl Repository for Category {
    async fn find_all() -> Result<Vec<Self>, Error> {
        println!("Connecting to database to find all Categories.");
        let db = Db::db_connect().await?;

        let found_categories = db.query("SELECT * FROM ecommerce.category", &[]).await?;

        let mut category_array = Vec::new();

        for cat in found_categories {
            let id = cat.get(0);
            let name = cat.get(1);
            category_array.push(Category{ id, name });
        }

        println!("A total of {} Categories were found.", &category_array.len());
        Ok(category_array)
    }

    async fn find_by_id(id: String) -> Result<Option<Self>, Error> {
        println!("Connecting to database to find Category by id.");
        let db = Db::db_connect().await?;

        let found_category = db.query("SELECT * FROM ecommerce.category WHERE ID = $1", &[&id]).await?;

        if &found_category.len() <= &0 {
            return Ok(None)
        }

        let cat_id: String = found_category[0].get(0);
        let cat_name: String = found_category[0].get(1);

        Ok(Some(Category{ id: cat_id, name: cat_name }))
    }
    
    async fn find_by_name(name: String) -> Result<Option<Self>, Error> {
        let db = Db::db_connect().await?;

        let category = db.query("SELECT * FROM ecommerce.category WHERE NAME = $1", &[&name]).await?;
        if &category.len() <= &0 {
            return Ok(None)
        };

        let id: String = category[0].get(0);
        let cat_name: String = category[0].get(1);

        Ok(Some(Category { id, name: cat_name }))
    }

    async fn save(&self) -> Result<(), Error> {
        println!("Connecting to database.");
        let db = Db::db_connect().await?;

        println!("Trying to save to database.");
        db.query("INSERT INTO ecommerce.category (id, name) VALUES ($1, $2)", &[&self.id, &self.name]).await?;

        println!("Category saved successfully.");
        Ok(())
    }
}

impl Category {
    pub fn new(id: String, name: String) -> Self {
        let converted_name = String::from(name);
        Category { id, name: converted_name }
    }
}