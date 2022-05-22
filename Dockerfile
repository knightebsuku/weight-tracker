FROM debian:buster-slim
WORKDIR /app

COPY target/armv7-unknown-linux-gnueabihf/release/weight-tracker /app/
COPY templates/ /app/templates/
COPY static/ /app/static/


RUN chmod +x weight-tracker

EXPOSE 8000
