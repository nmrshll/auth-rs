.DEFAULT_GOAL=dev
dev: deps pg adminer migrate
	cargo watch -x run



protected:
	curl -X POST http://${API_ADDR}/protected/ -d '{}' -b token=AAAAAF3Xvg0AAAAAXdkPjQAAAAAAAAAB2dED23NwFsCtZAz59+b00GE9o29UGYfiW0ALLbY17ssfTCe+d57nInnAbJx6uFR81evjZfIpl0BSxjCkT29g3g==
login:
	curl -X POST http://${API_ADDR}/auth/ -d '{"email":"user.email@gmail.com", "password":"nopass"}'
users/check:
	curl -X POST http://${API_ADDR}/users/check -d '{"email":"user.email@gmail.com"}'
users/register:
	curl -X POST http://${API_ADDR}/users/register -d '{"email":"user.email@gmail.com","password":"nopass"}'
DEFAULT_PORT=8080
API_ADDR=127.0.0.1:${DEFAULT_PORT}


# CONTAINERS
pg:
	$(eval srvc := pg) ${(re)launchContainer} -p 127.0.0.1:5432:5432 -e POSTGRES_PASSWORD=docker -e POSTGRES_USER=docker -e POSTGRES_DB=docker -d postgres:alpine
adminer:
	$(eval srvc := adminer) ${(re)launchContainer} -d -p 127.0.0.1:7897:8080 adminer:4.2.5
migrate: $(eval SHELL:=/bin/bash)
	@while ! test "`echo -ne "\x00\x00\x00\x17\x00\x03\x00\x00user\x00username\x00\x00" | nc -w 3 127.0.0.1 5432 2>/dev/null | head -c1`" = R; do echo "waiting on postgres..."; sleep 0.3; done;
	diesel migration run 
export DATABASE_URL=postgres://docker:docker@127.0.0.1/docker
down:
	-docker rm -f -v `docker ps -a -q --filter "name=${current_dir}"`
current_dir = $(notdir $(shell pwd))
container_name = ${current_dir}-${srvc}
ifContainerMissing = @docker container inspect ${container_name} > /dev/null 2>&1 || 
(re)launchContainer = ${ifContainerMissing} docker run --rm --name ${container_name}

# DEPS
deps: rust-version
	@drill --version | grep 0.5.0 $s || cargo install drill --version 0.5.0
rust-version:
	@rustc --version | grep -E 'nightly.*2019-10-28' $s || rustup default nightly-2019-10-28
s = 2>&1 >/dev/null
