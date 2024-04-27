# 第九周汇报

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



## 2.

Arce OS 的层次化结构：

-硬件有关、系统无关的模块：大多是控制硬件的驱动，例如时钟

-硬件无关、系统无关的模块：资源分配工具箱（算法），例如内存管理、进程（线程）调度器等、页表

-系统有关的模块：例如动态内存分配、网络、驱动接口等。

框架文件架构：

```
api -- 面向用户程序提供的接口
apps -- 用户程序
crates -- 系统无关模块
modules -- 系统核心实现
doc --
platforms -- 平台设置
```

## 

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

## 3.2. 基于 AXHAL 输出 Hello World 内容

替换原有的 `rust_main`, 只保留其核心功能：

`rust_main` 是一些初始化，之后才调用用户程序然后推出，其实可以在 AXHAL 里直接完成，略过该部分。观察在 axruntime 中， `rust_main` 最后的执行流：

```rust

unsafe { main() };

#[cfg(feature = "multitask")]
axtask::exit(0);
#[cfg(not(feature = "multitask"))]
{
    debug!("main task exited: exit_code={}", 0);
    axhal::misc::terminate();
}
```

那直接留下

```
main();
axhal::misc::terminate();
```

即可。

编译运行输出结果如下：

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
Hello, world!
```

## 3.3. 加入 runtime，提供更完备的功能

阅读 `modules/axruntime/src/lib.rs:rust_main` 可以观察系统的初始化过程：

首先调用 axalloc 初始化 memory allocator，然后

- 对于 RISC-V 平台，打开三种中断（软件、外设、时钟），设置 mtimecmp (`sbi_rt::set_timer(0)`)；
- 对于 x86 平台，设置 APIC, 设置对应的中断向量和时钟信息。

如果有多任务，初始化调度器；

按需初始化网络、块和显示设备；

按需初始化其他CPU核；

初始化时钟中断的处理方式；

...

各个部件的初始化都是按需完成的。

首先复原 3.2 的变更，然后按提示编译 FIFO-Scheduler，得到如下结果。

```
[  0.061340 0 axruntime:126] Logging is enabled.
[  0.064383 0 axruntime:127] Primary CPU 0 started, dtb = 0x87000000.
[  0.066431 0 axruntime:129] Found physcial memory regions:
[  0.068564 0 axruntime:131]   [PA:0x80200000, PA:0x80209000) .text (READ | EXECUTE | RESERVED)
[  0.072448 0 axruntime:131]   [PA:0x80209000, PA:0x8020d000) .rodata (READ | RESERVED)
[  0.074021 0 axruntime:131]   [PA:0x8020d000, PA:0x80210000) .data .tdata .tbss .percpu (READ | WRITE | RESERVED)
[  0.076704 0 axruntime:131]   [PA:0x80210000, PA:0x80250000) boot stack (READ | WRITE | RESERVED)
[  0.079494 0 axruntime:131]   [PA:0x80250000, PA:0x80274000) .bss (READ | WRITE | RESERVED)
[  0.082734 0 axruntime:131]   [PA:0x80274000, PA:0x88000000) free memory (READ | WRITE | FREE)
[  0.086283 0 axruntime:131]   [PA:0xc000000, PA:0xc210000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.089007 0 axruntime:131]   [PA:0x10000000, PA:0x10001000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.090795 0 axruntime:131]   [PA:0x10001000, PA:0x10009000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.092635 0 axruntime:131]   [PA:0x30000000, PA:0x40000000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.094064 0 axruntime:131]   [PA:0x40000000, PA:0x80000000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.095842 0 axruntime:207] Initialize global memory allocator...
[  0.097237 0 axruntime:208]   use TLSF allocator.
[  0.099589 0 axruntime:149] Initialize platform devices...
[  0.101154 0 axtask::api:66] Initialize scheduling...
[  0.103459 0 axtask::api:72]   use FIFO scheduler.
[  0.104804 0 axruntime:185] Primary CPU 0 init OK.
Hello, main task!
Hello, task 0! id = ThreadId(4)
Hello, task 1! id = ThreadId(5)
Hello, task 2! id = ThreadId(6)
Hello, task 3! id = ThreadId(7)
Hello, task 4! id = ThreadId(8)
Hello, task 5! id = ThreadId(9)
Hello, task 6! id = ThreadId(10)
Hello, task 7! id = ThreadId(11)
Hello, task 8! id = ThreadId(12)
Hello, task 9! id = ThreadId(13)
Task yielding tests run OK!
[  0.117239 0:2 axhal::platform::riscv64_qemu_virt::misc:3] Shutting down...
```

观察其输出信息：物理内存段有内核的数据、代码段和 BOOT  信息段，以及五个外设：(platforms/riscv64-qemu-virt.toml)

- [PA:0xc000000, PA:0xc210000)        PLIC
- [PA:0x10000000, PA:0x10001000)   UART
- [PA:0x10001000, PA:0x10009000)   VirtIO
- [PA:0x30000000, PA:0x40000000)   PCI 配置
- [PA:0x40000000, PA:0x80000000)    PCI MMIO （PMIO, 32b MMIO, 64b MMIO)

平台无关：具有普遍性的模块（功能），只要固定好条件，即可放在不同地方执行；

平台有关：需要结合应用场景分析的地方。

调整 Hello World 的实现，使之动用 memory allocator:

```
#[cfg(feature = "axstd")]
use axstd::println;
use axstd::string::String;
#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let prompt = String::from("Greet from ArceOS");
    println!("Hello, world: {}", prompt);
}

```



得到

```
HELLO FROM SBI

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

Hello, world: Greet from ArceOS
```



## 4. 协程

yield 的输出：

```
Hello, main task!
Hello, task 0! id = ThreadId(4)
Hello, task 1! id = ThreadId(5)
Hello, task 2! id = ThreadId(6)
Hello, task 3! id = ThreadId(7)
Hello, task 4! id = ThreadId(8)
Hello, task 5! id = ThreadId(9)
Hello, task 6! id = ThreadId(10)
Hello, task 7! id = ThreadId(11)
Hello, task 8! id = ThreadId(12)
Hello, task 9! id = ThreadId(13)
```



## 思考

ArceOS 通过 axhal 的 boot 作为入口：

利用 Linker Script 指定 `.text.boot` 段作为入口。



https://embeddedinn.com/articles/tutorial/RISCV-Uncovering-the-Mysteries-of-Linux-Boot-on-RISC-V-QEMU-Machines/

![The RISC-V Linux Boot Process](https://www.embeddedinn.com/images/posts/rvLinuxQemuBoot/bootAnimation.gif)

RISC-V 之下，设备的启动过程如下：

- 固件驱动（zsbl）：QEMU 进入 0x1000, 设置好 mhartid 和 DTB 信息，
- 转入 Bootloader. Bootloader 进行必要的初始化，例如部分委托中断处理功能、设置自身的中断处理入口等（second stage bootloader），
- 调用 sret （事前指定系统入口） 进入内核。

相比之下，x86 以下列方式驱动：

- ROM Stage: 检测 RAM 类型，RAM 自检；
- RAM Stage：硬件自检，固件存储映射
- MBR(master boot record): 引导其他 Boot record 加载
- Bootloader 进入 Protected mode, 加载内核

https://wiki.osdev.org/System_Initialization_(x86)