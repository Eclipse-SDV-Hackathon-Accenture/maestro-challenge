FROM ghcr.io/eclipse-ankaios/app-ankaios-dev:latest as dev

RUN apt-get update && apt-get -y install \
    git \
    jq \
    # Add your development tools here ...
    # Cleanup
    && \
    rm -rf /var/lib/apt/lists/* \
    && curl -L https://github.com/mikefarah/yq/releases/latest/download/yq_linux_amd64 -o /usr/bin/yq \
    && chmod +x /usr/bin/yq

RUN mkdir /usr/bin/grpcurl.d \
    && curl -sSL https://github.com/fullstorydev/grpcurl/releases/download/v1.8.7/grpcurl_1.8.7_linux_x86_64.tar.gz | tar -xvz --directory /usr/bin/grpcurl.d \
    && ln /usr/bin/grpcurl.d/grpcurl /usr/bin/grpcurl

# Trust all repositories in this container
RUN git config --global --add safe.directory '*' \
    && echo "if [ -f /etc/bash_completion ] && ! shopt -oq posix; then\n . /etc/bash_completion \nfi" >> /root/.bashrc \
    && echo 'export PATH=$PATH:/workspaces/app/scripts' >> /root/.bashrc

# prod stage, please do not change it
FROM ghcr.io/eclipse-ankaios/app-ankaios-dev:latest
RUN curl -L https://github.com/mikefarah/yq/releases/latest/download/yq_linux_amd64 -o /usr/bin/yq \
    && chmod +x /usr/bin/yq
COPY scripts/run_maestro.sh /run/ankaios/script/run_maestro.sh
RUN chmod +x /run/ankaios/script/run_maestro.sh
COPY config /run/ankaios/config
