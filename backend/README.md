## Description

Backend service for the planning_knock_out_wist fullstack webapp.
This service is intended to serve the web frontend, and to provide an API that the web frontend can then call.

## Tools

- Rust (programming language)
- Actix (backend framework)
- Cargo (build tool)
- cargo-watch (enhancement to Cargo that automatically rebuilds the project when the code changes)

## How to develop

0. Open the devcontainer and cd to this directory. Then the following commands will work.
1. Run `cargo run` to run the backend server
2. Run `cargo watch -x run` to run the backend server with automatic recompilation on code changes

## Other

Might be nice to auto-generate API documentation from the source code itself.
[This article](https://medium.com/netwo/documenting-api-for-actix-web-b575adb841a1) suggests a potentially nice way of doing it, using the `apistos` and `schemars` crates.
