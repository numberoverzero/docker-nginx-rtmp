# docker-nginx-rtmp

This is a pair of small, simple docker images that can be used to stand up
an [rtmp](https://en.wikipedia.org/wiki/Real-Time_Messaging_Protocol) server
with authentication.

Authentication is simple but cryptographically sound: any events
(publish, play) that you choose to authenticate must include an access key
in the query string.  You can provide an access key when starting the image,
or it will generate a cryptographically random one for you.

# How do I use it?

For an example of a setup that authenticates on publish but allows anyone
to play streams, see the `demo/` directory.
