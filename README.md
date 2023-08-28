# Rust GraphQL Template

The goal of this repository is to provide a template project that is both accessible to newcomers and useful for those experienced with Rust and GraphQL.

We use [actix-web](https://crates.io/crates/actix-web) as our web server but just about any web server crate is fine to use; we don't depend on any features specific to actix-web.

## Rust?

If you are completely new to Rust, you should probably read the [Rust book](https://doc.rust-lang.org/stable/book/) first.

Rust has nice, strong typing and great performance. The cost is the initial learning curve.

## GraphQL?

If you are completely new to GraphQL, you should read a little bit about it on the [official GraphQL website](https://graphql.org/learn/).

GraphQL allows you to create flexible interfaces for your frontend developers, allowing them to choose how data is accessed without requiring tons of page-specific [DTOs](https://en.wikipedia.org/wiki/Data_transfer_object). The cost, compared to the traditional REST alternative, is that more thought and time must be put into caching and batching.

## Rust and GraphQL _together_?

If you are new to using GraphQL and Rust together, you might want to read [the documentation](https://async-graphql.github.io/) for the GraphQL dependency that we'll be using, [async-graphql](https://crates.io/crates/async-graphql). Another viable alternative is [juniper](https://crates.io/crates/juniper), but we won't be using that here.

# Overview

## Workspace and Crates

Unlike typical Rust projects, we use a virtual workspace to use two crates instead of one: core and gateway.

Why do this?

Firstly, this promotes a good separate of concerns.

The core library should implement all of your application logic, with as much abstraction and encapsulation as you'd like.

The gateway crate, on the other hand, should only be concerned about transforming data pulled from the core library and presenting it to the client.

By doing this, you make it very easy to expose alternate APIs, should you choose to implement part or all of your API via REST or gRPC.
You also make it easy to create tools and jobs that cleanly depend on your application logic without needing to pull in your GraphQL code.

## Database Interaction

For the sake of simplicity, this repository doesn't implement database interaction. When you decide to tackle this, you should look into the [diesel](https://crates.io/crates/diesel) crate. If you want to work with diesel using async, look into [diesel-async](https://crates.io/crates/diesel-async).

## Authentication

TODO

## Authorization

TODO