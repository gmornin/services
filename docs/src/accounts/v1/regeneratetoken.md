POST `/api/accounts/v1/regeneratetoken`

---

Regenerate your token, invalidating all other logins.

## Request

```json
{
  "token": String,
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

- `invalid token`
- `password incorrect`
- `external`
