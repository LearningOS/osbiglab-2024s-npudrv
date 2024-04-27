## 1.5

```
yuezq21@LAPTOP-92IKODO2:/mnt/d/github/arceos (main)$ make ARCH=riscv64 A=apps/helloworld run
    Building App: helloworld, Arch: riscv64, Platform: riscv64-qemu-virt, App type: rust
cargo build --target riscv64gc-unknown-none-elf --target-dir /mnt/d/github/arceos/target --release  --manifest-path apps/helloworld/Cargo.toml --features "axstd/log-level-warn"
   Compiling riscv v0.10.1
   Compiling axconfig v0.1.0 (/mnt/d/github/arceos/modules/axconfig)
   Compiling axerrno v0.1.0 (/mnt/d/github/arceos/crates/axerrno)
   Compiling axio v0.1.0 (/mnt/d/github/arceos/crates/axio)
   Compiling axhal v0.1.0 (/mnt/d/github/arceos/modules/axhal)
   Compiling axruntime v0.1.0 (/mnt/d/github/arceos/modules/axruntime)
   Compiling axfeat v0.1.0 (/mnt/d/github/arceos/api/axfeat)
   Compiling arceos_api v0.1.0 (/mnt/d/github/arceos/api/arceos_api)
   Compiling axstd v0.1.0 (/mnt/d/github/arceos/ulib/axstd)
   Compiling arceos-helloworld v0.1.0 (/mnt/d/github/arceos/apps/helloworld)
    Finished release [optimized] target(s) in 10.03s
rust-objcopy --binary-architecture=riscv64 apps/helloworld/helloworld_riscv64-qemu-virt.elf --strip-all -O binary apps/helloworld/helloworld_riscv64-qemu-virt.bin
    Running on qemu...
qemu-system-riscv64 -m 128M -smp 1 -machine virt -bios default -kernel apps/helloworld/helloworld_riscv64-qemu-virt.bin -nographic

OpenSBI v1.0
   ____                    _____ ____ _____
  / __ \                  / ____|  _ \_   _|
 | |  | |_ __   ___ _ __ | (___ | |_) || |
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |
 | |__| | |_) |  __/ | | |____) | |_) || |_
  \____/| .__/ \___|_| |_|_____/|____/_____|
        | |
        |_|

Platform Name             : riscv-virtio,qemu
Platform Features         : medeleg
Platform HART Count       : 1
Platform IPI Device       : aclint-mswi
Platform Timer Device     : aclint-mtimer @ 10000000Hz
Platform Console Device   : uart8250
Platform HSM Device       : ---
Platform Reboot Device    : sifive_test
Platform Shutdown Device  : sifive_test
Firmware Base             : 0x80000000
Firmware Size             : 252 KB
Runtime SBI Version       : 0.3

Domain0 Name              : root
Domain0 Boot HART         : 0
Domain0 HARTs             : 0*
Domain0 Region00          : 0x0000000002000000-0x000000000200ffff (I)
Domain0 Region01          : 0x0000000080000000-0x000000008003ffff ()
Domain0 Region02          : 0x0000000000000000-0xffffffffffffffff (R,W,X)
Domain0 Next Address      : 0x0000000080200000
Domain0 Next Arg1         : 0x0000000087000000
Domain0 Next Mode         : S-mode
Domain0 SysReset          : yes

Boot HART ID              : 0
Boot HART Domain          : root
Boot HART ISA             : rv64imafdcsuh
Boot HART Features        : scounteren,mcounteren,time
Boot HART PMP Count       : 16
Boot HART PMP Granularity : 4
Boot HART PMP Address Bits: 54
Boot HART MHPM Count      : 0
Boot HART MIDELEG         : 0x0000000000001666
Boot HART MEDELEG         : 0x0000000000f0b509

       d8888                            .d88888b.   .d8888b.
      d88888                           d88P" "Y88b d88P  Y88b
     d88P888                           888     888 Y88b.
    d88P 888 888d888  .d8888b  .d88b.  888     888  "Y888b.
   d88P  888 888P"   d88P"    d8P  Y8b 888     888     "Y88b.
  d88P   888 888     888      88888888 888     888       "888
 d8888888888 888     Y88b.    Y8b.     Y88b. .d88P Y88b  d88P
d88P     888 888      "Y8888P  "Y8888   "Y88888P"   "Y8888P"

arch = riscv64
platform = riscv64-qemu-virt
target = riscv64gc-unknown-none-elf
smp = 1
build_mode = release
log_level = warn

Hello, world!
```

## 3.1. 系统引导输出

修改 `modules/axhal/src/platform/riscv64_qemu_virt/boot.rs`

加入实现 `console_putchar()`

调整内联汇编的调用过程为如下：

```rust
 core::arch::asm!("
 #...
 call    {init_boot_page_table}
 call    {console_putchar}
 call    {init_mmu}
 #...", /**/ console_putchar = sym console_putchar, /**/)
```

以及其配套实现

```rust
unsafe fn console_putchar() {
    putchar(0x0a);
    
    putchar(0x48); // H
    putchar(0x45); // E
    putchar(0x4C); // L
    putchar(0x4C); // L
    putchar(0x4F); // O
    
    putchar(0x20); // ' '

    putchar(0x46); // F
    putchar(0x52); // R
    putchar(0x4F); // O
    putchar(0x4D); // M
    
    putchar(0x20); // ' '

    putchar(0x53); // S
    putchar(0x42); // B
    putchar(0x49); // I
}
```

编译得到

```
Boot HART ID              : 0
Boot HART Domain          : root
Boot HART ISA             : rv64imafdcsuh
Boot HART Features        : scounteren,mcounteren,time
Boot HART PMP Count       : 16
Boot HART PMP Granularity : 4
Boot HART PMP Address Bits: 54
Boot HART MHPM Count      : 0
Boot HART MIDELEG         : 0x0000000000001666
Boot HART MEDELEG         : 0x0000000000f0b509

HELLO FROM SBI
       d8888                            .d88888b.   .d8888b.
      d88888                           d88P" "Y88b d88P  Y88b
     d88P888                           888     888 Y88b.
    d88P 888 888d888  .d8888b  .d88b.  888     888  "Y888b.
   d88P  888 888P"   d88P"    d8P  Y8b 888     888     "Y88b.
  d88P   888 888     888      88888888 888     888       "888
 d8888888888 888     Y88b.    Y8b.     Y88b. .d88P Y88b  d88P
d88P     888 888      "Y8888P  "Y8888   "Y88888P"   "Y8888P"
```

