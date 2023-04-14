Service crate for websites, including account management.

It can also be ran as a binary for quick testing.

## ~~Progress~~

Moved to [github projects](https://github.com/orgs/GoodMorning-Network/projects/1)

## Definitions

### IdentifierType

`email`, `id`, or `username`.

### ItemVisibility

`public`, `hidden` or `private`.

## Accounts

### [Post] `/api/services/v1/account/create`

Creates a new account.

```json
{
	"username": String,
	"email": "String",
	"password": String
}
```

### [Post] `/api/services/v1/account/delete`

Deletes account.

```json
{
	"token": String
}
```

### [Post] `/api/service/v1/account/gettoken`

Get's account token.

```json
{
	"identifier": String,
	"identifier_type" IdentifierType
}
```

### [Post] `/api/services/v1/account/regeneratetoken`

Reset's account token.

```json
{
	"token" String
}
```

## Triggers

### [Get] `/api/services/v1/triggers/use/{id}`

Runs a trigger (e.g. email verification)

## Storage

### [Post with multipart] `/api/services/v1/storage/{token}/overwrite/{path}`

Overwrites an existing file

Example

```sh
curl http://localhost:8080/api/v1/storage/{token}/overwrite/test.txt -X POST -F 'file=@Cargo.toml'
```

### [Post with multipart] `/api/services/v1/storage/{token}/write_new/{path}`

Upload a new file

Example

```sh
curl http://localhost:8080/api/v1/storage/{token}/write_new/test.txt -X POST -F 'file=@Cargo.toml'
```

### [Get] `/api/services/v1/storage/{token}/read/{path}`

Gets a file, or directory info

### [Get] `/api/services/v1/storage/{token}/mkdir/{path}`

Creates a directory

### [Post] `/api/services/v1/storage/{token}/set_visibility/{path}`

Changes visibility for a file/folder

```json
{
    "new": ItemVisibility
}
```

### [Get] `/api/services/v1/storage/{token}/remove_visibility/{path}`

Changes visibility for a file/folder - back of inheriting parent visibility
