use std::{ffi::OsStr, path::Path};

pub fn editable(path: &Path) -> bool {
    if is_bson(path) {
        return false;
    }

    let path_slice = path
        .iter()
        .skip(5)
        .map(|section| section.to_str().unwrap())
        .collect::<Vec<_>>();
    !(path_slice.len() < 2 || path_slice[1] == ".system")
}

pub fn is_bson(path: &Path) -> bool {
    path.extension() == Some(OsStr::new("bson"))
}

pub fn has_dotdot(path: &Path) -> bool {
    path.iter()
        .any(|section| matches!(section.to_str().unwrap(), "." | ".."))
}
