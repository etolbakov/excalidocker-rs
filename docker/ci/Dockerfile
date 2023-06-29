FROM scratch
ARG TARGETARCH

COPY $TARGETARCH/excalidocker-config.yaml $TARGETARCH/excalidocker /excalidocker/bin/

ENV PATH /excalidocker/bin/:$PATH

CMD ["excalidocker", "--input-path", "${INPUT_PATH}"]