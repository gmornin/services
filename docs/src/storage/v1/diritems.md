GET `/api/storage/v1/diritems/[token]/[path]`

---

List items in directory.

For example to access `/dir/deeper` in your storage, you GET `/api/storage/v1/diritems/token-gibberish/dir/deeper`, where `/dir/deeper` is a directory.

## Response

Status code: `200`

```json
{
  "type": "dir content",
  "content": [
    {
      "visibility": {
        "inherited": bool,
        "visibility": "hidden"/"private"/"public"
      },
      "is_file": bool,
      "name": String,
      "last_modified": u64,
      "size": u64
    },
    ...
  ]
}
```

## Possible errors

- `invalid token`
- `not verified`
- `permission denied`
- `file not found`
- `type mismatch`
- `external`
