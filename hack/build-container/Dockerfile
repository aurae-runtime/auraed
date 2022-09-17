FROM rust:1-bullseye

RUN apt-get update && apt-get upgrade -y && apt-get install -y \
    flex bison bc libelf-dev \
    cpio zstd \
    gperf \
    ninja-build \
    protobuf-compiler \
    libprotobuf-dev
    
RUN mkdir /aurae
RUN mkdir /target
WORKDIR /aurae