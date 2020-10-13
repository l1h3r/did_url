//! Utilities for [Web DID](https://w3c-ccg.github.io/did-method-web/)
//!
//! ### DID Format
//!
//!   `web-did = "did:web:" domain-name`
//!
//!   `web-did = "did:web:" domain-name * (":" path)`
//!
//! ### References
//!
//! - [Web DID Method](https://w3c-ccg.github.io/did-method-web/)
//!
use alloc::format;
use alloc::string::String;

use crate::did::DID;
use crate::error::Error;
use crate::error::Result;

const METHOD: &str = "web";

/// Creates a new Web DID from the provided `input`.
pub fn new(input: impl AsRef<str>) -> Result<DID> {
  let (domain, path): (&str, &str) = parse(input.as_ref(), '/')?;

  if path.is_empty() {
    DID::parse(format!("did:{}:{}", METHOD, domain))
  } else {
    DID::parse(format!(
      "did:{}:{}{}",
      METHOD,
      domain,
      path.replace('/', ":")
    ))
  }
}

/// Creates a Web DID URL specifying the location of the DID document resource.
pub fn url(did: &DID) -> Result<String> {
  if did.method() != METHOD {
    return Err(Error::InvalidMethodName);
  }

  let (domain, path): (&str, &str) = parse(did.method_id(), ':')?;

  if path.is_empty() {
    Ok(format!("https://{}/.well-known/did.json", domain))
  } else {
    Ok(format!(
      "https://{}{}/did.json",
      domain,
      path.replace(':', "/")
    ))
  }
}

fn parse(data: &str, split: char) -> Result<(&str, &str)> {
  match data
    .find(split)
    .map(|index| data.split_at(index))
    .unwrap_or((data, ""))
  {
    ("", _) => Err(Error::InvalidMethodId),
    (domain, path) => Ok((domain, path.trim_end_matches('/'))),
  }
}
