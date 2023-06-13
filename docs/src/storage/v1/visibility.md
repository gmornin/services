POST `/api/storage/v1/set-visibility` or `/api/storage/v1/remove-visibility`

---

Change file or directory visibility.

## Request (set)

```json
{
  "token": String,
  "path": String,
  "visibility": "hidden"/"private"/"public"
}
```

## Request (remove)

```json
{
  "token": String,
  "path": String,
}
```

## Response

Status code: `200`

```json
{
  "type": "visibility changed"
}
```

or

Status code: `304`

```json
{
  "type": "nothing changed"
}
```

## Possible errors

- `invalid token`
- `not verified`
- `permission denied`
- `file not found`
- `external`
