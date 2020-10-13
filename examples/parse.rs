use did::did;
use did::Result;
use did::DID;

#[rustfmt::skip]
fn main() -> Result<()> {
  dbg!(did!("did:foo:21tDAKCERh95uGgKbJNHYp?service=agent"));
  dbg!(did!("did:foo:21tDAKCERh95uGgKbJNHYp?version-time=2002-10-10T17:00:00Z"));
  dbg!(did!("did:example:123456/path"));
  dbg!(did!("did:example:123456?query=true"));
  dbg!(did!("did:example:123456#public-key-1"));
  dbg!(did!("did:example:123456789abcdefghi").join("#key-1")?);

  dbg!(did!("did:example:123?initial-state=eyJkZWx0YV9oYXNoIjoiRWlDUlRKZ2Q0U0V2YUZDLW9fNUZjQnZJUkRtWF94Z3RLX3g"));
  dbg!(did!("did:example:123?initial-state=eyJkZWx0YV9oYXNoIjoiRWlDUlRKZ2Q0U0V2YUZDLW9fNUZjQnZJUkRtWF94Z3RLX3g#keys-1"));

  dbg!(did!("did:example:123?transform-keys=jwk"));
  dbg!(did!("did:example:123?transform-keys=jwks"));
  dbg!(did!("did:example:123?transform-keys=jwk#keys-1"));

  dbg!(did!("did:web:w3c-ccg.github.io"));
  dbg!(did!("did:web:w3c-ccg.github.io:user:alice"));

  let did: DID = did!("did:example:123456789abcdefghi/path/to/this?key=val#frag");

  println!("[+] {}", "=".repeat(80));
  println!("[+] Size   > {} bytes", ::core::mem::size_of::<DID>());
  println!("[+] DID(?) > {:?}", did);
  println!("[+] DID(!) > {}", did);
  println!("[+] DID(Q) > {:?}", did.query_pairs().collect::<Vec<_>>());
  println!("[+] DID(R) > {}", did.join("?foo=bar#hello")?);
  println!("[+] {}", "=".repeat(80));

  // dbg!(did!("did:method:identifier;foo:bar=baz;service=thing;o.key=123;o.value=456/path/to/this?thing=that"));
  dbg!(did!("did:method:identifier/path/to/this?thing=that#fragment=hash"));
  // dbg!(did!("did:method:identifier;init_param;param=value;boolean=true;number=123;object.key=hello;object.value=world/path/to/this?thing=that&nested[property]=this#fragment=hash"));
  // dbg!(did!("did:method:identifier;service=this;value=12;param=true;o.a=1;o.b=3/path/to/this?thing=that#fragment=hash"));

  Ok(())
}
