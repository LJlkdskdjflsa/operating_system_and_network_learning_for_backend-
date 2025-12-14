# 1.2 Linux Environment: Understanding Your Programs from a System Perspective

## Section Goals

> Learn to use Linux tools to observe program system behavior, building the "code ↔ OS" mental connection

After completing this section, you will be able to:
- Use basic commands to check system status
- Use `strace` to trace program system calls
- Understand the `/proc` virtual filesystem
- Understand what your program is doing from a "system level"

---

## 1. Why Do Backend Engineers Need to Know Linux?

Your backend services will ultimately run on Linux. When problems occur:

```
"Why is the service slow?"     → Use top/htop to check CPU, memory
"Why can't connections be made?" → Use ss/netstat to check socket status
"Why is file reading so slow?" → Use strace to see which syscall is blocking
"Why is it OOM?"               → Use /proc/[pid]/status to check memory usage
```

**Understanding Linux = Ability to diagnose problems = Valuable skill**

---

## 2. Basic Commands Overview

### Process Related

| Command | Purpose | Common Parameters |
|---------|---------|-------------------|
| `ps` | View processes | `ps aux` to see all processes |
| `top` | Real-time monitoring | Press `1` to see each CPU, `M` to sort by memory |
| `htop` | Better version of top | Press `F5` to see tree structure |
| `kill` | Send signals | `kill -9 PID` to force terminate |

### File Related

| Command | Purpose | Example |
|---------|---------|---------|
| `ls` | List files | `ls -la` for detailed info |
| `cat` | Display file | `cat file.txt` |
| `less` | Paginated view | `less big_file.log` |
| `grep` | Search text | `grep "error" log.txt` |
| `find` | Search files | `find . -name "*.rs"` |

### Network Related (covered in detail in next chapter)

| Command | Purpose | Example |
|---------|---------|---------|
| `ss` | View sockets | `ss -tulpn` |
| `netstat` | Old version of ss | `netstat -an` |
| `curl` | HTTP requests | `curl http://localhost:8080` |

---

## 3. Understanding Process and Thread

### Viewing Processes with ps

```bash
# View all processes
$ ps aux
USER    PID  %CPU %MEM    VSZ   RSS TTY   STAT START   TIME COMMAND
root      1   0.0  0.1 168000 12000 ?     Ss   10:00   0:01 /sbin/init
user   1234   2.0  0.5 500000 50000 pts/0 Sl+  10:05   0:10 ./my_server

# Column meanings:
# PID   - Process ID
# %CPU  - CPU usage percentage
# %MEM  - Memory usage percentage
# VSZ   - Virtual memory size (KB)
# RSS   - Actual physical memory used (KB)
# STAT  - Status (S=sleeping, R=running, Z=zombie)
```

### Real-time Monitoring with htop

```
  CPU[||||||||                    25.0%]   Tasks: 89, 320 thr
  Mem[||||||||||||||||||      2.0G/8.0G]   Load average: 1.20 0.80 0.60
  Swp[                          0K/2.0G]   Uptime: 5 days, 03:24:12

    PID USER      PRI  NI  VIRT   RES   SHR S CPU%  MEM%   TIME+  Command
   1234 user       20   0  500M   50M   10M S  2.0   0.6   0:10.5 ./my_server
   1235 user       20   0  500M   10M    5M S  0.5   0.1   0:02.1 └─ worker_1
   1236 user       20   0  500M   10M    5M S  0.5   0.1   0:02.0 └─ worker_2
```

### Key Concepts

```
┌─────────────────────────────────────────────────────────────┐
│                        Process                               │
│  - Independent memory space                                  │
│  - Independent file descriptor table                         │
│  - Has its own PID                                          │
│                                                              │
│    ┌─────────┐  ┌─────────┐  ┌─────────┐                    │
│    │ Thread 1│  │ Thread 2│  │ Thread 3│                    │
│    │ (main)  │  │(worker) │  │(worker) │                    │
│    └─────────┘  └─────────┘  └─────────┘                    │
│         │            │            │                          │
│         └────────────┼────────────┘                          │
│                      │                                       │
│              Share the same memory                           │
│              Share the same fd table                         │
└─────────────────────────────────────────────────────────────┘
```

---

## 4. System Call

### What is a System Call?

Your program cannot directly operate hardware. To do anything "real" (read files, open network connections, allocate memory), you must request the OS for help:

```
┌─────────────────┐
│   Your Program  │  User Space
│  (Rust program) │
└────────┬────────┘
         │ System Call
         ▼
┌─────────────────┐
│   Linux Kernel  │  Kernel Space
│  (Operating     │
│   System)       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│    Hardware     │
│ (CPU, Disk, NIC)│
└─────────────────┘
```

### Common System Calls

| System Call | Purpose | Rust Equivalent |
|-------------|---------|-----------------|
| `open` | Open file | `File::open()` |
| `read` | Read data | `file.read()` |
| `write` | Write data | `file.write()` / `println!()` |
| `close` | Close file | Automatic (Drop) |
| `socket` | Create socket | `TcpListener::bind()` |
| `accept` | Accept connection | `listener.accept()` |
| `mmap` | Memory mapping | `Vec` memory allocation |
| `nanosleep` | Sleep | `thread::sleep()` |

### Observing with strace

```bash
# Trace all system calls of a program
$ strace ./my_program

# Example output:
execve("./my_program", ...) = 0
mmap(NULL, 4096, ...) = 0x7f...          # Allocate memory
open("test.txt", O_RDONLY) = 3           # Open file, returns fd=3
read(3, "Hello World\n", 4096) = 12      # Read 12 bytes
write(1, "Hello World\n", 12) = 12       # Write to stdout (fd=1)
close(3) = 0                             # Close file
exit_group(0) = ?                        # Program exits
```

---

## 5. File Descriptor (fd)

### What is an fd?

An fd is an integer representing an open "resource":
- File
- Socket connection
- Pipe
- Even `/dev/null`

### Default fds

| fd | Name | Purpose |
|----|------|---------|
| 0 | stdin | Standard input |
| 1 | stdout | Standard output |
| 2 | stderr | Standard error |

```rust
// println! ultimately becomes write(1, ...)
println!("Hello");  // → write(1, "Hello\n", 6)

// eprintln! writes to stderr
eprintln!("Error"); // → write(2, "Error\n", 6)
```

### Viewing fds with lsof

```bash
# View all fds opened by PID 1234
$ lsof -p 1234

COMMAND  PID USER   FD   TYPE DEVICE SIZE/OFF NODE NAME
my_prog 1234 user  cwd    DIR    8,1     4096  123 /home/user
my_prog 1234 user  txt    REG    8,1   100000  456 /home/user/my_prog
my_prog 1234 user    0u   CHR  136,0      0t0    3 /dev/pts/0
my_prog 1234 user    1u   CHR  136,0      0t0    3 /dev/pts/0
my_prog 1234 user    2u   CHR  136,0      0t0    3 /dev/pts/0
my_prog 1234 user    3r   REG    8,1    12345  789 /home/user/data.txt
my_prog 1234 user    4u  IPv4  12345      0t0  TCP *:8080 (LISTEN)
```

---

## 6. /proc Virtual Filesystem

### What is /proc?

Linux exposes kernel information as "files". They're not real files, but virtual interfaces.

### Commonly Used /proc Paths

```bash
# System information
/proc/cpuinfo      # CPU information
/proc/meminfo      # Memory information
/proc/loadavg      # System load

# Process-specific information (replace [pid] with actual PID)
/proc/[pid]/status    # Process status (name, memory usage, etc.)
/proc/[pid]/cmdline   # Command line at startup
/proc/[pid]/fd/       # Open file descriptors
/proc/[pid]/maps      # Memory mappings
```

### Practical Examples

```bash
# View current shell's PID
$ echo $$
1234

# View this process's status
$ cat /proc/1234/status
Name:   bash
State:  S (sleeping)
Pid:    1234
PPid:   1000
Threads: 1
VmSize: 12000 kB
VmRSS:  4000 kB
...

# View this process's open fds
$ ls -la /proc/1234/fd/
lrwx------ 1 user user 64 Jan  1 10:00 0 -> /dev/pts/0
lrwx------ 1 user user 64 Jan  1 10:00 1 -> /dev/pts/0
lrwx------ 1 user user 64 Jan  1 10:00 2 -> /dev/pts/0
```

---

## 7. Practical Combinations

### Find processes using most CPU

```bash
ps aux --sort=-%cpu | head -10
```

### Find processes using most memory

```bash
ps aux --sort=-%mem | head -10
```

### Trace specific syscalls (only open/read/write)

```bash
strace -e trace=open,read,write ./my_program
```

### Trace a running program

```bash
# Attach to PID 1234
strace -p 1234
```

### View program's network connections

```bash
# Need to know the PID
ss -tulpn | grep 1234
```

---

## Summary: Building a System Perspective

```
┌────────────────────────────────────────────────────────────────┐
│                     Your Mental Model                           │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│   Code Level                      System Level                 │
│   ──────────                      ────────────                 │
│   File::open("x.txt")    ──→   open("x.txt") → fd=3           │
│   file.read(&mut buf)    ──→   read(3, buf, size) → N bytes   │
│   println!("Hello")      ──→   write(1, "Hello", 5)           │
│   drop(file)             ──→   close(3)                       │
│                                                                │
│   thread::spawn(...)     ──→   clone() → new thread           │
│   TcpListener::bind()    ──→   socket() + bind() + listen()   │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

When you can see both levels simultaneously, debugging and performance tuning become much easier.

---

## Next Steps

After completing the theory reading, proceed to hands-on practice:
1. **Lab 3**: Use strace to observe your Rust program
2. **Lab 4**: Implement a mini ps that reads /proc
