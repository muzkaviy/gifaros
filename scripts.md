>> rustup target add aarch64-unknown-none-softfloat
>> rustup default nightly
>> rustup component add rust-src --toolchain nightly
>> rustup component add llvm-tools-preview --toolchain nightly
>> cargo install cargo-binutils
>> rustup component add llvm-tools-preview

Derleme
>> cargo build --target aarch64-unknown-none-softfloat
>> cargo objcopy --target aarch64-unknown-none-softfloat -- -O binary gifaros.bin