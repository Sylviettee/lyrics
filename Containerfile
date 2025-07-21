FROM docker.io/alpine:3.22

ARG TARGETPLATFORM

COPY out/${TARGETPLATFORM}/lyrics /bin
COPY ./root /var/spool/cron/crontabs/root

CMD ["crond", "-l", "2", "-f"]
