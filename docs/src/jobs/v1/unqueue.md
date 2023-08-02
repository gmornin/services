POST `/api/jobs/v1/unqueue`

---

Remove a queued job

## Request

```json
{
  "token": String,
  "id": u64
}
```

## Response

Status code: `200`

```json
{
  "type": "unqueued"
}
```

## Possible errors

- `invalid token`
- `external`
- `job not found`
