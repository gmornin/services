POST `/api/accounts/v1/set-status`

---

Change user status.

## Request

```json
{
  "token": String,
  "new": String
}
```

## Response

Status code: `200`

```json
{
  "type": "profile updated"
}
```

## Possible errors

- `invalid token`
- `exceeds maximum length`
- `external`
