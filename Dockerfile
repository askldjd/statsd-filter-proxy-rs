FROM rust:1.51-slim

RUN mkdir /app
WORKDIR /app
COPY . .
RUN ls -hal
RUN cargo build --release

COPY entrypoint.sh /

EXPOSE 8125
ENTRYPOINT ["/app/entrypoint.sh"]