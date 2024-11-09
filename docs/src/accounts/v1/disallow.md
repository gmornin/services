POST `/api/accounts/v1/disallow`

---

Remove access for a user.

## Request

```json
{
  "token": String,
  "identifier": String,
  "identifier-type": "id"/"username"/"email",
  "access_type": AccessType
}
```

## Response

Status code: `200`

```json
{
  "type": "disallowed"
}
```

## Possible errors

- `no such user`
- `invalid token`
- `external`
