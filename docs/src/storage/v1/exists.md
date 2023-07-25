POST `/api/storage/v1/exists`

---

Check if file exists

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
  "type": "exists",
  "value": bool
}
```

## Possible errors

- `invalid token`
- `not verified`
- `permission denied`
- `external`
