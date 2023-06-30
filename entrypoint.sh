#!/bin/bash
set -e

if [[ -n "$INPUT_PATH" ]]; then
  exec excalidocker --input-path "$INPUT_PATH" ${SKIP_DEPS:+ --skip-dependencies} --config-path ${CONFIG_PATH:-/excalidocker/bin/excalidocker-config.yaml}
elif [[ -n "$SHOW_CONFIG" ]]; then
  exec excalidocker -C
else
  exec excalidocker -h
fi
