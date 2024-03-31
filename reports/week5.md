# 操作系统实验 第五周汇报

岳章乔 2021010706 [yuezq21@mails.tsinghua.edu.cn](mailto:yuezq21@mails.tsinghua.edu.cn) 2024.3.31

## 本周进展

•主要在找 Bootloader

•周五领了电脑，昨天开始把驱动程序往新电脑上迁移（观察驱动在平台上的运行效果），正在配置环境

•在查看教程，制作自定义 ISO 文件

•大致阅读了驱动程序运行的基本原理，正在看其和 Linux 系统的交互方式

重点在剖析 `vpu_driver` ，也就是内核部分的代码

`vpu_driver/source/os_interface` 里，能看到部分需要的系统调用：

```c++
class OsInterface {
  public:
    virtual ~OsInterface() = default;

    virtual int osiOpen(const char *pathname, int flags, mode_t mode) = 0;
    virtual int osiClose(int fildes) = 0;
    virtual int osiFcntl(int fd, int cmd) = 0;
    virtual int osiIoctl(int fd, unsigned long request, void *arg) = 0;

    virtual size_t osiGetSystemPageSize() = 0;

    virtual void *osiMmap(void *addr, size_t size, int prot, int flags, int fd, off_t offset) = 0;
    virtual int osiMunmap(void *addr, size_t size) = 0;
};
```

找到一个实现方式，面向 POSIX Linux:

`os_interface_imp.cpp`

```c++
OsInterfaceImp &OsInterfaceImp::getInstance() {
    static OsInterfaceImp instance;
    return instance;
}

int OsInterfaceImp::osiOpen(const char *pathname, int flags, mode_t mode) {
    struct stat fstatInfo = {};
    int fd;

    LOG_V("Trying to open file '%s'.", pathname);
    if ((fd = open(pathname, flags, mode)) == -1) {
        LOG_V("Failed to open file '%s'.", pathname);
        return -1;
    }

    if (fstat(fd, &fstatInfo) != 0) {
        LOG_E("Failed to get file information. Closing.");
        close(fd);
        return -1;
    }

    if (!S_ISCHR(fstatInfo.st_mode)) {
        LOG_E("Open file is not the expected device file. Closing.");
        close(fd);
        return -1;
    }

    LOG_I("Returning file descriptor %d", fd);
    return fd;
}

int OsInterfaceImp::osiClose(int fd) {
    return close(fd);
}

int OsInterfaceImp::osiFcntl(int fd, int cmd) {
    return fcntl(fd, cmd);
}

int OsInterfaceImp::osiIoctl(int fd, unsigned long request, void *args) {
    return ioctl(fd, request, args);
}

size_t OsInterfaceImp::osiGetSystemPageSize() {
    return safe_cast<size_t>(sysconf(_SC_PAGESIZE));
}

void *OsInterfaceImp::osiMmap(void *addr, size_t size, int prot, int flags, int fd, off_t offset) {
    return mmap(addr, size, prot, flags, fd, offset);
}

int OsInterfaceImp::osiMunmap(void *addr, size_t size) {
    return munmap(addr, size);
}
```

TODO: 将之移植到 ACREOS 上。

vpu 有下列模块

command - VPU 控制指令的高级抽象

device：硬件的底层实现方式？？ -- 需要进一步分析

memory： NPU 存储，以缓冲区的形式表示，提供控制的接口

os_interface: 上文

## 下周展望

•配置好基于linux驱动程序（安装 ubuntu），直接观察驱动的运行效果

•配置好 Bootloader，使得 ARCEOS 可以直接在平台上运行



- 先封装好内核态部分与 Linux 的交互方式，然后再看如何更好的融合用户态部分