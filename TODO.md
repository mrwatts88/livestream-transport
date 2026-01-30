# UP NEXT

# TODO

[ ] anylyze what's happening. does MPEG-TS work if the producer sends fast enough? slower?
[ ] allow passing port as consumer arg and remove hardcoded signaling stuff
[ ] auto connect when consumer starts and add to consumers array in producer
[ ] add observability
[ ] add a bounded channel and rate limit pipe to ffmpeg
[ ] pace producer sends
[ ] switch to H.264
[ ] add sequencing and/or timestamps (simple RTP)
[ ] reorder in client buffer, drop stale frames

# DONE

[x] read bytes from camera with ffmpeg
[x] play video from url with ffmpeg
[x] pipe bytes to ffplay and show in media player (MPEG-TS)
[x] socket producer and consumer
[x] create byte producer and send to clients
[x] on client read and print bytes
[x] pipe ffmpeg to rust producer and read from stdout, send over socket
[x] on client read bytes from socket and pipe to ffplay
