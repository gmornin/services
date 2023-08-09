use std::path::Path;
use std::{error::Error, ffi::OsStr};
use tokio::process::Command;

use goodmorning_bindings::services::v1::{V1Error, V1Response};

use tokio::fs;

pub async fn pdflatex_latex2pdf(
    source: &Path,
    taskid: u64,
    user_path: &Path,
    restrict_path: &Path,
) -> Result<V1Response, Box<dyn Error>> {
    if source.extension() != Some(OsStr::new("tex")) {
        return Err(V1Error::ExtensionMismatch.into());
    }

    if !fs::try_exists(source).await? {
        return Err(V1Error::FileNotFound.into());
    }

    let output = Command::new("firejail")
        .arg(format!("--private={}", restrict_path.to_str().unwrap()))
        .arg("--noprofile")
        .arg("pdflatex")
        .arg("-interaction")
        .arg("nonstopmode")
        .arg("-halt-on-error")
        .arg("-file-line-error")
        .arg(format!(
            "-output-directory=./{}",
            user_path
                .parent()
                .unwrap_or(Path::new(""))
                .to_str()
                .unwrap()
                .trim_start_matches('/')
        ))
        .arg(user_path.to_str().unwrap())
        .output()
        .await?;

    println!("{}", String::from_utf8(output.stdout.clone()).unwrap());

    if output.status.code() != Some(0) {
        return Err(V1Error::CompileError {
            content: String::from_utf8(output.stdout)?
                .lines()
                .rev()
                .skip(2)
                .step_by(2)
                .take(2)
                .collect::<Vec<_>>()
                .join("\n"),
        }
        .into());
    }

    Ok(V1Response::Compiled {
        id: taskid,
        newpath: user_path
            .with_extension("pdf")
            .to_str()
            .unwrap()
            .to_string(),
    })
}
