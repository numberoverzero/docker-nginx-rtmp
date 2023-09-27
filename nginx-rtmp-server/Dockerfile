# modified by n/0:
#  dependencies:
#   * replaced PCRE download path since ftp.pcre.org is not supported
#   * migrated PCRE 8.43 -> PCRE2 10.42
#   * migrated ZLIP 1.2.11 -> 1.3
#   * migrated OPENSSL 1.1.1b -> 3.0.11
#   * migrated NGINX 1.15.9 -> 1.25.2
#   * updated checksums
#  features:
#   * added nginx-rtmp-module
# based on nginx-tiny (no license provided)
# src: https://github.com/sean-public/nginx-tiny/tree/32f7024ba24e266dbc7434a3da8b28e048d72bcc
#
FROM alpine:3.9 AS build

ARG PCRE_VERSION="10.42"
ARG PCRE_CHECKSUM="c33b418e3b936ee3153de2c61cc638e7e4fe3156022a5c77d0711bcbb9d64f1f"

ARG ZLIB_VERSION="1.3"
ARG ZLIB_CHECKSUM="ff0ba4c292013dbc27530b3a81e1f9a813cd39de01ca5e0f8bf355702efa593e"

ARG OPENSSL_VERSION="3.0.11"
ARG OPENSSL_CHECKSUM="b3425d3bb4a2218d0697eb41f7fc0cdede016ed19ca49d168b78e8d947887f55"

ARG NGINX_VERSION="1.25.2"
ARG NGINX_CHECKSUM="05dd6d9356d66a74e61035f2a42162f8c754c97cf1ba64e7a801ba158d6c0711"

ARG NGINX_RTMP_VERSION="1.2.2"
ARG NGINX_RTMP_CHECKSUM="07f19b7bffec5e357bb8820c63e5281debd45f5a2e6d46b1636d9202c3e09d78"

ARG NGINX_CONFIG="\
    --sbin-path=/nginx \
    --conf-path=/etc/nginx/nginx.conf \
    --pid-path=/tmp/nginx.pid \
    --http-log-path=/dev/stdout \
    --error-log-path=/dev/stdout \
    --http-client-body-temp-path=/tmp/client_temp \
    --http-proxy-temp-path=/tmp/proxy_temp \
    --http-fastcgi-temp-path=/tmp/fastcgi_temp \
    --http-uwsgi-temp-path=/tmp/uwsgi_temp \
    --http-scgi-temp-path=/tmp/scgi_temp \
    --with-pcre=/tmp/pcre-$PCRE_VERSION \
    --with-openssl=/tmp/openssl-$OPENSSL_VERSION \
    --with-zlib=/tmp/zlib-$ZLIB_VERSION \
    --with-file-aio \
    --with-http_v2_module \
    --with-http_ssl_module \
    --with-http_stub_status_module \
    --with-stream \
    --with-stream_ssl_module \
    --with-threads \
    --add-module=/tmp/nginx-rtmp \
    "

ADD https://github.com/PCRE2Project/pcre2/releases/download/pcre2-${PCRE_VERSION}/pcre2-${PCRE_VERSION}.tar.gz /tmp/pcre.tar.gz
ADD https://www.zlib.net/zlib-${ZLIB_VERSION}.tar.gz /tmp/zlib.tar.gz
ADD https://www.openssl.org/source/openssl-${OPENSSL_VERSION}.tar.gz /tmp/openssl.tar.gz
ADD https://nginx.org/download/nginx-${NGINX_VERSION}.tar.gz /tmp/nginx.tar.gz
ADD https://github.com/arut/nginx-rtmp-module/archive/refs/tags/v${NGINX_RTMP_VERSION}.tar.gz /tmp/nginx-rtmp.tar.gz

RUN cd /tmp && \
    if [ "$PCRE_CHECKSUM" != "$(sha256sum /tmp/pcre.tar.gz | awk '{print $1}')" ]; then exit 1; fi && \
    tar xf /tmp/pcre.tar.gz && \
    mv /tmp/pcre2-$PCRE_VERSION /tmp/pcre-$PCRE_VERSION && \
    if [ "$ZLIB_CHECKSUM" != "$(sha256sum /tmp/zlib.tar.gz | awk '{print $1}')" ]; then exit 1; fi && \
    tar xf /tmp/zlib.tar.gz && \
    if [ "$OPENSSL_CHECKSUM" != "$(sha256sum /tmp/openssl.tar.gz | awk '{print $1}')" ]; then exit 1; fi && \
    tar xf /tmp/openssl.tar.gz && \
    if [ "$NGINX_CHECKSUM" != "$(sha256sum /tmp/nginx.tar.gz | awk '{print $1}')" ]; then exit 1; fi && \
    tar xf /tmp/nginx.tar.gz && \
    mv /tmp/nginx-$NGINX_VERSION /tmp/nginx && \
    if [ "$NGINX_RTMP_CHECKSUM" != "$(sha256sum /tmp/nginx-rtmp.tar.gz | awk '{print $1}')" ]; then exit 1; fi && \
    tar xf /tmp/nginx-rtmp.tar.gz && \
    mv /tmp/nginx-rtmp-module-$NGINX_RTMP_VERSION /tmp/nginx-rtmp

RUN cd /tmp/nginx && \
    apk add git gcc g++ perl make linux-headers upx binutils && \
    ./configure $NGINX_CONFIG && \
    make
RUN strip /tmp/nginx/objs/nginx && \
    upx -9 /tmp/nginx/objs/nginx

FROM scratch

COPY /rootfs /

COPY --from=build /lib/ld-musl-x86_64.so.1 \
                  /lib/libssl.so.1.1 \
                  /lib/
COPY --from=build /tmp/nginx/objs/nginx /nginx

STOPSIGNAL SIGTERM

ENTRYPOINT ["/nginx", "-g", "daemon off;"]