# portal
portal（传送门）是一款内网穿透工具，通过它我们可以使内网服务在外网可以访问。

## 入门教程

* 根据操作系统类型，从[release][1]页下载提前构建好的可执行程序（如果你熟悉Rust的话，也可以自己构建）

* 执行如下命令，开启一个传送门隧道

```sh
$ portal open 80
Forwarding: p.ytcode.io:39755 -> 127.0.0.1:80
```

* 访问地址 `p.ytcode.io:39755`，连接会被自动转发到 `127.0.0.1:80`

## 更多示例

```sh
$ portal open 8888 192.168.0.1:3306 # 可以一次开启多个隧道，甚至是到内网的其他机器
Forwarding: p.ytcode.io:41657 -> 192.168.0.1:3306
Forwarding: p.ytcode.io:38913 -> 127.0.0.1:8888
```

有关更多细节，请执行命令 `portal help`

[1]: https://github.com/wangyuntao/portal/releases
