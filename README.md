This repo demonstrates my difficulty factoring out "initialization" code into separate functions in Rust.
See [this Hacker News comment](https://news.ycombinator.com/item?id=13786831) for my thoughts about it.

The `master` branch compiles. (It fails to connect to a database, and it would probably fail to prepare the statements too, but that's not the point.)

The `i_wish` branch shows what I'd like to have, but it doesn't compile.

I am using Rust 1.15.1.

