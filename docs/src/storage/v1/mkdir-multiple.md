POST `/api/storage/v1/mkdir-multiple`

---

Creates multiple directories in order

## Request

```json
{
  "token": String,
  "paths": [String]
}
```

## Response

Status code: `201`

```json
{
  "type": "multi",
  "res": [V1Response]
}
```

## Possible errors

- `invalid token`
- `not verified`
- `permission denied`
- `external`
