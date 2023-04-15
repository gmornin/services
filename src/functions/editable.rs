use std::{path::Path, ffi::OsStr};

pub fn editable(path: &Path) -> bool {
    if is_bson(path) {
        return false;
    }

    !matches!(path.iter().skip(2).map(|section| section.to_str().unwrap()).collect::<Vec<_>>().as_slice(), [] | [_] | ["system", ..])
}

pub fn is_bson(path: &Path) -> bool {
    path.extension() == Some(OsStr::new("bson"))
}
