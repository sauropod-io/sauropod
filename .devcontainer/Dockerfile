FROM ubuntu:24.04

RUN export DEBIAN_FRONTEND=noninteractive && \
    apt-get update && apt-get install -y \
    bash-completion \
    clang \
    clang-format \
    cmake \
    curl \
    git \
    git-lfs \
    gnupg \
    libclang1 \
    libssl-dev \
    lld \
    lsb-release \
    ninja-build \
    nodejs \
    npm \
    pkg-config \
    protobuf-compiler \
    python-is-python3 \
    python3 \
    python3-pip \
    python3-setuptools \
    python3-wheel \
    ripgrep \
    rustup \
    sccache \
    sudo \
    unzip \
    wget \
    zip \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

RUN echo "ubuntu ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/ubuntu

RUN mkdir -p /opt/docker_build
COPY platform_specific_install.py /opt/docker_build/platform_specific_install
RUN chmod +x /opt/docker_build/platform_specific_install

USER ubuntu

RUN rustup default stable && \
    rustup component add rust-analyzer

RUN pip install --break-system-packages pre-commit==4.2.0

ARG VALE_VERSION=3.10.0
RUN mkdir -p $HOME/.local/bin && \
    /opt/docker_build/platform_specific_install \
    --x86_64 "https://github.com/errata-ai/vale/releases/download/v${VALE_VERSION}/vale_${VALE_VERSION}_Linux_64-bit.tar.gz" \
    --aarch64 "https://github.com/errata-ai/vale/releases/download/v${VALE_VERSION}/vale_${VALE_VERSION}_Linux_arm64.tar.gz" | \
    tar -xzf - -C $HOME/.local/bin vale

COPY config.toml /home/ubuntu/.cargo/config.toml
RUN sudo chown -R ubuntu:ubuntu /home/ubuntu/.cargo

RUN echo 'export PATH=$HOME/.cargo/bin:$PATH' >> ~/.bashrc
