FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/testing-actix
COPY ./testing-actix .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/testing-actix /usr/local/bin/testing-actix

# EXPOSE 5000

# HEALTHCHECK --interval=30s --timeout=3s \
#     CMD curl -s -S -f http://localhost:80/healthcheck || exit 1

CMD ["testing-actix"]