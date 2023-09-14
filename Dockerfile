FROM rust:1.67

WORKDIR /usr/src/crud
COPY . ./crud

RUN apt-get update && apt-get install -y cmake clang