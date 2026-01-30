# UP NEXT
[ ] on server, call ffmpeg from rust and read from stdout, send over socket
[ ] on client send bytes into ffplay by piping into process

# TODO

[ ] anylyze what's happening. does MPEG-TS work if the producer sends fast enough? slower?
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
