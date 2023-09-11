POST `/api/accounts/v1/resend-verify`

---

Resend verification message to email.

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
  "type": "verification sent"
}
```

## Possible errors

- `invalid token`
- `nothing changed`
- `cooldown`
- `external`
