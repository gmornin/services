GET `/api/triggers/v1/peek/[trigger id]`

---

View trigger content without triggering it.

## Response

Status code: `200`

```json
{
  "type": "trigger peak",
  "value": {
    "type": "...",
    ...
  }
}
```

## Possible errors

- `trigger not found`
- `external`
