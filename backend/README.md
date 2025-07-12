## Description

Backend service for the planning_knock_out_wist fullstack webapp.
This service is intended to serve the web frontend, and to provide an API that the web frontend can then call.

## Tools

- Rust (programming language)
- Actix (backend framework)
- Cargo (build tool)
- cargo-watch (enhancement to Cargo that automatically rebuilds the project when the code changes)

## How to develop

- Open the Rust devcontainer devined in the parent's `.devcontainer` directory. This contains the development environment.
- Run `cargo watch -x run` to start the backend service. It will automatically recompile and restart when you change the code

## How to deploy

TODO: Figure out how to deploy. It'll probably use github actions.


