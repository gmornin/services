GET `/api/usercontent/v1/file/id/[userid]/[path]`

---

Returns file content.

For example to access `/dir/file.txt` in user with ID 1's storage, you GET `/api/usercontent/v1/file/id/1/dir/file.txt`, where `/dir/file.txt` is a file.

## Response

Status code: `200`

```json
[file content]
```

## Possible errors

- `permission denied`
- `file not found`
- `type mismatch`
- `external`
