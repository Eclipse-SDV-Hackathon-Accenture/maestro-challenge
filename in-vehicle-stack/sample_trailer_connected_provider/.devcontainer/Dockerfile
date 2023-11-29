# podman build -t sample_trailer_connected_provider:0.1 -f .devcontainer/Dockerfile .
FROM docker.io/ubuntu:22.04 as base
RUN apt-get update && apt-get -y install \
    # Development tools
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

FROM base as dev
ENV PYTHONPATH="${PYTHONPATH}:/usr/local/lib/digital_twin_proto"

RUN apt-get update && apt-get -y install \
    # Development tools
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

COPY requirements.txt .
RUN pip3 install -r requirements.txt
ADD https://raw.githubusercontent.com/Eclipse-SDV-Hackathon-Accenture/maestro-challenge/main/in-vehicle-stack/scenarios/smart_trailer_use_case/interfaces/digital_twin_get_provider.proto /usr/local/lib/digital_twin_proto/digital_twin_get_provider.proto
RUN protoc --python_out=/usr/local/lib/digital_twin_proto/ --proto_path=/usr/local/lib/digital_twin_proto/ digital_twin_get_provider.proto && touch /usr/local/lib/digital_twin_proto/__init__.py

# prod stage
FROM base
ENV PYTHONPATH="${PYTHONPATH}:/usr/local/lib/digital_twin_proto"
COPY --from=dev /usr/local/lib/digital_twin_proto /usr/local/lib/digital_twin_proto
COPY . /sample_trailer_connected_provider
RUN pip3 install -r /sample_trailer_connected_provider/requirements.txt

ENTRYPOINT ["python3", "-u", "/sample_trailer_connected_provider/src/main.py"]