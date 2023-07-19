use goodmorning_bindings::services::v1::{V1Error, V1Response};
use pulldown_cmark::*;
use std::error::Error;
use std::{ffi::OsStr, path::Path};
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub async fn pulldown_cmark_md2html(
    source: &Path,
    taskid: u64,
    user_path: &Path,
) -> Result<V1Response, Box<dyn Error>> {
    if source.extension() != Some(OsStr::new("md")) {
        return Err(V1Error::ExtensionMismatch.into());
    }

    let md = fs::read_to_string(source).await?;
    let mut buf = String::new();
    html::push_html(&mut buf, Parser::new_ext(&md, Options::all()));

    let newfile = source.with_extension("html");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&newfile)
        .await?;
    file.write_all(buf.as_bytes()).await?;
    Ok(V1Response::Compiled {
        id: taskid,
        newpath: user_path
            .with_extension("html")
            .to_str()
            .unwrap()
            .to_string(),
    })
}
