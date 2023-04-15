# account

All paths in this section are located in `/api/services/v1/account`

## [Post] `/create`

Creates an account.

### Post content

```json
{
    "username": String,
    "email": String,
    "password": String
}
```

### Responses

- OK: Created
- Err: UsernameTaken, EmailTaken, External

## [Post] `/delete`

Deletes an account.

### Post content

```json
{
    "token": String
}
```

### Responses

- OK: Deleted
- Err: InvalidToken, External

## [Post] `/gettoken`

Get a token using password.

### Post content

```json
{
    "identifier": String,
    "identifier_type": `email`, `id`, or `username`,
    "password": String
}
```

`identifier` should be of `identifier_type`, for example if `identifier_type` is `id` then `identifier` should be 

## [Post] `/regeneratetoken`

Regenerates token, useful for "log out of all devices" scenario.

### Post content

```json
{
    "identifier": String,
    "identifier_type": `email`, `id`, or `username`,
    "password": String
}
```

### Responses

- OK: RegenerateToken
- Err: NoSuchUser, PasswordIncorrect, External

## [Post] `/rename`

Changes a username.

### Post content

```json
{
    "token": String,
    "new": String
}
```

`new` would be the new username to change to.

### Responses

- OK: Renamed
- Err: InvalidToken, UsernameTaken, External
