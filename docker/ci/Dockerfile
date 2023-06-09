FROM ubuntu:22.04

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    apt-get clean && \
    rm -rf /var/cache/apt/archives/* /var/lib/apt/lists/* && \
    apt-get -y autoremove

# apt-get remove -y gcc  && \

ARG TARGETARCH

COPY $TARGETARCH/excalidocker-config.yaml $TARGETARCH/excalidocker /excalidocker/bin/

ENV PATH /excalidocker/bin/:$PATH

CMD ["sh", "-c", "excalidocker --input-path ${INPUT_PATH} ${SKIP_DEPS:+ --skip-dependencies} --config-path ${CONFIG_PATH:-/excalidocker/bin/excalidocker-config.yaml}"]