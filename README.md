# TextGraph

A small terminal utility which displays graphs as plain text,
with the goal of having few dependencies,
being lightweight, and reasonably performant.

It is meant to be used in a traditional unix way, 
by piping data through it.
It was written because I sometimes have the need to
watch some state in a sql table for a short while.

The documentation is in the manual file `textgraph.1`.

It is only tested with glibc and terminals with ansi escape code support.

## Usage
There is a script in `packaging` which can be used to make a `.deb`.
If you are on a debian based distro, you should be able to run `./packaging/package-debian.sh`.
Then a `textgraph.deb` file should be made somewhere in `/tmp/`.

There is a script in `packaging` which can be used to make a `.rpm`.
If you are on a rpm based distro, you should be able to run `./packaging/package-fedora.sh`.
Then a `textgraph-<buildinfo>.rpm` should be build in `~/rpmbuild/RPMS/x86_64/`.

You can compile it with `cargo build --release`.
If you compile with `cargo build --release`, you can copy it to your path, e.g. 
```
sudo install target/release/textgraph /usr/local/bin/textgraph
sudo install textgraph.1 /usr/share/man/man1/textgraph.1
```

### Features

By default `libc` and `ansi` is enabled.

The `libc` feature makes the program able to get the terminal size,
and catch signals like SIGINT.
The last part is needed if you want the program to use the alternate screen
in filter-mode.
It does not currently use the `libc`-crate, but it does use c-functions.

The `ansi` feature enables the use of ansi escape codes,
this is used for manipulating the terminal. 
Like adding colors to the graph,
and switching back and fourth from the alternate screen 
(this will only happen when `libc` also is enabled).

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
8  │             ⠠⠔⠊⠉⠉⠑⠢⠄              │
6  │            ⠔⠁      ⠈⠢             │
3  │          ⠠⠊          ⠑⠄           │
1  │         ⠔⠁            ⠈⠢          │
-1 │        ⠌                ⠡         │
-3 │      ⠠⠊                  ⠑⠄       │
-6 │    ⠠⠔⠁                    ⠈⠢⠄     │
-8 │⠤⠤⠤⠒⠁                        ⠈⠒⠤⠤⠤⠒│
-10└───────────────────────────────────┘
```

## Manual
For reference, this is a translation of the manual page.
Best is to check to manual page itself, not the README `man -l textgraph.1`.
```
TEXTGRAPH(1)                General Commands Manual               TEXTGRAPH(1)

name
       TextGraph - Text graphing utility

SYNOPSIS
       textgraph [OPTIONS] [input_file]

DESCRIPTION
       textgraph TermGraph is a utility for graphing

OPTIONS
       --help Display help information.

       -s, --silent
              Disable distracting elements, such as axis and non-graph text.

       -n, --last-n count
              If  specified,  only  the  newest count samples will be plotted.
              This can be useful if you want to follow the latest state  of  a
              graph that is piped in.

       -c, --cut
              This  is a special case of --last-n. Where the number of columns
              --width will be used for the count.

       -a, --ascii
              Shorthand for -t ascii, if  multiple  options  setting  mode  is
              specified, the last will likely be respected.

       -b, --braille
              Shorthand  for  -t  braille, if multiple options setting mode is
              specified, the last will likely be respected.

       -t star|ascii|braille|braille6|braille8
              The type of graph to draw, it defaults to  star,  which  is  the
              fastest one.

              star Scatter plot using only the '*' character.

              ascii Ascii is slightly prettier to look at.

              braille, braille6 Uses braille characters to draw higher resolu‐
              tion plots.

              braille8 This is the most scatter-plot-ish with the highest res‐
              olution, but also the most buggy.

       -w, --width width
              Specify  a  width for the output.  If not specified, it will at‐
              tempt to determine the TTY width and use that.  If it cannot  be
              automatically determined, it will fail.

       -h, --height height
              Specify  a height for the output.  If not specified, it will at‐
              tempt to determine the TTY height and use that.  If it cannot be
              automatically determined, it will fail.

       --color yes|no
              Enable or disable colors, by default color will be enabled if it
              looks like a tty is connected.

              It can therefore be nice to use --color yes if  you  are  piping
              the output into another program that supports colors.

EXAMPLES
       The simplest version is if you have a text file of values

              cat file | textgraph

                                  2024-06-08                      TEXTGRAPH(1)
```
