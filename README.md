# build
```
cargo build --release --target x86_64-unknown-linux-musl
docker build -t cli-mixed -f Dockerfile-cli-mixed .
```

# run
```
docker run --rm -p 9000:8080 cli-mixed hello
# or
docker run --rm -p 9000:8080 cli-mixed reg


# in another terminal
curl -XPOST "http://localhost:9000/2015-03-31/functions/function/invocations" -d '{"date": "2014-01-01"}'
```
