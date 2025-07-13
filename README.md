## Description

A small project to try out building a full stack webapp.
Intended to be a simple Planning Poker webapp.

Split into a Rust/Arctix backend and a Typescript/React frontend.

See their relevant readmes for more details:
- [Backend Readme](./backend/README.md)
- [Frontend Readme](./frontend/README.md)

## Dev environment

In order to develop this repo there are two devcontainers, one for frontend one for backend.
To run the full application you currently need to open both devcontainers simultaneously.
Run the backend service in one, then run the frontend dev server in the other.

## Publish

To publish a new release, simply push to master.
This will build a docker container using the dockerfile in the `deployment` directory.
To run the release build locally, run `docker run -p 8080:8080 ghcr.io/brstrutt/planning_knock_out_whist:latest`. Then you can visit `localhost:8080` in the browser to access the frontend.

## TODO:

### release
- Find some service that will host the release docker container for me and automatically pull the latest version when I bump it
- Investigate if I can somehow use the devcontainers to build the backend and frontend, and then just use the dockerfile to copy the build outputs.
- Decide if I should tie releases to version tags, rather than building a new release every time I push to main

### Devcontainers
- Add a second Frontend devcontainer that spins up the latest production backend docker container automatically on start. This is to make it easier to update JUST the frontend without needing to manually start the backend
- Figure out how to make modifying the fullstack environment easier. Currently you need to open a THIRD VSCode window in order to see and update the devctonainers/global readme, because the devcontainers pretend they can't see the root folder

