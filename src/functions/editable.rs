use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::structs::GMServices;

pub fn editable(path: &Path, services: &[GMServices]) -> bool {
    if is_bson(path) {
        return false;
    }
    let path_slice = path
        .iter()
        .map(|section| section.to_str().unwrap())
        .collect::<Vec<_>>();
    path_slice.len() >= 2
        && (path_slice[1] != ".system" || path_slice[2] == "trash")
        && match GMServices::from_str(path_slice[0]) {
            Some(service) => services.contains(&service),
            None => false,
        }
}

pub fn is_bson(path: &Path) -> bool {
    path.extension() == Some(OsStr::new("bson"))
}

pub fn has_dotdot(path: &Path) -> bool {
    path.iter()
        .any(|section| matches!(section.to_str().unwrap(), "." | ".."))
}

pub fn parse_path(mut s: String) -> PathBuf {
    if s.starts_with('~') {
        s = s.replacen(
            '~',
            dirs::home_dir().unwrap().as_os_str().to_str().unwrap(),
            1,
        );
    }

    PathBuf::from(s)
}
