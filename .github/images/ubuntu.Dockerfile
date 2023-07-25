FROM ubuntu:22.04

RUN rm -f /var/lib/man-db/auto-update && \
	apt-get update && apt-get -y install pkg-config libaio-dev libblkid-dev \
              libkeyutils-dev liblz4-dev libsodium-dev liburcu-dev libzstd-dev \
              uuid-dev zlib1g-dev valgrind libudev-dev python3-docutils \
              debhelper llvm clang libclang-dev rustc cargo \
              devscripts
RUN useradd -m builder
USER builder
