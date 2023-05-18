# Winvoice Schema

<!-- cargo-rdme start -->

This crate provides definitions for the information which is managed by Winvoice. The data is represented as it would be after all `JOIN`s are performed on a database (e.g. an [`Organization`] in a database would likely reference [`Location`] by [`Id`], rather than aggregating it).

## Features

* `serde` adds support for the [`serde`] crate.

## Re-exports

The crate provides access to the following elements of other crates:

* Elements of the [`winvoice_finance`] which are required to instantiate data (e.g. [`Money`]).
* The entire [`chrono`] crate, as almost all of it is required to instantiate certain data.

<!-- cargo-rdme end -->
