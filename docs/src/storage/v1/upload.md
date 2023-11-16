POST with multipart `/api/storage/v1/upload/[token]/[path]`, `/api/storage/v1/upload-createdirs/[token]/[path]`, `/api/storage/v1/upload-overwrite/[token]/[path]` and `/api/storage/v1/upload-createdirs-overwrite/[token]/[path]`

---

Upload a file to destination path.

- `upload` returns an error if the directory the file is uploading to does not exist, or the file path is occupied.
- `upload-overwrite` overwrites whatever is in the path.
- `upload-createdirs` create parent directory for the upload target if it does not exist.
- `upload-createdirs-overwrite` a mix of both

## Request

```json
[multipart file post]
```

## Response

Status code: `201`

```json
{
  "type": "file item created"
}
```

## Possible errors

- `invalid token`
- `not verified`
- `permission denied`
- `path occupied` (non overwrite only)
- `file not found`
- `storage full`
- `external`
