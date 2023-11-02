FROM ghcr.io/eclipse-ankaios/app-ankaios-dev:latest as dev

RUN apt-get update && apt-get -y install \
    git \
    # Add your development tools here ...
    # Cleanup
    && \
    rm -rf /var/lib/apt/lists/*

# Trust all repositories in this container
RUN git config --global --add safe.directory '*'

# prod stage, please do not change it
FROM ghcr.io/eclipse-ankaios/app-ankaios-dev:latest
COPY scripts/run_ankaios.sh /run/ankaios/script/run_ankaios.sh
RUN chmod +x /run/ankaios/script/run_ankaios.sh
COPY config/startupState.yaml /run/ankaios/config/startupState.yaml
CMD ["/run/ankaios/script/run_ankaios.sh", "2>&1"]
