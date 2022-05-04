use std::ptr::NonNull;

use tonic::{Request, Response, Status};

use crate::blog_server::Blog;
use crate::{CreateBlogEntryRequest, CreateBlogEntryResponse, ContentsFormat as ProtoContentsFormat};
use prost_types::Timestamp;
use uuid::Uuid;
use chrono::{NaiveDateTime, Utc, Date, NaiveDate};

#[derive(Default)]
pub struct BlogService {}

#[tonic::async_trait]
impl Blog for BlogService {
    async fn create_blog_entry(
        &self,
        request: Request<CreateBlogEntryRequest>,
    ) -> Result<Response<CreateBlogEntryResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let proto = request.into_inner();

        let timestamp = match proto.timestamp {
            Some(timestamp) => NaiveDateTime::from_timestamp(timestamp.seconds, timestamp.nanos as u32),
            None => return Err(Status::invalid_argument("Missing timestamp."))
        };

        let contents_format = match ProtoContentsFormat::from_i32(proto.contents_format) {
            Some(ProtoContentsFormat::Markdown) => crate::BlogContentsFormat::Markdown,
            _ => return Err(Status::invalid_argument(format!("Unknown contents format: {}", proto.contents_format)))
        };

        let blogEntry = crate::BlogEntry::new( proto.title, proto.contents,  contents_format, timestamp );

        crate::write_blog_entry(blogEntry);

        let page_uuid = crate::Uuid {
            id: Uuid::new_v4().to_string(),
        };
        let reply = crate::CreateBlogEntryResponse {
            page_id: Some(page_uuid),
        };
        Ok(Response::new(reply))
    }
}
