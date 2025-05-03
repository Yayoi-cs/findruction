# findruction
findruction is a fast and loose instruction finder.
findruction is used for find arbitrary instructions from a large binary such as `swapgs` in `vmlinux`.

## usage
```shell
$ findruction --help
Usage: findruction [OPTIONS] --file <FILE> --asm <ASM>

Options:
  -f, --file <FILE>  
  -a, --asm <ASM>    
  -n, --no-disass    
  -h, --help         Print help
  -V, --version      Print version
```

## install
```shell
git clone https://github.com/Yayoi-cs/findruction
cd findruction
cargo build --release
echo "export PATH=$PATH:$(pwd)/target/release/" >> ~/.bashrc
```

## example

```shell
$ findruction -f vmlinux -a "swapgs;"
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

```shell
$ findruction -f vmlinux -a "pop rbp; ret;" -n
[+] pop rbp
[+] ret
[*] Generated Machine Code: 5dc3
[*] Finish process in 48.35ms
[+] Instr #1/21 Offset: 0x435b4f Vaddr: 0xffffffff81235b4f
[+] Instr #2/21 Offset: 0x49813f Vaddr: 0xffffffff8129813f
[+] Instr #3/21 Offset: 0x4a0ff3 Vaddr: 0xffffffff812a0ff3
[+] Instr #4/21 Offset: 0x4a1153 Vaddr: 0xffffffff812a1153
[+] Instr #5/21 Offset: 0x6eefb3 Vaddr: 0xffffffff814eefb3
[+] Instr #6/21 Offset: 0x6ef2cb Vaddr: 0xffffffff814ef2cb
[+] Instr #7/21 Offset: 0x6ef378 Vaddr: 0xffffffff814ef378
[+] Instr #8/21 Offset: 0x6ef420 Vaddr: 0xffffffff814ef420
[+] Instr #9/21 Offset: 0x6ef485 Vaddr: 0xffffffff814ef485
[+] Instr #10/21 Offset: 0x946b0f Vaddr: 0xffffffff81746b0f
[+] Instr #11/21 Offset: 0x9b6f3f Vaddr: 0xffffffff817b6f3f
[+] Instr #12/21 Offset: 0xc258d1 Vaddr: 0xffffffff81a258d1
[+] Instr #13/21 Offset: 0xdae9a2 Vaddr: 0xffffffff81bae9a2
[+] Instr #14/21 Offset: 0xe50b3e Vaddr: 0xffffffff81c50b3e
[+] Instr #15/21 Offset: 0xe50b56 Vaddr: 0xffffffff81c50b56
[+] Instr #16/21 Offset: 0xe510b3 Vaddr: 0xffffffff81c510b3
[+] Instr #17/21 Offset: 0xe51126 Vaddr: 0xffffffff81c51126
[+] Instr #18/21 Offset: 0x1376f3f Vaddr: 0xffffffff81f76f3f
[+] Instr #19/21 Offset: 0x1452e30 Vaddr: 0xffffffff82052e30
[+] Instr #20/21 Offset: 0x145a03c Vaddr: 0xffffffff8205a03c
[+] Instr #21/21 Offset: 0x149ae1c Vaddr: 0xffffffff8209ae1c
```