FROM lscr.io/linuxserver/openssh-server:latest

RUN apk add --no-cache borgbackup
