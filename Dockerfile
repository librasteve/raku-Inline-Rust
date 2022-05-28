FROM --platform=linux/arm64 p6steve/rakudo:ubuntu-latest-arm64 

ENV PATH=$PATH:~/.cargo/bin

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y  
