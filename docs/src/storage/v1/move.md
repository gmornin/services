POST `/api/storage/v1/move` or `/api/storage/v1/move-overwrite`

---

Move/rename file to destination path.

## Request

```json
{
  "token": String,
  "from": String,
  "to": String
}
```

## Response

Status code: `200`

```json
{
  "type": "moved"
}
```

## Possible errors

- `invalid token`
- `not verified`
- `permission denied`
- `path occupied` (non overwrite only)
- `file not found`
- `external`
