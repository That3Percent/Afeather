#[macro_use]
extern crate criterion;
use criterion::{Criterion, BenchmarkId};


// TODO: Compare Process to ProcessSimple for an easy case with straightforward types
fn macro_process() {

}

// TODO: Add another which compares Process and ProcessSimple for a complex type, eg: with Option


criterion_group!(benches, macro_process);
criterion_main!(benches);
