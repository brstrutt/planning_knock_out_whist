## Description

Backend service for the planning_knock_out_wist fullstack webapp.
This service is intended to serve the web frontend, and to provide an API that the web frontend can then call.

## Tools

- Rust (programming language)
- Actix (backend framework)
- Cargo (build tool)
- cargo-watch (enhancement to Cargo that automatically rebuilds the project when the code changes)

## How to develop

1. Open the backend devcontainer. This contains the dev environment
2. (initial setup) follow the [frontend readme](../frontend/README.md) to create an initial build of the frontend
3. Run `cargo run` to run the backend server
4. Run `cargo watch -x run` to run the backend server with automatic recompilation on code changes
