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
  "type": "deleted"
}
```

## Possible errors

- `invalid token`
- `not verified`
- `permission denied`
- `file not found`
- `external`
