# TODO

[ ] read bytes from camera with ffmpeg
[ ] play video from url with ffmpeg
[ ] pipe bytes to ffmpeg and show in media player (MPEG-TS)
[ ] socket client and server
[ ] create byte producer and send to clients
[ ] on client read and print bytes
[ ] on server, call ffmpeg from rust and read from stdout, send over socket
[ ] on client send bytes into ffmpeg by piping into process
[ ] anylyze what's happening. does MPEG-TS work if the producer sends fast enough? slower?
[ ] add a bounded channel and rate limit pipe to ffmpeg
[ ] pace producer sends
[ ] switch to H.264
[ ] add sequencing and/or timestamps (simple RTP)
[ ] reorder in client buffer, drop stale frames

