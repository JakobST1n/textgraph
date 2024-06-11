# TextGraph

A small terminal utility which displays graphs as plain text,
with the goal of having few dependencies,
being lightweight, and reasonably performant.

It is meant to be used in a traditional unix way, 
by piping data through it.
It was written because I sometimes have the need to
watch some state in a sql table for a short while.

The documentation is in the manual file `tg.1`.

It is only tested with glibc and terminals with ansi escape code support.
