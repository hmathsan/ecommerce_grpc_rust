use tonic::{Request, Response, Status};
use category::{CategoryResponse, FindCategoryByIdRequest, CreateCategoryRequest, CreateCategoryResponse};
use category::category_service_server::{CategoryService, CategoryServiceServer};

mod category {
    include!(concat!(env!("OUT_DIR"), "\\category.rs"));
}

#[derive(Debug, Default)]
pub struct CategoryEndpoint;

#[tonic::async_trait]
impl CategoryService for CategoryEndpoint {
    async fn create_category(&self, request:Request<CreateCategoryRequest>) -> Result<Response<CreateCategoryResponse>, Status> {
        unimplemented!()
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