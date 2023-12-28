# ptodd.org Backend Framework

This is the repository for the front- and back-ends supporting ptodd.org.

It is built "the hard way" intentionally avoiding third-party crates with a
preference to implement all capabilities using only the standard library. The
exception to this is the usage of the [log](https://crates.io/crates/log)
crate as a facade over the included logging implementation.
