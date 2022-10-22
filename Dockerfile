FROM debian:sid-slim

COPY ./linkz /app/linkz
WORKDIR /app

RUN apt-get update && apt-get install libssl-dev ca-certificates -y && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["/app/linkz"]
