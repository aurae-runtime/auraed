# Auraed

A runtime daemon written in Rust. 

 - [X] Designed to run as pid 1
 - [X] mTLS backed gRPC API over unix domain socket
 - [X] Run executables
 - [ ] Run containers
 - [ ] Run virtual machines (as a hypervisor)
 - [ ] Schedule workloads
 - [ ] Piping for `stdout` and `stderr` from scheduled workloads
 - [ ] Mapping network devices to workloads
 - [ ] Piping for kernel logs
 - [ ] Piping for syslog
 - [ ] Piping for kernel events
 - [ ] Native eBPF support
 - [X] Built on glibc

## Build from source

We suggest using the [environment](https://github.com/aurae-runtime/environment) repository for building.

Otherwise you will need to check out the Aurae API in the following directory structure.

```bash
.
├── api
│   └── v1
│       └── *.proto
└── auraed
    ├── Cargo.toml
    └── Makefile
```

Navigate to the `/auraed` directory and build using Make

```bash
make install
```

or using Cargo directly

```bash
cargo clippy
cargo install --debug --path .
```


