# findruction
findruction is a fast and loose instruction finder.
findruction is used for find arbitrary instructions from a large binary such as `swapgs` in `vmlinux`.

## usage
```shell
findruction <elf> <assembly>
findruction vmlinux "pop rdi; ret;"
findruction vmlinux "swapgs;"
```

## install
```shell
git clone https://github.com/Yayoi-cs/findruction
cd findruction
cargo build --release
echo "export PATH=$PATH:$(pwd)/target/release/"
```
