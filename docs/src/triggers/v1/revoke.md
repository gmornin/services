GET `/api/triggers/v1/use/[trigger id]`

---

Revokes trigger so it can no longer be used.

## Response

Status code: `200`

```json
{
  "type": "revoked"
}
```

## Possible errors

- `trigger not found`
- `external`
