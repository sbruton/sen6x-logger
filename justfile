_default: build

build:
    cargo build --release

clean:
    cargo clean

expand:
    cargo expand --color=always | less -R
