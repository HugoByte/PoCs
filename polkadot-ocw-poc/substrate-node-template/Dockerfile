FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /substrate
COPY . /substrate
RUN cargo build --locked --release

FROM docker.io/library/ubuntu:20.04

COPY --from=builder /substrate/target/release/node-template /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /node node && \
	mkdir -p /node/.local/share/node && \
	chown -R node:node /node/.local

RUN chown node:node /usr/local/bin/node-template
RUN chmod +x /usr/local/bin/node-template

USER node

EXPOSE 30333 9933 9944 9615

ENTRYPOINT ["/usr/local/bin/node-template"]

