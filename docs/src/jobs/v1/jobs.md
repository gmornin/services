POST `/api/jobs/v1/jobs`

---

List queued and running jobs.

## Request

```json
{
  "token": String
}
```

## Response

Status code: `200`

```json
{
  "type": "jobs",
  "current": [
    {
      "id": u64,
      "task": {
        "type": "compile",
        "from": FromFormat,
        "to": ToFormat,
        "compiler": Compiler,
        "path": String,
      }
    }
  ],
  "queue": [
    // ...
  ]
}
```

## Possible errors

- `invalid token`
- `external`
