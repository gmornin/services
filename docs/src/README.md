# Overview

Goodmorning Services contains the api stuff, manages various accounts, storage related things.

## Api structure

Structured similar to a REST api, all paths can be found in `/api/[service]/vX/[path...]`.

Some requires posts requests, some get requests, and sometimes multipart post if the client is trying to upload a file.

## Requirements

To run this, use `cargo r (--release)`:

- Copy `dotenv-example` to `.env`, and fill in the blanks.
- Have mongodb running as specified in `.env`.

If you are using this as a dependency (such as `gmt-server`):

- Run `goodmorning_services::init()` at the beginning.

## Bindings

- [Rust](https://github.com/GoodMorning-Network/rust-bindings)
