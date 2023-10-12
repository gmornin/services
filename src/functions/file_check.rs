use std::{ffi::OsStr, path::Path};

use bindings::services::v1::V1Error;

use crate::{structs::*, *};

use super::mime_collapse;

pub fn file_check(bytes: &[u8], path: &Path) -> Result<(), (String, String)> {
    let check_type = FILE_CHECK.get().unwrap();

    if check_type == &FileCheckType::None || bytes.is_empty() {
        return Ok(());
    }
    let expected = MIME_DB
        .get()
        .unwrap()
        .get_mime_types_from_file_name(path.file_name().unwrap().to_str().unwrap());

    if expected.is_empty()
        || (check_type == &FileCheckType::Whitelist
            && (FILE_CHECK_MIMETYPES
                .get()
                .unwrap()
                .contains(expected[0].type_().as_str())
                || FILE_CHECK_SUBTYPES
                    .get()
                    .unwrap()
                    .contains(expected[0].subtype().as_str())
                || FILE_CHECK_EXT
                    .get()
                    .unwrap()
                    .contains(path.extension().unwrap_or(OsStr::new("")).to_str().unwrap())))
    {
        return Ok(());
    }

    let expected_collapsed = expected
        .iter()
        .map(|mime| mime_collapse(mime.essence_str()))
        .collect::<Vec<_>>();
    match MIME_DB.get().unwrap().get_mime_type_for_data(bytes) {
        Some((mime, _)) if !expected_collapsed.contains(&mime.essence_str()) => {
            Err((expected[0].to_string(), mime.to_string()))
        }
        _ => Ok(()),
    }
}

pub fn file_check_v1(bytes: &[u8], path: &Path) -> Result<(), V1Error> {
    file_check(bytes, path).map_err(|(expected, got)| V1Error::FileTypeMismatch { expected, got })
}
