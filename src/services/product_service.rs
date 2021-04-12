use tonic::{Request, Response, Status, Code};
use uuid::Uuid;

use product::{CreateProductRequest, GetProductByIdRequest, ProductResponse, CategoryResponse};
use product::product_service_server::{ProductService, ProductServiceServer};

use crate::repositories::{product_repository::Product, category_repository::Category, repository::Repository};

mod product {
    include!(concat!(env!("OUT_DIR"), "\\product.rs"));
}

#[derive(Debug, Default)]
pub struct ProductEndpoint{}

#[tonic::async_trait]
impl ProductService for ProductEndpoint {
    async fn create_product(&self, request: Request<CreateProductRequest>) -> Result<Response<ProductResponse>, Status> {
        println!("Receiving request for new Product.");
        let new_id = Uuid::new_v4();
        let product_request: CreateProductRequest = request.into_inner();
        let cat_name = String::from(&product_request.category);
        let category: Category;

        println!("Verifying if product with name {} already exists.", String::from(&product_request.name));
        match Product::find_by_name(String::from(&product_request.name)).await {
            Ok(possible_prod) => {
                if let Some(_) = possible_prod {
                    println!("Product with name {} already exists.\r\n", String::from(&product_request.name));
                    return Err(Status::new(Code::AlreadyExists, 
                        format!("Product with name {} already exists.", String::from(&product_request.name))))
                }
            },
            Err(err) => {
                println!("Unexpected error ocurred while searching for Product by name: {:?}\r\n", err);
                return Err(Status::new(Code::Internal, 
                    format!("Unexpected error ocurred while searching for Product by name: {}", err)))
            }
        }

        println!("Trying to find Category from request.");
        match Category::find_by_name(String::from(&cat_name)).await {
            Ok(possible_cat) => {
                match possible_cat {
                    Some(cat) => {
                        println!("Category with name {} found.", String::from(&cat_name));
                        category = cat;
                    },
                    None => {
                        println!("No Category with name {} found.\r\n", String::from(&cat_name));
                        return Err(Status::new(Code::NotFound, format!("No Category with name {} found.", cat_name)))
                    }
                }
            }, 
            Err(err) => {
                println!("Unexpected error while searching for category name: {:?}\r\n", err);
                return Err(Status::new(Code::Internal, format!("Unexpected error while searching for category name: {}", err)))
            }
        }

        let product_name: &String = &product_request.name;
        let product_price: &f64 = &product_request.price;
        let product_quantity: &i32 = &product_request.quantity_in_stock;
        let product_desc: &String = &product_request.description;

        let new_product = Product::new(new_id.to_string(), product_name.to_owned(), 
           product_price.to_owned(), product_quantity.to_owned() as u32, 
           product_desc.to_owned(), String::from(&category.id));

        if let Err(err) = new_product.save().await {
            println!("An error ocurred while saving product: {:?}\r\n", err);
            return Err(Status::new(Code::Internal, format!("An error ocurred while saving product: {:?}", err)))
        };

        println!("Building response struct.");
        let response = ProductResponse{
            id: new_product.id,
            name: new_product.name,
            price: new_product.price,
            quantity_in_stock: new_product.quantity_in_stock as i32,
            description: new_product.description,
            category: Some(CategoryResponse {
                id: category.id,
                name: category.name
            })
        };

        println!("Returning response. Request completed successfully.\r\n");
        Ok(Response::new(response))
    }

    async fn get_product_by_id(&self, request: Request<GetProductByIdRequest>) -> Result<Response<ProductResponse>, Status> {
        println!("Receiving request to find Product by id.");
        let request_id = request.into_inner().id;

        println!("Trying to find Product.");
        match Product::find_by_id(String::from(&request_id)).await {
            Ok(possible_product) => {
                match possible_product {
                    Some(product) => {
                        println!("Product with id {} found.", &request_id);
                        let category: Category;

                        println!("Getting Category information.");
                        match Category::find_by_id(String::from(&product.category_id)).await {
                            Ok(possible_cat) => {
                                match possible_cat {
                                    Some(cat) => {
                                        println!("Category with id {} found.", String::from(String::from(&product.category_id)));
                                        category = cat;
                                    },
                                    None => {
                                        println!("No Category with id {} found.\r\n", String::from(String::from(&product.category_id)));
                                        return Err(Status::new(Code::NotFound, 
                                            format!("No Category with name {} found.", String::from(&product.category_id))))
                                    }
                                }
                            }, 
                            Err(err) => {
                                println!("Unexpected error while searching for category id: {:?}\r\n", err);
                                return Err(Status::new(Code::Internal, 
                                    format!("Unexpected error while searching for category name: {}", err)))
                            }
                        }

                        println!("Building response struct.");
                        let response = ProductResponse {
                            id: product.id,
                            name: product.name,
                            price: product.price,
                            quantity_in_stock: product.quantity_in_stock as i32,
                            description: product.description,
                            category: Some(CategoryResponse{
                                id: category.id,
                                name: category.name
                            })
                        };

                        println!("Returning response. Request completed successfully.\r\n");
                        return Ok(Response::new(response))
                    },
                    None => {
                        println!("No Product with id {} were found.\r\n", &request_id);
                        return Err(Status::new(Code::NotFound, format!("No Product with id {} were found.", &request_id)))
                    }
                }
            },
            Err(err) => {
                println!("Unexpected error ocurred while finding Product by id: {:?}\r\n", err);
                return Err(Status::new(Code::Internal, 
                    format!("Unexpected error ocurred while finding Product by id: {:?}", err)))
            }
        }
    }
}

impl ProductEndpoint {
    pub fn rtn_service() -> ProductServiceServer<Self> {
        ProductServiceServer::new(Self::default())
    }
}