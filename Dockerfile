FROM alpine
MAINTAINER Bruce Adams <bruce.adams@acm.org>

RUN mkdir -p /usr/local/bin
ADD https://github.com/bruceadams/wdscli/releases/download/2.0.0/wdscli.linux /usr/local/bin/wdscli
RUN chmod +rx /usr/local/bin/wdscli
