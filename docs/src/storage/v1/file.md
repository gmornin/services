GET `/api/storage/v1/file/[token]/[path]`

---

Returns file content.

For example to access `/dir/file.txt` in your storage, you GET `/api/storage/v1/diritems/token-gibberish/dir/file.txt`, where `/dir/file.txt` is a file.

## Response

Status code: `200`

```json
[file content]
```

## Possible errors

- `invalid token`
- `not verified`
- `permission denied`
- `file not found`
- `type mismatch`
- `external`
