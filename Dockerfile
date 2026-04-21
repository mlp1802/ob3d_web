FROM rust:latest

RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    clang \
    mold \
    wget \
    libudev-dev \
    libasound2-dev \
    libgl1-mesa-dev \
    libglu1-mesa-dev \
    libglfw3 \
    libx11-dev \
    libxrandr-dev \
    libxi-dev \
    libxinerama-dev \
    libxcursor-dev

# mujoco
RUN wget https://github.com/google-deepmind/mujoco/releases/download/3.3.7/mujoco-3.3.7-linux-x86_64.tar.gz \
 && tar -xzf mujoco-3.3.7-linux-x86_64.tar.gz \
 && mv mujoco-3.3.7 /usr/local/mujoco

ENV MUJOCO_DYNAMIC_LINK_DIR=/usr/local/mujoco/lib
ENV LD_LIBRARY_PATH=/usr/local/mujoco/lib

# 👇 important
WORKDIR /web

# 👇 hot reload
RUN cargo install cargo-watch --locked

# 👇 cache builds
ENV CARGO_TARGET_DIR=/web/target
