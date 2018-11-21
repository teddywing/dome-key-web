#!/bin/sh

# Copyright (c) 2018  Teddy Wing
#
# This file is part of DomeKey Web.
#
# DomeKey Web is free software: you can redistribute it and/or modify it
# under the terms of the GNU Affero General Public License as published
# by the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# DomeKey Web is distributed in the hope that it will be useful, but
# WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
# Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public
# License along with DomeKey Web. If not, see
# <https://www.gnu.org/licenses/>.

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
