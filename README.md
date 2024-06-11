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
### Ascii
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
### Braille
10 ┌───────────────────────────────────┐
8  │             ⠐⠢⠄  ⠠⠔⠂              │
6  │            ⠢        ⠔             │
3  │          ⠐⠄          ⠠⠂           │
1  │         ⠢              ⠔          │
-1 │        ⠂                ⠐         │
-3 │      ⠐⠄                  ⠠⠂       │
-6 │    ⠐⠢                      ⠔⠂     │
-8 │⠒⠒⠒⠤                          ⠤⠒⠒⠒⠤│
-10└───────────────────────────────────┘
