POST `/api/accounts/v1/access`

---

View users with this user have access to.

## Request

```json
{
  "token": String,
  "access_type": AccessType,
  "identifier": String?,
  "identifier-type": String?
}
```

`AccessType` includes `file`, `blue`, and `access`.

## Response

Status code: `200`

```json
{
  "type": "access",
  "users": [
    {
      "id": Int,
      "name": String
    }
    // ...
  ]
}
```

## Possible errors

- `no such user`
- `invalid token`
- `permission denied`
- `external`
