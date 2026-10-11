# The public demo: `caudal serve`, the whole network in one process on the in-memory ledger.
FROM rust:1.99-slim-bookworm AS build
WORKDIR /src
COPY . .
RUN cargo build --release --locked -p caudal

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=build /src/target/release/caudal /usr/local/bin/caudal
ENV PORT=8080
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/caudal"]
CMD ["serve"]
