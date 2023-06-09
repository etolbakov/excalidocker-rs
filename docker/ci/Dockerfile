FROM ubuntu:22.04

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    ca-certificates 

ARG TARGETARCH

COPY $TARGETARCH/excalidocker-config.yaml $TARGETARCH/excalidocker /excalidocker/bin/

ENV PATH /excalidocker/bin/:$PATH

CMD ["sh", "-c", "excalidocker --input-path ${INPUT_PATH} ${SKIP_DEPS:+ --skip-dependencies}"]