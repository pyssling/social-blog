use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("social_blog_descriptor.bin"))
        .compile(&["proto/social_blog.proto"], &["proto"])
        .unwrap();
}
