POST `/api/accounts/v1/create`

---

Create an account

## Request

```json
{
  "username": String,
  "email": String,
  "password": String
}
```

## Response

Status code: `201`

```json
{
  "type": "created",
  "id": i64,
  "token": String,
  "verify": Boolean
}
```

## Possible errors

- `invalid username`
- `username taken`
- `email taken`
- `external`
