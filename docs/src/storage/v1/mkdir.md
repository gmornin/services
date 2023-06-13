POST `/api/storage/v1/mkdir`

---

Creates a directory

## Request

```json
{
  "token": String,
  "path": String
}
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
- `external`
