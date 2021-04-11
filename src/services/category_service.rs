use tonic::{Request, Response, Status, Code};
use category::{CategoryResponse, FindCategoryByIdRequest, CreateCategoryRequest, CreateCategoryResponse, 
    FindAllCategoriesRequest, FindAllCategoriesResponse};
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
        if let Err(e) = new_category.save().await {
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
        println!("Request received to find Category by id");
        let request_id = request.into_inner().id;
        match Category::find_by_id(String::from(&request_id)).await {
            Ok(cat) => {
                match cat {
                    Some(c) => {
                        let id = c.id;
                        let name = c.name;
                        println!("Category of id {} found. Returning response.\r\nRequest completed successfully.\r\n", &request_id);
                        return Ok(Response::new(CategoryResponse{ id, name }));
                    },
                    None => {
                        println!("No Category of id {} were found.\r\n", &request_id);
                        return Err(Status::new(Code::NotFound, format!("No Category of id {} were found.", &request_id)))
                    }
                }
            },
            Err(err) => {
                println!("An unexpected error ocurred while finding category by id.\r\n");
                return Err(Status::new(Code::Internal, format!("An unexpected error ocurred while finding Category by id: {}", err)))
            }
        }
    }

    async fn find_all_categories(&self, _request:Request<FindAllCategoriesRequest>) -> Result<Response<FindAllCategoriesResponse>, Status> {
        println!("Request received to find all Categories.");
        let mut categories = Vec::new();
        
        match Category::find_all().await {
            Ok(cats) => {
                println!("Categories found. Building response array.");
                for cat in cats {
                    categories.push(CategoryResponse{ id: cat.id, name: cat.name });
                }
            },
            Err(err) => {
                println!("An error ocurred while searching for all Categories: {:?}", err);
                return Err(Status::new(Code::Internal, format!("Error while getting all categories: {}", err)))
            }
        };

        println!("Returning response. Request completed successfully.\r\n");
        Ok(Response::new(FindAllCategoriesResponse{ categories }))
    }
}

impl CategoryEndpoint {
    pub fn rtn_service() -> CategoryServiceServer<Self> {
        CategoryServiceServer::new(Self::default())
    }
}