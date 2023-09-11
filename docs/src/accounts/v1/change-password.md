POST `/api/accounts/v1/change-password`

---

Regenerate your token, invalidating all other logins.

## Request

```json
{
  "new": String, // the password you want to change to
  "old": String,
  "token": String
}
```

## Response

Status code: `200`

```json
{
  "type": "password changed"
}
```

## Possible errors

- `password incorrect`
- `invalid token`
- `external`
