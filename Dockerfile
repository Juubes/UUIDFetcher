# syntax=docker/dockerfile:experimental
FROM rust as build
COPY . .
RUN cargo build --release

FROM debian:11-slim
RUN apt-get install libssl1.1
COPY --from=build "./target/release/uuid-fetcher" .
CMD [ "./uuid-fetcher -s" ]
