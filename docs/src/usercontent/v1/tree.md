GET `/api/usercontent/v1/tree/[userid]/[path]`

---

Tree items in directory.

## Response

Status code: `200`

```json
{
  "type": "tree",
  "content": TreeNode
}
```

Where `TreeNode` can either be a directory:

```json
{
  "type": "dir",
  "name": String,
  "visibility": {
    "inherited": bool,
    "visibility": Visibility // public, hidden or private
  }
}
```

or a file:

```json
{
  "type": "file",
  "name": String,
  "size": u64,
  "last_modified": u64,
  "visibility": {
    "inherited": bool,
    "visibility": Visibility,
  }
}
```

---

For example

```json
{
  "type": "tree",
  "content": {
    "type": "dir",
    "name": ".system",
    "visibility": {
      "inherited": true,
      "visibility": public,
    },
    "content": [
      {
        "type": "file",
        "name": "pfp.png",
        "size": 1165233,
        "last_modified": 1688643521,
        "visibility": {
          "inherited": true,
          "visibility": "public",
        }
      }
    ]
  }
}
```

## Possible errors

- `permission denied`
- `file not found`
- `type mismatch`
- `external`
