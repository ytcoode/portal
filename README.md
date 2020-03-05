# portal
Exposes local servers behind NATs and firewalls to the Internet

[中文文档](README_zh.md)

## Getting Started

* Download the pre-built binary suitable for your operating system from the [release page][1] (or build from source if you're familar with Rust)

* Run the following command to open a portal tunnel

```sh
$ portal open 80
Forwarding: portals.rs:39755 -> 127.0.0.1:80
```

* Now visit the address `portals.rs:39755` and the connections will be forwarded to `127.0.0.1:80`

## More Examples

```sh
$ portal open 8888 192.168.0.1:3306 # Opens multiple tunnel
Forwarding: portals.rs:41657 -> 192.168.0.1:3306
Forwarding: portals.rs:38913 -> 127.0.0.1:8888

$ portal open 8000:8000             # Specifies the remote port (may fail if it's in use)
Forwarding: portals.rs:8000 -> 127.0.0.1:8000
```

For more details, run `portal help`

[1]: https://github.com/wangyuntao/portal/releases
