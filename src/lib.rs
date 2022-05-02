tonic::include_proto!("social_blog");


pub mod blog_service;
pub use blog_service::BlogService;

pub mod blog;
pub use blog::BlogEntry;
pub use blog::BlogContentsFormat;
