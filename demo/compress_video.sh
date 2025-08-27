#!/bin/env bash

ffmpeg -i $1 -vcodec libx264 -crf 28 -preset veryslow -acodec aac compressed.mp4
