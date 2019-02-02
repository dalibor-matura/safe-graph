# Safe Graph

Rust implementation of Graph, being refactored version of `GraphMap` from popular [petgraph](https://crates.io/crates/petgraph) crate.

|Crate|Documentation|Travis CI|CodeCov|
|:---:|:-----------:|:-------:|:-----:|
|[![Crate](http://meritbadge.herokuapp.com/safe-graph)](https://crates.io/crates/safe-graph)|[![Documentation](https://docs.rs/safe-graph/badge.svg)](https://docs.rs/safe-graph)|[![Build Status](https://travis-ci.org/dalibor-matura/safe-graph.svg?branch=master)](https://travis-ci.org/dalibor-matura/safe-graph)|[![codecov](https://codecov.io/gh/dalibor-matura/safe-graph/branch/master/graph/badge.svg)](https://codecov.io/gh/dalibor-matura/safe-graph)

## Refactoring done

I’ve decided to start refactoring of [petgraph](https://crates.io/crates/petgraph) crate for couple of reasons:
* I don't like to use a big library with a large codebase when I need just a piece of it (in my case `GraphMap`). It is a high risk of introducing way in for mailicious attackers.
* [Petgraph](https://crates.io/crates/petgraph) has almost no tests and I wanted to have a high/full test coverage, so I’ve added tests in.
* [Petgraph](https://crates.io/crates/petgraph) has [ordermap](https://crates.io/crates/ordermap) crate as a dependency, but it is outdated and not stable. Its current stable version was renamed to [indexmap](https://crates.io/crates/indexmap), so I've updated to it.
* I’ve done a few other modifications according to best practice. 
* [Petgraph](https://crates.io/crates/petgraph) crate doesn't have a stable release version yet and I didn't want to depend on it.

## Plans

* I'm going to contact autors of [Petgraph](https://crates.io/crates/petgraph) to offer them integrate my improvements.
* I'm thinking about refactoring of [Petgraph](https://crates.io/crates/petgraph) into more modular collection of smaller Graph libraries with better Test Coverage and more structured separation of functionality.

## License
Licensed under the General Public License (GPL), version 3 ([LICENSE](https://github.com/dalibor-matura/safe-graph/blob/master/LICENSE) http://www.gnu.org/licenses/gpl-3.0.en.html).
