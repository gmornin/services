POST `/api/accounts/v1/change-email`

---

Regenerate your token, invalidating all other logins.

## Request

```json
{
  "new": String, // the email you want to change to
  "token": String,
  "password": String
}
```

## Response

Status code: `200`

```json
{
  "type": "email changed",
  "verify": Boolean
}
```

## Possible errors

- `email taken`
- `password incorrect`
- `invalid token`
- `cooldown`
- `external`
