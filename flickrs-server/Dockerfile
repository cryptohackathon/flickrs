FROM rustlang/rust:nightly as builder

COPY . .
RUN cargo build --release --all-features

RUN ls target/
RUN ls target/release

FROM ubuntu:20.04

RUN apt-get update && apt-get install -y sqlite3

COPY --from=builder target/release/flickrs_server /usr/local/bin/

CMD ["/usr/local/bin/flickrs_server"]
