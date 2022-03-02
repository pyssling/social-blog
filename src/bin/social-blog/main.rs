use tonic::transport::Server;

use social_blog::blog_server::BlogServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let blog = social_blog::blog_service::BlogService::default();

    println!("BlogServer listening on {}", addr);

    Server::builder()
        .add_service(BlogServer::new(blog))
        .serve(addr)
        .await?;

    Ok(())
}
