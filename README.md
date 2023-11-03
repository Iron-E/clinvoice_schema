# Winvoice Schema

<!-- cargo-rdme start -->

This crate provides definitions for the information which is managed by Winvoice:

* **Contact** information,
* **employees** and their **departments**,
* **jobs** and **invoices**,
* **organizations** and their respective **locations** around the world, and
* **timesheets** and related **expenses**.

## Features

* `serde` adds support for the [`serde`] crate.

## Re-exports

The crate provides access to the following elements of other crates:

* Elements of the [`money2`] which are required to instantiate data (e.g. [`Money`]).
* The entire [`chrono`] crate, as almost all of it is required to instantiate certain data.

<!-- cargo-rdme end -->
