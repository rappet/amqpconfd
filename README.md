![GitHub](https://img.shields.io/github/license/rappet/amqpconfd)

---

# AMQP Config Deamon

# Work in progress

This deamon listens to an AMQP topic for JSON objects,
templates a config with them and optinally runs a specified command after that.

## Build

```shell
$ cargo build --release
cp target/release/amqpconfd <destination>
```