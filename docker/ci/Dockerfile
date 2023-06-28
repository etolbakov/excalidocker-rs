FROM scratch
ARG TARGETARCH

COPY $TARGETARCH/excalidocker-config.yaml $TARGETARCH/excalidocker /excalidocker/bin/

ENV PATH /excalidocker/bin/:$PATH

CMD ["/bin/bash", "-c", "./excalidocker --input-path ${INPUT_PATH} ${SKIP_DEPS:+ --skip-dependencies} --config-path ${CONFIG_PATH:-/excalidocker/bin/excalidocker-config.yaml}"]