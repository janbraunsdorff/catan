ARG RUST_VERSION=1.75.0
ARG APP_NAME=app
ARG BUILDPLATFORM=linux/amd64
ARG TARGETPLATFORM=linux/amd64
################################################################################
# Create a stage for building the application.
FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-slim-buster AS build
USER root

RUN apt-get update && apt-get upgrade
RUN apt-get install -y pkg-config libssl-dev musl-dev libc6-dev  build-essential musl-tools


RUN rustup target add x86_64-unknown-linux-musl
RUN update-ca-certificates


RUN mkdir -p /usr/src/ping
WORKDIR /usr/src/ping


# Build the application.
RUN openssl version -d
RUN find / -name "*include*" -type d 




ENV USER=runner
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /wd

RUN --mount=type=bind,source=api,target=api \
    --mount=type=bind,source=app,target=app \
    --mount=type=bind,source=database,target=database \
    --mount=type=bind,source=game,target=game \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    <<EOF
set -x
cargo build --target x86_64-unknown-linux-musl  --bins --release
ls -al /wd/target/x86_64-unknown-linux-musl/release
EOF


FROM scratch AS final


#COPY --from=build /etc/passwd /etc/passwd
#COPY --from=build /etc/group /etc/group
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

COPY --from=build  /wd/target/x86_64-unknown-linux-musl/release/app /

# Copy the executable from the "build" stage.
# Expose the port that the application listens on.
EXPOSE 4000
#USER runner:runner
# What the container should run when it is started.

# CMD ["ldd", "/app"]
CMD ["/app"]
