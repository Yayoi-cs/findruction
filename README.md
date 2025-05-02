# findruction
findruction is a fast and loose instruction finder.
findruction is used for find arbitrary instructions from a large binary such as `swapgs` in `vmlinux`.

## usage
```shell
findruction <elf> <assembly>
```

## install
```shell
git clone https://github.com/Yayoi-cs/findruction
cd findruction
cargo build --release
echo "export PATH=$PATH:$(pwd)/target/release/"
```

## example

```shell
$ findruction vmlinux "swapgs;"
[+] swapgs
[*] Generated Machine Code: 0f01f8
[*] Finish process in 48.80ms
[+] Instr #1/19 Offset: 0x9e3400 Vaddr: 0xffffffff817e3400
    0xffffffff817e3400: swapgs
    0xffffffff817e3403: rdgsbase rax
    0xffffffff817e3408: swapgs
    0xffffffff817e340b: jmp 0FFFFFFFF817F32A0h
    └-->0xffffffff817f32a0: ret
        0xffffffff817f32a1: nop
        0xffffffff817f32a2: nop
    0xffffffff817e3410: nop
    0xffffffff817e3411: nop
    0xffffffff817e3412: nop

[+] Instr #2/19 Offset: 0x9e3408 Vaddr: 0xffffffff817e3408
    0xffffffff817e3408: swapgs
    0xffffffff817e340b: jmp 0FFFFFFFF817F32A0h
    └-->0xffffffff817f32a0: ret
        0xffffffff817f32a1: nop
        0xffffffff817f32a2: nop
    0xffffffff817e3410: nop
    0xffffffff817e3411: nop
    0xffffffff817e3412: nop
    0xffffffff817e3413: nop
    0xffffffff817e3414: nop
```