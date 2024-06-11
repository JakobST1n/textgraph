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


## Example 1
Some examples of the different modes, from the same input file with random numbers
### Star mode
```
100┌─────────────────────────────────────────────┐
89 │           *                             *   │
79 │             *    *     *        **          │
68 │         **      *            *              │
58 │ *    *       *       *        *     *  *    │
47 │  *    *       *         *         *       **│
37 │*       *          *** *    *   *   *     *  │
26 │   *        *   *          *           *     │
16 │    **                    *  *        *      │
5  └─────────────────────────────────────────────┘
```
### Ascii mode
```
100┌─────────────────────────────────────────────┐
89 │          ╭╮                            ╭╮   │
79 │          ││╭╮   ╭╮    ╭╮       ╭─╮     ││   │
68 │        ╭─╯│││  ╭╯│    ││    ╭╮ │ │     ││   │
58 │╭╮   ╭╮ │  ││╰╮ │ │  ╭╮││    │╰╮│ │ ╭╮ ╭╯│   │
47 ││╰╮  │╰╮│  ││ ╰╮│ │  │││╰╮   │ ││ ╰╮││ │ │╭──┤
37 ├╯ │  │ ╰╯  ││  ││ ╰──╯╰╯ │ ╭╮│ ╰╯  ╰╯│ │ ╰╯  │
26 │  ╰╮ │     ╰╯  ╰╯        │╭╯││       │╭╯     │
16 │   ╰─╯                   ╰╯ ╰╯       ╰╯      │
5  └─────────────────────────────────────────────┘
```
### Braille mode (Probably the most buggy mode)
```
100┌────────────────────┐
89 │   ⠄⠂               │
79 │              ⠄     │
68 │  ⠠   ⠂     ⠠       │
58 │⠠   ⠠        ⠄  ⠂⠠  │
47 │          ⠄    ⠂   ⠂│
37 │ ⠠     ⠂ ⠄ ⠐ ⠠ ⠐    │
26 │     ⠠     ⠂     ⠂  │
16 │⠂         ⠠ ⠂   ⠐   │
5  └────────────────────┘
```

## Example 2
Example of a simple sinusoid
### Star
```
10 ┌──────────────────────────────────────────────────────────────────────┐
8  │                             **********                               │
6  │                        *****          *****                          │
3  │                     ***                    ***                       │
1  │                  ***                          ***                    │
-1 │               ***                                ***                 │
-3 │            ***                                      ***              │
-6 │        ****                                            ****          │
-8 │********                                                    **********│
-10└──────────────────────────────────────────────────────────────────────┘
```
### Ascii
```
10 ┌──────────────────────────────────────────────────────────────────────┐
8  │                            ╭─────────╮                               │
6  │                       ╭────╯         ╰────╮                          │
3  │                    ╭──╯                   ╰──╮                       │
1  │                 ╭──╯                         ╰──╮                    │
-1 │              ╭──╯                               ╰──╮                 │
-3 │           ╭──╯                                     ╰──╮              │
-6 │       ╭───╯                                           ╰───╮          │
-8 ├───────╯                                                   ╰──────────┤
-10└──────────────────────────────────────────────────────────────────────┘
```
### Braille
```
10 ┌───────────────────────────────────┐
8  │             ⠈⠑⠢⠤⠤⠔⠊⠁              │
6  │            ⠑⠄      ⠠⠊             │
3  │          ⠈⠢          ⠔⠁           │
1  │         ⠑⠄            ⠠⠊          │
-1 │        ⠡                ⠌         │
-3 │      ⠈⠢                  ⠔⠁       │
-6 │    ⠈⠑⠄                    ⠠⠊⠁     │
-8 │⠉⠉⠉⠒⠄                        ⠠⠒⠉⠉⠉⠒│
-10└───────────────────────────────────┘
```

## Manual
For reference, this is a translation of the manual page.
Best is to check to manual page itself, not the README `man -l tg.1`.
```
TG(1)                       General Commands Manual                      TG(1)



name
       tg - TermGraph - Text graphing utility

SYNOPSIS
       tg  [-s|--silent]  [-l|--last-n  N]  [-h|--height N] [-w|--width N] [-t
       type]


DESCRIPTION
       tg TermGraph is a utility for graphing


OPTIONS
       -h, --help
              Display help information.


       -s, --silent
              Disable distracting elements, such as axis and non-graph text.


       -a, --ascii
              Shorthand for -t ascii, if  multiple  options  setting  mode  is
              specified, the last will likely be respected.


       -b, --braille
              Shorthand  for  -t  braille, if multiple options setting mode is
              specified, the last will likely be respected.


       -n, --last-n count
              If specified, only the newest count  samples  will  be  plotted.
              This  can  be useful if you want to follow the latest state of a
              graph that is piped in.


       -c, --cut
              This is a special case of --last-n. Where the number of  columns
              --width will be used for the count.


       -t type
              The  type  of  graph  to draw, it defaults to star, which is the
              fastest one.  Options are star, ascii  and  braille.   Ascii  is
              slightly prettier to look at.


       -w, --width width
              Specify  a  width for the output.  If not specified, it will at‐
              tempt to determine the TTY width and use that.  If it cannot  be
              automatically determined, it will fail.


       -h, --height height
              Specify  a height for the output.  If not specified, it will at‐
              tempt to determine the TTY height and use that.  If it cannot be
              automatically determined, it will fail.


EXAMPLES
       The simplest version is if you have a text file of values

              cat file | tg

                                  2024-06-08                             TG(1)
```
