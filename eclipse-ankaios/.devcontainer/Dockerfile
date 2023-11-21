FROM ghcr.io/eclipse-ankaios/app-ankaios-dev:latest as dev
ARG TARGETARCH

RUN apt-get update && apt-get -y install \
    git \
    jq \
    # Add your development tools here ...
    # If you like to use Rust please uncomment the two following lines...
    # build-essential \
    # && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y \
    # Cleanup
    && rm -rf /var/lib/apt/lists/* \
    && curl -sL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_${TARGETARCH} -o /usr/bin/yq \
    && chmod +x /usr/bin/yq

# Trust all repositories in this container
RUN git config --global --add safe.directory '*' \
    && echo "if [ -f /etc/bash_completion ] && ! shopt -oq posix; then\n . /etc/bash_completion \nfi" >> /root/.bashrc \
    && echo 'export PATH=$PATH:/workspaces/app/scripts:/workspaces/app/in-vehicle-stack/scenarios/smart_trailer_use_case/scripts' >> /root/.bashrc

# prod stage, please do not change it
FROM ghcr.io/eclipse-ankaios/app-ankaios-dev:latest
RUN curl -sL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_${TARGETARCH} -o /usr/bin/yq \
    && chmod +x /usr/bin/yq
COPY scripts/run_maestro.sh /run/ankaios/script/run_maestro.sh
RUN chmod +x /run/ankaios/script/run_maestro.sh
COPY config /run/ankaios/config
