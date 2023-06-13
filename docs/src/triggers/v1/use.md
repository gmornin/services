GET `/api/triggers/v1/use/[trigger id]`

---

Uses trigger - and runs the code.

## Response

Status code: `200`

```json
{
  "type": "triggered"
}
```

## Possible errors

- `trigger not found`
- `external`
