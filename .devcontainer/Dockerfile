FROM debian:bullseye-slim

ARG USERNAME=nonroot
ARG USER_UID=1000
ARG USER_GID=1000

WORKDIR /

RUN : \
    && groupadd --gid "${USER_GID}" "${USERNAME}" \
    && useradd --create-home --uid "${USER_UID}" --gid "${USER_GID}" --shell /bin/bash "${USERNAME}" \
    && mkdir -p /etc/sudoers.d/ \
    && echo "$USERNAME" ALL=\(root\) NOPASSWD:ALL > "/etc/sudoers.d/$USERNAME" \
    && chmod 0440 "/etc/sudoers.d/$USERNAME"

USER root
RUN DEBIAN_FRONTEND=noninteractive && apt-get update \
    && apt-get -y --no-install-recommends install \
    apt-transport-https \
    build-essential \
    ca-certificates \
    clang \
    curl \
    gcc \
    git \
    gnupg \
    less \
    locales \
    lsb-release \
    sudo \
    unzip \
    wget \
    zip \
    && apt-get -y clean \
    && rm -rf /var/lib/apt/lists/* \
    && sed -i -e 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen \
    && locale-gen

USER nonroot
WORKDIR /home/nonroot
