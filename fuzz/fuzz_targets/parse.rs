#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
  if let Ok(string) = core::str::from_utf8(data) {
    if let Ok(did) = did_url::DID::parse(string) {
      assert_eq!(
        did,
        did_url::DID::parse(did.as_str()).unwrap()
      );
    }
  }
});
