# NOTES

Simplest camera/mic to media player setup:

```
ffmpeg -f v4l2 -i /dev/video0 -f mpegts - | ffplay -vf setpts=0 -
```

This project is essentially building the | part for distributed systems.
