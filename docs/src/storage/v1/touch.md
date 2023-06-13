POST `/api/storage/v1/touch`

---

Creates an empty file.

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
