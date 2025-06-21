# Rust development environment for devcontainer
FROM rust:1.87.0-slim-bullseye

ENV LANG=ja_JP.UTF-8
ENV LANGUAGE=ja_JP:ja
ENV LC_ALL=ja_JP.UTF-8
ENV MISE_GLOBAL_CONFIG_FILE=/workspace/mise.toml

# Install packages required for development
RUN apt-get update && apt-get install -y \
    # Locales
    locales \
    # Basic tools
    git \
    curl \
    wget \
    vim \
    nano \
    zsh \
    unzip \
    # Build tools
    build-essential \
    pkg-config \
    libssl-dev \
    # Debugging tools
    gdb \
    lldb \
    # Network tools
    net-tools \
    # Generate Japanese locale
    && echo "ja_JP.UTF-8 UTF-8" >> /etc/locale.gen \
    && locale-gen \
    # Clean up
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Optimize Rust settings
RUN rustup component add rustfmt clippy llvm-tools-preview
RUN cargo install cargo-llvm-cov cargo-binstall

# Install Mise
COPY mise.toml /workspace/mise.toml
RUN cargo binstall mise
RUN mise install node

# Oh My Zsh
RUN sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# Set working directory
WORKDIR /workspace

CMD ["/bin/zsh"]
