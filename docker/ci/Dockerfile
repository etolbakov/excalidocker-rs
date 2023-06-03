FROM ubuntu:22.04

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    ca-certificates 

ARG TARGETARCH

ADD $TARGETARCH/excalidocker /excalidocker/bin/

ENV PATH /excalidocker/bin/:$PATH

# CMD ["sh", "-c", "excalidocker --input-path ${input_path}"]