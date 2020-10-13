use did::DID;

#[test]
#[rustfmt::skip]
fn test_parse_valid_method() {
  assert!(DID::parse("did:method:identifier").is_ok());
  assert!(DID::parse("did:000:identifier").is_ok());
  assert!(DID::parse("did:m:identifier").is_ok());
}

#[test]
#[rustfmt::skip]
fn test_parse_invalid_method() {
  assert!(DID::parse("did::identifier").is_err());
  assert!(DID::parse("did:METHOD:").is_err());
  assert!(DID::parse("did:-f:o:o-:").is_err());
  assert!(DID::parse("did:").is_err());
  assert!(DID::parse("did: : : :").is_err());
}

#[test]
#[rustfmt::skip]
fn test_parse_valid_method_id() {
  assert!(DID::parse("did:method:i").is_ok());
  assert!(DID::parse("did:method:identifier").is_ok());
  assert!(DID::parse("did:method:IDENTIFIER").is_ok());
  assert!(DID::parse("did:method:did:__:--:1231093213").is_ok());
}

#[test]
#[rustfmt::skip]
fn test_parse_invalid_method_id() {
  assert!(DID::parse("did:method:: :").is_err());
  assert!(DID::parse("did:method: - - -").is_err());
  assert!(DID::parse("did:method:*****").is_err());
  assert!(DID::parse("did:method:identifier-|$|").is_err());
}

#[test]
#[rustfmt::skip]
fn test_parse_simple_2() {
  let _: DID = dbg!(DID::parse("did:method:identifier").unwrap());
  let _: DID = dbg!(DID::parse("did:method:identifier/path/to/this").unwrap());
  let _: DID = dbg!(DID::parse("did:method:identifier/path/to/this?thing=that").unwrap());
  let _: DID = dbg!(DID::parse("did:method:identifier/?thing=that").unwrap());
  let _: DID = dbg!(DID::parse("did:method:identifier/path/to/this?thing=that.foo#fragment=hash.foo").unwrap());
  let _: DID = dbg!(DID::parse("did:method:identifier#fragment=hash").unwrap());
  let _: DID = dbg!(DID::parse("did:method:identifier?#fragment=hash").unwrap());
}

#[test]
#[rustfmt::skip]
fn test_parse_simple() {
  let did: DID = DID::parse("did:foo:21tDAKCERh95uGgKbJNHYp").unwrap();
  assert_eq!(did.method(), "foo");
  assert_eq!(did.method_id(), "21tDAKCERh95uGgKbJNHYp");
  assert_eq!(did.path(), "");
  assert_eq!(did.query(), None);
  assert_eq!(did.fragment(), None);
}

#[test]
#[rustfmt::skip]
fn test_parse_simple_did_parameter_1() {
  let did: DID = DID::parse("did:foo:21tDAKCERh95uGgKbJNHYp?service=agent").unwrap();
  assert_eq!(did.method(), "foo");
  assert_eq!(did.method_id(), "21tDAKCERh95uGgKbJNHYp");
  assert_eq!(did.path(), "");
  assert_eq!(did.query(), Some("service=agent"));
  assert_eq!(did.fragment(), None);
}

#[test]
#[rustfmt::skip]
fn test_parse_simple_did_parameter_2() {
  let did: DID = DID::parse("did:foo:21tDAKCERh95uGgKbJNHYp?version-time=2002-10-10T17:00:00Z").unwrap();
  assert_eq!(did.method(), "foo");
  assert_eq!(did.method_id(), "21tDAKCERh95uGgKbJNHYp");
  assert_eq!(did.path(), "");
  assert_eq!(did.query(), Some("version-time=2002-10-10T17:00:00Z"));
  assert_eq!(did.fragment(), None);
}

#[test]
#[rustfmt::skip]
fn test_parse_simple_path() {
  let did: DID = DID::parse("did:example:123456/path").unwrap();
  assert_eq!(did.method(), "example");
  assert_eq!(did.method_id(), "123456");
  assert_eq!(did.path(), "/path");
  assert_eq!(did.query(), None);
  assert_eq!(did.fragment(), None);
}

#[test]
#[rustfmt::skip]
fn test_parse_simple_query() {
  let did: DID = DID::parse("did:example:123456?query=true").unwrap();
  assert_eq!(did.method(), "example");
  assert_eq!(did.method_id(), "123456");
  assert_eq!(did.path(), "");
  assert_eq!(did.query(), Some("query=true"));
  assert_eq!(did.fragment(), None);
}

#[test]
#[rustfmt::skip]
fn test_parse_simple_fragment() {
  let did: DID = DID::parse("did:example:123456#public-key-1").unwrap();
  assert_eq!(did.method(), "example");
  assert_eq!(did.method_id(), "123456");
  assert_eq!(did.path(), "");
  assert_eq!(did.query(), None);
  assert_eq!(did.fragment(), Some("public-key-1"));
}
