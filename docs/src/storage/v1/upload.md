POST with multipart `/api/storage/v1/upload/[token]/[path]` or `/api/storage/v1/upload-overwrite/[token]/[path]`

---

Upload a file to destination path.

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
- `file too large`
- `external`
