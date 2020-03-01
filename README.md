# portal
Exposes local servers behind NATs and firewalls to the Internet


## Get Started

1. Download the pre-built binary suitable for your operating system from the release page (or build from source if you're familar with Rust).

2. Run the following command to open a portal tunnel.

```sh
$ portal open 80
Forwarding: p.ytcode.io:39755 -> 127.0.0.1:80
```

3. Now visit the address `p.ytcode.io:39755` and the connections will be forwarded to `127.0.0.1:80`.
