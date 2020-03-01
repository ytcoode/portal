# portal
Exposes local servers behind NATs and firewalls to the Internet

### Get Started

* Download the pre-built binary suitable for your operating system from the [release page][1] (or build from source if you're familar with Rust).

* Run the following command to open a portal tunnel.

```sh
$ portal open 80
Forwarding: p.ytcode.io:39755 -> 127.0.0.1:80
```

* Now visit the address `p.ytcode.io:39755` and the connections will be forwarded to `127.0.0.1:80`.

### More Examples

```sh

$ portal open 8888 192.168.0.1:3306
Forwarding: p.ytcode.io:41657 -> 192.168.0.1:3306
Forwarding: p.ytcode.io:38913 -> 127.0.0.1:8888
```

[1]: https://github.com/wangyuntao/portal/releases
