FROM docker.io/alpine:3.18.4 as base
RUN apk update && apk add --update-cache \
    # Development tools
    python3 \
    && rm -rf /var/cache/apk/* \
    && python3 -m ensurepip && pip3 install protobuf==3.20.2


FROM base as dev
ENV PYTHONPATH="${PYTHONPATH}:/usr/local/lib/digital_twin_proto"
RUN apk update && apk add --update-cache \
    # Development tools
    protoc \
    && rm -rf /var/cache/apk/*

ADD https://raw.githubusercontent.com/ArndtMeyer/maestro-challenge/main/in-vehicle-stack/scenarios/smart_trailer_use_case/interfaces/digital_twin_get_provider.proto /usr/local/lib/digital_twin_proto/digital_twin_get_provider.proto
RUN protoc --python_out=/usr/local/lib/digital_twin_proto/ --proto_path=/usr/local/lib/digital_twin_proto/ digital_twin_get_provider.proto && touch /usr/local/lib/digital_twin_proto/__init__.py

# prod stage
FROM base
ENV PYTHONPATH="${PYTHONPATH}:/usr/local/lib/digital_twin_proto"
COPY --from=dev /usr/local/lib/digital_twin_proto /usr/local/lib/digital_twin_proto
COPY ../ /python_provider

ENTRYPOINT ["python3", "-u", "/python_provider/src/main.py"]

# podman build -t python_provider:0.1 -f .devcontainer/Dockerfile .