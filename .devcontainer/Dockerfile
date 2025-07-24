FROM ubuntu:noble

# Install dependencies for Rust and Neovim
RUN apt update && export DEBIAN_FRONTEND=noninteractive \
  && apt upgrade -y \
  && apt install -y \
    git \
    zsh \
    curl \
    unzip \
    build-essential

ARG USERNAME=scott
ARG HOME=/home/$USERNAME

RUN useradd -m -s /bin/zsh $USERNAME

# Set working directory
WORKDIR $HOME

# Switch to the non-root user
USER $USERNAME

ARG NVIM=$HOME/.neovim
RUN mkdir $NVIM
RUN curl -L https://github.com/neovim/neovim/releases/download/v0.11.3/nvim-linux-arm64.tar.gz | tar -zx -C $NVIM --strip-components=1
RUN git clone https://github.com/sleb/kickstart.nvim.git $HOME/.config/nvim

# Install Rust using rustup (recommended way)
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y --profile default

# Add Rust to the PATH for all users
ENV PATH="$HOME/.cargo/bin:${PATH}"
ENV PATH="$NVIM/bin:${PATH}"

# Print versions for debugging
RUN rustc --version && cargo --version && nvim --version

# (Optional) Set entrypoint or CMD if desired
# CMD ["nvim"]
