POST `/api/accounts/v1/delete`

---

Delete an account

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
  "type": "deleted"
}
```

## Possible errors

- `invalid token`
- `external`
