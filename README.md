|Crate|Documentation|Linux/OS X|Windows|
|:---:|:-----------:|:--------:|:-----:|
|[![Crate](https://img.shields.io/crates/v/ex.svg)](https://crates.io/crates/ex)|[![Documentation](https://docs.rs/ex/badge.svg)](https://docs.rs/ex/)|[![Build Status](https://travis-ci.org/Ereski/ex.svg?branch=master)](https://travis-ci.org/Ereski/ex)|[![Build Status](https://ci.appveyor.com/api/projects/status/github/Ereski/ex?svg=true&branch=master)](https://ci.appveyor.com/project/Ereski/ex)|

A `libstd` wrapper with more detailed errors. This crate mirrors the `std`
module tree, replacing failable functions and structs with wrappers that add
additional information to the errors returned. *WIP, so not all of `libstd`
may be covered*.

To use `ex`, simply replace a `use std::x` with `use ex::x` for any `x`.
Some structs are different from `libstd`'s, so there might exist some
friction with external crates. In that case, see the `Wrapper` trait on how
to get the wrapped structs.

`ex` also uses custom error types to transport the augmented error messages.
In all cases, you can use `err.cause()` to get a reference to the original
error, or the `Wrapper trait`.
