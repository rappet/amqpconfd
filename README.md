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

## Testing

```shell
# start RabbitMQ container
docker run -d --hostname my-rabbit --name some-rabbit -p 5672:5672 rabbitmq:3
# run amqpconfd
cargo run -- -c testfiles/test.toml
# add JSON to queue (password is guest)
amqpcli send -m '{"foo": "uff"}' localhost 5672 '' foobar -u guest
```