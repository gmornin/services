# GM Services

> ***If you are just looking for the API docs, [here](https://siriusmart.github.io/gm-services) it is***.

This crate provides account and storage management, which can be shared by many services.

## Overview

This repo can either be ran as a standalone API server, or used as a crate to build more specialised web servers without rewriting the basic functionalities.

Not only does this crate provides predefined API endpoints, it also provides structs and functions that are not included in the [GM Rust bindings](https://github.com/gmornin/rust-bindings). Allowing for the ease of creating servers with the same shared account/storage management system while keeping the bindings crate minimal and quick to compile.

### Features

This crate manages user storage by allowing a configurable *private folder path* for each user, and provides a ***blazingly fast*** account system by storing user info a MongoDB database.

#### Account management

All account data is stored in MongoDB.

- Each user receives a unique (but not random) **user ID**. The first user to create an account gets an ID of `1`, the second gets `2`, and so on. (This is inspired by the ID system used in osu!)
- Users are able carry out **basic operations** on their account, including changing username, email and password, creating, logging in and deleting account, and invalidating all other logged in sessions by requesting the token to be regenerated.
- **Email verification** out of the box using SMTP providers from such as Gmail.

> Users who are not verified will be permission denied in many actions.

#### Storage management

Each user is given a "user root directory", his file operations will be limited to be within that.

- Basic **file operations** such as copying, deleting, moving, ls/directory tree, uploading files to server, etc.
- Storage size **limit** for each user.
- **File type validation** - ensure all files are what they claim to be (content same as file extension).
- File/folder **visibility** can be set to *public*, *hidden* (accessible to public only with URL) or *private*.

#### URL triggers

Custom code can be ran when a URL is visited with a unique ID.

This is currently used for *email verification*.

#### Job system

The job system prevents a user from creating too many CPU intensive tasks, such as compiling large LaTex files.

Each user is allowed:

- A queue: where new tasks are added.
- Limited number of concurrent runs: only *n* tasks can be run at the same time.

Once the current running task has been completed, the next oldest task in queue (if not empty) will be ran.

<!-- #### Profile system (Tex) -->
<!--  -->
<!-- Currently, user profile along with the user's profile picture are stored as files under the user's private folder (within `/tex/.system/`). This may be subject to change as there might be benefits of storing it in MongoDB, such as ease of indexing for search and speed. -->
<!--  -->
<!-- The user is allowed to make basic modifications to their bio/user description, profile picture, and add connection badges to their profile. -->
<!--  -->
<!-- #### Compiling (Tex) -->
<!--  -->
<!-- One of the core features of GM Tex is the ability to compile files from one formats to another. -->
<!--  -->
<!-- Currently, here are the valid compilations. -->
<!--  -->
<!-- |Original|Destination|Compiler| -->
<!-- |---|---|---| -->
<!-- |Markdown|Html|[Pulldown Cmark](https://github.com/raphlinus/pulldown-cmark)| -->
<!-- |LaTex|PDF|[PDFlatex](https://man.archlinux.org/man/pdflatex.1.en)| -->
<!--  -->
<!-- > Note that users may only use files in their own user directory when compiling with PDFlatex. -->

## Usage

### Standalone server

**Requirements**:

- **MongoDB** server running at `localhost:27017` (address configurable in configs).
- **sudo** which is a hard dependency, as accessing ports such as 80 and 443 requires root privileges. So sudo is always required to keep things consistent.
- Other **basic dependencies** such as OpenSSL.

When running as a standalone server, it has **no user interface** of any sort. However, you (and anyone) can still interact with it using the API [as documented](https://siriusmart.github.io/gm-services). You may also use the [GM CLI](https://github.com/gmornin/gm-cli) to interact with it with command line.

#### Config files

When first running the server, config files will be generated at `~/.config/gm`. But because the server escalates to root on start, the config files generated should be at `/root/.config/gm`.

### As a dependency

To use it as a dependency, you must first initialise the required values.

```rs
goodmorning_services::init();
```

Then construct the `HttpServer`.

```rs
let jobs: Data<Jobs> = Data::new(Jobs::default()); // make sure that it is declared outside the `move ||` closure

HttpServer::new(move || {
    App::new()
        .service(api::scope())
        // add your own routes
        .app_data(jobs.clone())
    })
.bind(("0.0.0.0", 80))
.unwrap()
.run()
.await
.unwrap();
```

> You may also use HTTPS instead. For how that can be done, check out the working example in [main.rs](https://github.com/gmornin/services/blob/master/src/main.rs).

### Development

The development follows a few basic principles.

1. **Do not remove** or rename fields, this break applications interacting with the API.
2. **Avoid making any changes** to a stable version of the API.
3. **When adding fields** to a POST requirement, use `#[serde(default)]`.

#### Structure and versioning

```
api/
├── accounts/
│   └── v1/
│       ├── create
│       ├── login
│       ├── delete
│       └── ...
├── storage/
│   └── v1/
│       ├── upload
│       ├── copy
│       └── dir-items
└── tex/
    ├── generic/
    │   └── v1/
    │       ├── set-status
    │       └── profile
    └── compile/
        └── v1/
            ├── simple
            └── ...
```

Note that the API is divided into modules, and each service/module can have its own versions. For example `accounts` can be still at `v1` while `storage` is already updated to `v3`.

Seen both `generic` and `compile` goes under `tex/`, here `tex/` is a *category* rather than a service. As they are both closely related to the project of GM Tex.

> TODO would be allowing categories and services to be enabled/disabled in config, as well as conditionally compiling certain services (feature gating).
