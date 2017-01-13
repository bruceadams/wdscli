FROM busybox
MAINTAINER Bruce Adams <bruce.adams@acm.org>

RUN mkdir -p /usr/local/bin
ADD https://github.com/bruceadams/wdscli/releases/download/v0.5.0/wdscli.linux /usr/local/bin/wdscli
RUN chmod +rx /usr/local/bin/wdscli
RUN env | sort > env-at-build-time.log
