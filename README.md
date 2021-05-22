# Veil
Limits filesystem visibility and access for any *dynamically linked* program. Uses OpenBSD's `unveil(2)`.

## Building
```sh
pkg_add rust
cargo build --release
```

## Usage
```sh
~ $ cat program.rules
/bin rx
/etc r
/lib r
/var/log w
~ $ VEIL_RULES_PATH=program.rules LD_PRELOAD=/path/to/libveil.so /path/to/program arg1 arg2
```
