POST `/api/storage/v1/delete`

---

Delete file.

## Request

```json
{
  "token": String,
  "path": String
}
```

## Response

Status code: `200`

```json
{
  "type": "copied"
}
```

## Possible errors

- `invalid token`
- `not verified`
- `permission denied`
- `file not found`
- `external`
