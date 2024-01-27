FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /substrate
COPY . /substrate
RUN cargo build --locked --release

FROM docker.io/library/ubuntu:20.04

COPY --from=builder /substrate/target/release/node-template /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /substrate substrate && \
	mkdir -p /data /substrate/.local/share/substrate && \
	chown -R substrate:substrate /data && \
	ln -s /data /substrate/.local/share/substrate && \
	ldd /usr/local/bin/node-template && \
	rm -rf /usr/bin /usr/sbin && \
	/usr/local/bin/node-template --version

RUN chown substrate:substrate /usr/local/bin/node-template
RUN chmod +x /usr/local/bin/node-template

USER substrate

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/node-template"]

