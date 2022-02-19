FROM rust:1.58

WORKDIR /workspace/app

RUN apt update && apt install -y rust-lldb
RUN cargo install cargo-watch

COPY . .

CMD [ "cargo", "run" ]
