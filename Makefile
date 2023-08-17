SHELL := /usr/bin/env bash

## Development
test: test-build test-run
test-build:
	git submodule update --recursive && \
	buildah build -f tests.dockerfile -t wired-tests --layers=true
test-run:
	podman run wired-tests
