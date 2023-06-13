POST `/api/accounts/v1/login`

---

Get user ID and token from identifier and password.

## Request

```json
{
  "identifier": String,
  "identifier-type": "id"/"username"/"email",
  "password": String
}
```

## Response

Status code: `200`

```json
{
  "type": "login",
  "id": i64,
  "token": String
}
```

## Possible errors

- `no such user`
- `password incorrect`
- `external`
