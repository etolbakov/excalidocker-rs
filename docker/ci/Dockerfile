FROM alpine:3.18.2
MAINTAINER Evgeny Tolbakov <ev.tolbakov@gmail.com>

ARG TARGETARCH

COPY $TARGETARCH/excalidocker-config.yaml $TARGETARCH/entrypoint.sh $TARGETARCH/excalidocker /excalidocker/bin/

ENV PATH /excalidocker/bin/:$PATH

ENTRYPOINT ["./excalidocker/bin/entrypoint.sh"]