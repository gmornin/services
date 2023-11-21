POST `/api/storage/v1/delete-multiple`

---

Delete multiple file in order.

## Request

```json
{
  "token": String,
  "paths": [String]
}
```

## Response

Status code: `200`

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
- `file not found`
- `external`
