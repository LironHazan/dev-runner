# dev-runner 

Enter a list of <= 4 local node based projects and execute concurrently via the runner ui.

#### Runner UI 
Simple React based interface. 

#### Runner API
Local actix-web server.


Development: 

server on watch mode: ``cargo watch -x run``

frontend dev-server: ``yarn dev``

Build image + start the container: 
```shell
docker build -t rust-dev-runner .

docker run -d -p 8080:8080 rust-dev-runner
```