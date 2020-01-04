FROM rust:1.26.0

WORKDIR ~/rust/another_kafka_poc
COPY . .

RUN cargo install

CMD ["another_kafka_poc"]
