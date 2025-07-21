## Description

A small project to try out building a full stack webapp.
Intended to be a simple Planning Poker webapp.

Split into a Rust/Arctix backend and a Typescript/React frontend.

See their relevant readmes for more details:

- [Backend Readme](./backend/README.md)
- [Frontend Readme](./frontend/README.md)

The latest release is published on Digital Ocean's App Platform

## Dev environment

The dev environment is encapsulated in a devcontainer.
It provides the basic tools needed to build both frontend and backend.
It also includes useful vscode extensions/settings for working on this repo.

## To develop

See [backend](./backend/README.md) / [frontend](./frontend/README.md) readmes for more details.

You probably want to:

- open vscode terminal + `cd backend` + `cargo watch -x run`
- open vscode terminal + `cd frontend` + `npm run dev` (only if you want to modify the frontend)

## Publish

To publish a new release, simply push to master.
Github Actions will:

- build the deployment docker container
- push it to github container registry
- deploy it to digital ocean

To run the release build locally, run `docker run -p 8080:8080 ghcr.io/brstrutt/planning_knock_out_whist:latest`. Then you can visit `localhost:8080` in the browser to access the frontend.
