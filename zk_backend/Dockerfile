ARG BASE_IMAGE=rust:1.68.0-alpine3.17

FROM ${BASE_IMAGE} AS src
# Temporary workaround for alpine/non-alpine images
RUN apk add --update --no-cache build-base || true
WORKDIR /workspace
COPY . .
RUN cargo fetch

FROM src AS test
RUN cargo test

FROM src AS build
# https://github.com/rust-lang/rust/pull/40113#issuecomment-323193341
ARG RUSTFLAGS="-C target-feature=-crt-static"
RUN cargo build --release

FROM scratch AS build-result
COPY --from=build \
     /workspace/target/release/libzkkyc_c_bindings.so \
     /workspace/zkkyc_c_bindings/zkkyc_lib.h \
     /
