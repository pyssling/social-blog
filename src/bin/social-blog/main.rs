use tonic::{transport::Server, Request, Response, Status};

use social_blog::blog_server::{Blog, BlogServer};
use social_blog::{CreatePageRequest, CreatePageResponse};
use uuid::Uuid;

#[derive(Default)]
pub struct BlogService {}

#[tonic::async_trait]
impl Blog for BlogService {
    async fn create_page(
        &self,
        request: Request<CreatePageRequest>,
    ) -> Result<Response<CreatePageResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let page_uuid = social_blog::Uuid {
            id: Uuid::new_v4().to_string(),
        };
        let reply = social_blog::CreatePageResponse {
            page_id: Some(page_uuid),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let blog = BlogService::default();

    println!("BlogServer listening on {}", addr);

    Server::builder()
        .add_service(BlogServer::new(blog))
        .serve(addr)
        .await?;

    Ok(())
}
