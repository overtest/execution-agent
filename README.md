# Overtest Agent

**Overtest Agent** is a part of [Overtest project](https://github.com/overtest/overtest) that is used to run programs with some resource and feature limits set. It is powered by [`limtrac`](https://github.com/overtest/limtrac) - another project from Overtest galaxy, which aims to create a Rust and C-compatible library that gives access to all the previously mentioned features with a simple API.

> :information_source: Note that `Overtest Agent` can be useless thing when it's used without Overtest, just because it gives a friendlier JSON-in JSON-out command line interface to interact with `limtrac`. If you need to run CLI applications with resource limits enforcements, you should create a custom program using C or Rust with `limtrac` added as a dependency.