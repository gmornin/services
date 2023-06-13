POST `/api/accounts/v1/regeneratetoken`

---

Regenerate your token, invalidating all other logins.

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
  "type": "regenerated",
  "token": String
}
```

## Possible errors

- `no such user`
- `password incorrect`
- `external`
