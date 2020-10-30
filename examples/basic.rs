use did_url::Result;
use did_url::DID;

fn main() -> Result<()> {
  let did = DID::parse("did:example:alice")?;

  // Prints Method = example
  println!("Method = {}", did.method());

  // Prints Method Id = alice
  println!("Method Id = {}", did.method_id());

  // Prints DID = did:example:alice
  println!("DID = {}", did);

  // Prints Joined = did:example:alice?query=true#key-1
  println!("Joined = {}", did.join("?query=true")?.join("#key-1")?);

  Ok(())
}
