####################################################################################################
## Builder
####################################################################################################
# Start with a rust alpine image
FROM rust:alpine as builder
# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"
# if needed, add additional dependencies here
RUN apk add --no-cache musl-dev openssl-dev
# set the workdir and copy the source into it
WORKDIR /app
COPY ./ /app
# do a release build
RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
# use a plain alpine image, the alpine version needs to match the builder
FROM alpine
# if needed, install additional dependencies here and Create a group and User which is non-root.
RUN apk update && apk add --no-cache libgcc && addgroup -S appgroup && adduser -S appuser -G appgroup && \
    mkdir /app && chown -R appuser:appgroup /app
# Change to appuser and set work directory
USER appuser
WORKDIR /app
# copy the binary into the final image
COPY --chown=appuser:appgroup --from=builder /app/target/release/axum-api /app/.env /app/log4rs.yml ./
# set the binary as entrypoint
ENTRYPOINT ["/app/axum-api"]
