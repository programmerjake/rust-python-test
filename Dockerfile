FROM python:3 as rust_python

ENV PATH /root/.cargo/bin:$PATH
ENV USER root

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly-2019-08-21

FROM rust_python as build
RUN pip install --no-cache-dir maturin
COPY . /build/
WORKDIR /build/
RUN maturin sdist
FROM rust_python
COPY --from=build /build/target/wheels/ /root/
WORKDIR /root/
RUN pip install ~/*.tar.gz
