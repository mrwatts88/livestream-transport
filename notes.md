# NOTES

This project is essentially building the | part for distributed systems.


Simplest camera/mic to media player setup:

```
ffmpeg -f v4l2 -i /dev/video0 -f mpegts - | ffplay -vf setpts=0 -
```


Piping to producer, sending to consumer, piping to ffplay

```
ffmpeg -hide_banner -loglevel error -f v4l2 -i /dev/video0 -f mpegts - | ./target/debug/producer

./target/debug/consumer c | ffplay -vf setpts=0 -
```
