# dev-runner 

WIP

Enter a list of <= 4 local node based projects and execute concurrently via the runner ui.

Motivation - none, just for fun.. 

Disclaimer - Local dev server only, shouldn't be taken to "production", the executed get_commands
are according to what the user specified, there's no way of running other sys commands nor injecting any scripts...

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