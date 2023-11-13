FROM ghcr.io/eclipse-ankaios/app-ankaios-dev:latest as dev

RUN apt-get update && apt-get -y install \
    git \
    # Add your development tools here ...
    # Cleanup
    && \
    rm -rf /var/lib/apt/lists/* \
    && curl -L https://github.com/mikefarah/yq/releases/latest/download/yq_linux_amd64 -o /usr/bin/yq \
    && chmod +x /usr/bin/yq

# Trust all repositories in this container
RUN git config --global --add safe.directory '*' \
    && echo "if [ -f /etc/bash_completion ] && ! shopt -oq posix; then\n . /etc/bash_completion \nfi" >> /root/.bashrc \
    && echo 'export PATH=$PATH:/workspaces/app/scripts' >> /root/.bashrc

# prod stage, please do not change it
FROM ghcr.io/eclipse-ankaios/app-ankaios-dev:latest
COPY scripts/run_ankaios.sh /run/ankaios/script/run_ankaios.sh
RUN chmod +x /run/ankaios/script/run_ankaios.sh
COPY config/startupState.yaml /run/ankaios/config/startupState.yaml
CMD ["/run/ankaios/script/run_ankaios.sh", "2>&1"]
