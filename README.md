# simple-log
**simple-log** is a simple logging library for Rust.

This logger doesn't implement the trait required by `log`.

And it's so simple, only logging level and the message are output into the stdout.

So best use it for console applications where log messages are used for presenting information to the users,
instead of providing debug information for the developers.
