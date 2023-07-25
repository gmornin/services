GET `/api/usercontent/v1/exists/id/[id]/[path]`

---

Check if file exists

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
