use did::DID;
use proptest::prelude::*;
use proptest::string::string_regex;

fn did_syntax() -> impl Strategy<Value = String> {
  string_regex("did:([0-9a-z]+):([0-9a-zA-Z._-]+)(([0-9a-zA-Z._-]*:)*)")
    .unwrap()
    .prop_filter("ascii", |value| value.is_ascii())
}

proptest! {
  #![proptest_config(ProptestConfig::with_cases(1024))]
  // TODO: Test path
  // TODO: Test query
  // TODO: Test fragment
  #[test]
  fn parse_did_syntax(input in did_syntax()) {
    DID::parse(&input).unwrap();
  }
}
