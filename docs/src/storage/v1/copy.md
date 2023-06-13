POST `/api/storage/v1/copy` or `/api/storage/v1/copy-overwrite`

---

Copy file to destination path, file can be owned by self or by others (only if the file is hidden or public).

## Request

```json
{
  "token": String,
  "from-userid": i64, (who to copy from, can be your own id)
  "from": String,
  "to": String
}
```

## Response

Status code: `201`

```json
{
  "type": "copied"
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
