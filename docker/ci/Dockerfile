FROM alpine:3.18.2

ARG TARGETARCH

COPY $TARGETARCH/excalidocker-config.yaml $TARGETARCH/entrypoint.sh $TARGETARCH/excalidocker /excalidocker/bin/

RUN chmod +x /excalidocker/bin/entrypoint.sh

ENV PATH /excalidocker/bin/:$PATH
