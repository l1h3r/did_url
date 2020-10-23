#![feature(test)]
extern crate test;

#[bench]
fn short(bench: &mut test::Bencher) {
  let did: &str = "did:example:bench";

  bench.bytes = did.len() as u64;

  bench.iter(|| test::black_box(did).parse::<did_url::DID>().unwrap());
}

#[bench]
fn full(bench: &mut test::Bencher) {
  let did: &str = "did:example:bench/path/to/that?foo=a&bar=b&baz=c#my-frag";

  bench.bytes = did.len() as u64;

  bench.iter(|| test::black_box(did).parse::<did_url::DID>().unwrap());
}
