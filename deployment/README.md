## Description

This folder is intended to hold anything related to deploying the webapp.
Right now that's just a dockerfile that builds a docker image of the backend service.

To build the docker container, `cd` to the root of the directory and run `docker build -f deployment/dockerfile .`.
