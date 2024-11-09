POST `/api/accounts/v1/allow`

---

Allow access for a user.

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
  "type": "allowed"
}
```

## Possible errors

- `no such user`
- `invalid token`
- `external`
