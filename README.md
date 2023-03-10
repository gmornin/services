Service crate for websites, including account management.

It can also be ran as a binary for quick testing.

## Progress

- [x] Account creation
- [x] Checking passwords
- [x] Trigger actions
- [ ] Email verification
- [ ] Changing email address/password/username

## Definitions

### IdentifierType

`email`, `id`, or `username`.

## Paths

### [Post] `/api/v1/account/create`

Creates a new account.

```
{
	"username": String,
	"email": "String",
	"password": String
}
```

### [Post] `/api/v1/account/delete`

Deletes account.

```
{
	"token": String
}
```

### [Post] `/api/v1/account/gettoken`

Get's account token.

```
{
	"identifier": String,
	"identifier_type" IdentifierType
}
```

### [Post] `/api/v1/account/regeneratetoken`

Reset's account token.

```
{
	"token" String
}
```
