# micro-nginx

## What is this

Similar to [nginx-tiny](https://github.com/sean-public/nginx-tiny) with a few
changes:

* updated default versions of pcre, zlib, openssl, nginx
* updated download url for pcre
* adds the [nginx-rtmp-module](https://github.com/arut/nginx-rtmp-module) to nginx

## How do I use it

Create a `nginx.conf` that runs as the user and group `root` (note: there are no other users in the container):

```nginx
user root;
worker_processes auto;
rtmp_auto_push on;
events {}
rtmp {
    server {
        listen 1935;
        application live {
            live on;
            record off;
        }
    }
}
```

Create a dockerfile that builds from this and copies in your conf:
```docker
FROM micro-nginx
COPY nginx.conf /etc/nginx/nginx.conf
```

That's it!

## Where are my logs

If you need to debug the nginx container then you should
**use a different base image**.

The base image is built `from scratch` and the nginx binary is run through
both `strip` and `upx`.  When compiling nginx, both `--http-log-path` and
`--error-log-path` are set to `/dev/stdout`.
