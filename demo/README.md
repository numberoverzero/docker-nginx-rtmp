# docker-nginx-rtmp demo

To try this out yourself, clone the repo and run:

```
$ cd demo
$ docker compose up -d
[+] Running 2/2
 ✔ auth 1 layers [⣿]      0B/0B
   ✔ 6636430f4340 Pull complete
[+] Building 1.2s (7/7) FINISHED
[...]
[+] Running 3/3
 ✔ Network demo_default     Created
 ✔ Container demo-auth-1    Started
 ✔ Container demo-server-1  Started
```

We didn't set an access key, so we need to read it from the container log:

```
$ docker logs demo-auth-1
generating random access key (set your own with MA_ACCESS_KEY=)
socket: 0.0.0.0:80
access_key: ad657eee
querystring_key: key
```

From the above, we'll use the following url to authenticate while publishing
(the stream name doesn't matter, let's use `my_stream` for now):

```
rtmp://localhost:1935/live/my_stream?key=ad657eee
```

Let's test that out with [`ffmpeg`](https://ffmpeg.org/):

```
$ ffmpeg -re -i rickroll.mp4 -c:v libx264 -c:a aac -f flv "rtmp://localhost:1935/live/my_stream?key=ad657eee"
```

To view the stream, open the following url in a media player (using
[VLC](https://www.videolan.org/vlc/): Media -> Open Network Stream...)

```
rtmp://localhost:1935/live/my_stream
```

Run `docker compose down` to cleanup when you're done.
