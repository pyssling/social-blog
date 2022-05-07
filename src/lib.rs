tonic::include_proto!("social_blog");


pub mod blog_service;
pub use blog_service::BlogService;

pub mod blog_entry;
pub use blog_entry::BlogEntry;
pub use blog_entry::BlogContentsFormat;
pub use blog_entry::blog_entry_to_json;
