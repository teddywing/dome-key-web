#!/bin/sh

set -x

ffmpeg \
	-i Screencast.mov \
	-f webm \
	-c:v libvpx \
	-b 275k \
	-acodec libvorbis \
	DomeKey-Screencast.webm

ffmpeg \
	-i Screencast.mov \
	-vcodec h264 \
	-acodec aac \
	-strict \
	-2 \
	DomeKey-Screencast.h264.mp4

ffmpeg \
	-i Screencast.mov \
	-c:v mpeg4 \
	-q:v 5 \
	-acodec aac \
	DomeKey-Screencast.mpeg4.mp4
