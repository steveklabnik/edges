# Edges

A port of [Peter Norvig's Lispy](http://norvig.com/lispy.html) in Rust.

[![Build Status](https://travis-ci.org/steveklabnik/edges.svg?branch=master)](https://travis-ci.org/steveklabnik/edges)

It's called Edges because Rust has sharp edges, and `()`s kinda look like the
edge of something. I'm bad at names.

There are some annoying bits about the way that Lispy uses dynamic types, and so
it's kinda hard to port over directly. I might try porting a different Lisp
instead or just totally writing my own.

## Building

```bash
$ git clone https://github.com/steveklabnik/edges.git
$ cd edges
$ cargo build
$ ./target/edges "(+ 1 1)"
```

This doens't currently actually work. We'll get there.

