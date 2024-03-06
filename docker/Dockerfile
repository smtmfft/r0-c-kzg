FROM rustlang/rust:nightly

# install env
RUN apt-get update && apt-get install -y \
    autoconf automake autotools-dev curl python3 python3-pip libmpc-dev libmpfr-dev libgmp-dev gawk build-essential bison flex texinfo gperf libtool patchutils bc zlib1g-dev libexpat-dev ninja-build git cmake libglib2.0-dev \
    time \
    && rm -rf /var/lib/apt/lists/*

# mapping /opt/riscv to /opt/riscv
RUN mkdir -p /opt/riscv
VOLUME /opt/riscv

# setup workspace
RUN git clone https://github.com/smtmfft/r0-c-kzg.git /app
WORKDIR /app

RUN cargo install cargo-binstall
RUN cargo binstall -y cargo-risczero
RUN cargo risczero install

# need COPY /opt/riscv toolchain, delay to CMD.
#RUN CC=gcc CC_riscv32im_risc0_zkvm_elf=/opt/riscv/bin/riscv32-unknown-elf-gcc cargo build

# mapping tmp
VOLUME /tmp

# startup command
ENV CC=gcc
ENV CC_riscv32im_risc0_zkvm_elf=/opt/riscv/bin/riscv32-unknown-elf-gcc
#ENV BONSAI_API_KEY=""
#ENV BONSAI_API_URL=""
CMD ["cargo", "run", "--release"]