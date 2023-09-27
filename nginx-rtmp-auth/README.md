# nginx-rtmp-auth

## What is this

A very small, **very simple** rust-based http server that can be used as the
authentication endpoint for publishing rtmp streams through the
[nginx-rtmp-module](https://github.com/arut/nginx-rtmp-module).

When the process starts, it loads an access key from the env variable
`MA_ACCESS_KEY` or generates a secure random one if the variable isn't present.

On each incoming request, it checks the query string for a known key
(default: "key", configurable with `MA_QUERYSTRING_KEY`) and compares that
query string value against the known access key.

It returns 204 on success, 403 on wrong access key, and 400 on missing access
key.

This is designed to be a drop-in auth mechanism for `nginx-rtmp-server` when
you don't need anything more secure than a static key for publishing/playing
rtmp streams.

## How do I use this in docker?

Run this image (TODO: not connected to dockerhub yet)
```bash
$ docker run --rm -d -p 5000:5000 ghcr.io/numberoverzero/nginx-rtmp-auth
9840a2d81b2f7b9dd5fd9d60cbe890f37655a839518b1ed91c497bdf020281a9
```

Let's get the access key:
```bash
$ docker logs 984
generating random access key (set your own with MA_ACCESS_KEY=)
socket: 0.0.0.0:5000
access_key: 11119691
querystring_key: key
```

And verify that works:
```bash
$ curl -i localhost:5000?key=11119691
HTTP/1.1 204 No Content
date: Mon, 01 Jan 1980 00:00:01 GMT
```

## How do I configure this

There are a few environment variables:

* `MA_ACCESS_KEY` (default: `""``) -- the access key that callers must provide.
  If this is empty or unset, a secure random value will be generated.
* `MA_SOCKET` (default: `"0.0.0.0:5000"`) -- ip and port to bind to.
* `MA_QUERYSTRING_KEY` (default: `"key"`) -- the query string param name
  that the caller must provide the access key at


We can easily build and test this without docker.  We'll use a
generated access key and look for the query string param name `"secret"`:

```
$ cargo build

$ MA_QUERYSTRING_KEY="secret" ./target/debug/nginx-rtmp-auth
generating random access key (set your own with MA_ACCESS_KEY=)
socket: 0.0.0.0:5000
access_key: 1e0b81d5
querystring_key: secret
```

In another terminal, let's test the correct key:
```
$ curl -i localhost:5000?secret=1e0b81d5
HTTP/1.1 204 No Content
date: Mon, 01 Jan 1980 00:00:01 GMT
```

The wrong query string key returns a 400:
```
$ curl -i localhost:5000?wrong_key=1e0b81d5
HTTP/1.1 400 Bad Request
content-length: 0
date: Mon, 01 Jan 1980 00:00:01 GMT
```

The wrong value returns a 403:
```
$ curl -i localhost:5000?secret=hunter2
HTTP/1.1 401 Unauthorized
content-length: 0
date: Mon, 01 Jan 1980 00:00:01 GMT
```

Here's the corresponding output from the server:
```
allow: correct access key
deny: missing access key
deny: incorrect access key
```
