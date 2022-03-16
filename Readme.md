# dev-runner 

TBD

Development: 

``cargo watch -x run``

Build image + start the container: 
```shell
docker build -t rust-dev-runner .

docker run -d -p 8080:8080 rust-dev-runner
```