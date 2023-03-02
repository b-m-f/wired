FROM docker.io/library/rust:buster

RUN apt-get update && apt-get upgrade -yy
RUN apt-get install bats

RUN mkdir -p /app/src
COPY Cargo.lock Cargo.toml /app
COPY src /app/src
WORKDIR /app
RUN cargo build -r
RUN cp target/release/wired /usr/bin
RUN chmod +x /usr/bin/wired

COPY tests /app/tests
WORKDIR /app/tests

ENTRYPOINT ["bats", "tests/tests.sh"]
