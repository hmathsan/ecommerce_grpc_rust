use tonic::{Request, Response, Status, Code};
use category::{CategoryResponse, FindCategoryByIdRequest, CreateCategoryRequest, CreateCategoryResponse};
use category::category_service_server::{CategoryService, CategoryServiceServer};

use crate::repositories::{category_repository::Category, repository::Repository};


use uuid::Uuid;

mod category {
    include!(concat!(env!("OUT_DIR"), "\\category.rs"));
}

#[derive(Debug, Default)]
pub struct CategoryEndpoint;

#[tonic::async_trait]
impl CategoryService for CategoryEndpoint {
    async fn create_category(&self, request:Request<CreateCategoryRequest>) -> Result<Response<CreateCategoryResponse>, Status> {
        println!("Received request for new Category.");
        let id = Uuid::new_v4();
        let new_category = Category::new(id.to_string(), String::from(&request.into_inner().name));

        println!("Verifying if a Category of the same name exists.");
        if let Ok(result) = Category::find_by_name(String::from(&new_category.name)).await {
            if let Some(_) = result {
                println!("A Category named {} already exists.\r\n", &new_category.name);
                return Err(Status::new(Code::AlreadyExists, format!("Category of name {} already exists.", &new_category.name)))
            }
        };

        println!("Saving new Category to database.");
        if let Err(e) = new_category.save().await.or(Err(Status::new(Code::Internal, "Error savind to database."))) {
            println!("An error ocurred while trying to save to database: {:?}", e);
            return Err(Status::new(Code::Internal, format!("A error ocurred while saving to database: {:?}", e)))
        }

        println!("Building response Struct.");
        let created_category = Some(CategoryResponse{ 
            id: String::from(&new_category.id),
            name: String::from(&new_category.name),
        });

        println!("Returning response. Request completed successfully.\r\n");
        Ok(Response::new(CreateCategoryResponse{created_category}))
    }

    async fn find_category_by_id(&self, request:Request<FindCategoryByIdRequest>) -> Result<Response<CategoryResponse>, Status> {
        unimplemented!()
    }
}

impl CategoryEndpoint {
    pub fn rtn_service() -> CategoryServiceServer<Self> {
        CategoryServiceServer::new(Self::default())
    }
}