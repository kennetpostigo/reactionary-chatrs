# Base Image
FROM rust as build
# Copy our app into that directory
COPY . .

ENV SQLX_OFFLINE=true

# Build our app
RUN cargo build --release --verbose

# Base Image
FROM alpine
# Copy Files from build
COPY --from=build ./target/release/api ./api
# Running on
EXPOSE 8080
# Run our app
CMD api
