use actix_files::NamedFile;
use actix_web::{get, web::Path};

use crate::SERVICES_STATIC;

#[get("/{path:.*}")]
async fn r#static(path: Path<String>) -> actix_web::Result<NamedFile> {
    println!("{path}");
    Ok(NamedFile::open_async(SERVICES_STATIC.get().unwrap().join(path.into_inner())).await?)
}
