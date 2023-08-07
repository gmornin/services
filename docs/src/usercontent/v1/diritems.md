GET `/api/usercontent/v1/diritems/id/[userid]/[path]`

---

List items in directory.

For example to access `/dir/deeper` in user with ID 1's storage, you GET `/api/usercontent/v1/diritems/1/dir/deeper`, where `/dir/deeper` is a directory.

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

- `permission denied`
- `file not found`
- `type mismatch`
- `external`
