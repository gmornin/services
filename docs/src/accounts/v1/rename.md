POST `/api/accounts/v1/rename`

---

Change username.

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
  "type": "renamed"
  "token": String
}
```

## Possible errors

- `invalid username`
- `invalid token`
- `password incorrect`
- `external`
