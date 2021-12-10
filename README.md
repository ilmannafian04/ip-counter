# Docker Build - Print IP

## Running the application

You can run this application natively or using docker.

### Native

This application is built using Rust, so in order to run it, the application needed to be compiled first.

```shell
$ cargo build --release
```

The compiled binary will be inside `target` folder.

```shell
$ ./target/debug/ip-counter
```

To run the application in developer mode you can use cargo:

```shell
$ cargo run
```

### Docker

From scrat, you need to build the image first

```shell
$ docker build -t ip-counter:latest .
```

After successfully building the image, you can run it using:

```shell
$ docker run -it --rm -p 80:80 ip-counter:latest
```

## Scaling the service

Right now, the state of the counter is store in-memory using hash map. Of course we can't scale the application right away. First, the data needed to be separated from the application, for example it can be put inside a database, or a cache.

Now that the data has been separated, the application itself now become stateless. That means that we can have as many instance of the application as we need. At this point, it would be better to use an orchestration tool to easily manage how many instance we want, and to help scale up/down the application.
