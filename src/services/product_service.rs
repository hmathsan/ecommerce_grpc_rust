use tonic::{Request, Response, Status, Code};

use product::{CreateProductRequest, GetProductByIdRequest, ProductResponse};
use product::product_service_server::{ProductService, ProductServiceServer};

mod product {
    include!(concat!(env!("OUT_DIR"), "\\product.rs"));
}

#[derive(Debug, Default)]
pub struct ProductEndpoint{}

#[tonic::async_trait]
impl ProductService for ProductEndpoint {
    async fn create_product(&self, request: Request<CreateProductRequest>) -> Result<Response<ProductResponse>, Status> {
        unimplemented!()
    }

    async fn get_product_by_id(&self, request: Request<GetProductByIdRequest>) -> Result<Response<ProductResponse>, Status> {
        unimplemented!()
    }
}

impl ProductEndpoint {
    pub fn rtn_service() -> ProductServiceServer<Self> {
        ProductServiceServer::new(Self::default())
    }
}