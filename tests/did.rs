use did::*;

#[test]
fn test_method() -> Result<()> {
  let mut did: DID = did!("did:test:123/a/b/c?foo=bar#frag");
  assert_eq!(did.method(), "test");
  assert_eq!(did.method_id(), "123");
  assert_eq!(did.path(), "/a/b/c");
  assert_eq!(did.query(), Some("foo=bar"));
  assert_eq!(did.as_str(), "did:test:123/a/b/c?foo=bar#frag");

  did.set_method("foo");
  assert_eq!(did.method(), "foo");
  assert_eq!(did.method_id(), "123");
  assert_eq!(did.path(), "/a/b/c");
  assert_eq!(did.query(), Some("foo=bar"));
  assert_eq!(did.as_str(), "did:foo:123/a/b/c?foo=bar#frag");

  did.set_method("foobar");
  assert_eq!(did.method(), "foobar");
  assert_eq!(did.method_id(), "123");
  assert_eq!(did.path(), "/a/b/c");
  assert_eq!(did.query(), Some("foo=bar"));
  assert_eq!(did.as_str(), "did:foobar:123/a/b/c?foo=bar#frag");

  Ok(())
}

#[test]
fn test_method_id() -> Result<()> {
  let mut did: DID = did!("did:test:123/a/b/c?foo=bar#frag");
  assert_eq!(did.method(), "test");
  assert_eq!(did.method_id(), "123");
  assert_eq!(did.path(), "/a/b/c");
  assert_eq!(did.query(), Some("foo=bar"));
  assert_eq!(did.as_str(), "did:test:123/a/b/c?foo=bar#frag");

  did.set_method_id("456");
  assert_eq!(did.method(), "test");
  assert_eq!(did.method_id(), "456");
  assert_eq!(did.path(), "/a/b/c");
  assert_eq!(did.query(), Some("foo=bar"));
  assert_eq!(did.as_str(), "did:test:456/a/b/c?foo=bar#frag");

  Ok(())
}

#[test]
fn test_path() -> Result<()> {
  let mut did: DID = did!("did:test:123/a/b/c?foo=bar#frag");
  assert_eq!(did.method(), "test");
  assert_eq!(did.method_id(), "123");
  assert_eq!(did.path(), "/a/b/c");
  assert_eq!(did.query(), Some("foo=bar"));
  assert_eq!(did.as_str(), "did:test:123/a/b/c?foo=bar#frag");

  did.set_path("/foo/bar/baz");
  assert_eq!(did.method(), "test");
  assert_eq!(did.method_id(), "123");
  assert_eq!(did.path(), "/foo/bar/baz");
  assert_eq!(did.query(), Some("foo=bar"));
  assert_eq!(did.as_str(), "did:test:123/foo/bar/baz?foo=bar#frag");

  Ok(())
}

#[test]
fn test_query() -> Result<()> {
  // initial state
  let mut did: DID = did!("did:test:123");
  assert_eq!(did.fragment(), None);
  assert_eq!(did.as_str(), "did:test:123");

  // add query
  did.set_query(Some("query=true"));
  assert_eq!(did.query(), Some("query=true"));
  assert_eq!(did.as_str(), "did:test:123?query=true");

  // change query
  did.set_query(Some("other-query=true"));
  assert_eq!(did.query(), Some("other-query=true"));
  assert_eq!(did.as_str(), "did:test:123?other-query=true");

  // remove query
  did.set_query(None);
  assert_eq!(did.query(), None);
  assert_eq!(did.as_str(), "did:test:123");

  // set empty query
  did.set_query(Some(""));
  assert_eq!(did.query(), Some(""));
  assert_eq!(did.as_str(), "did:test:123?");

  // remove query
  did.set_query(None);
  assert_eq!(did.query(), None);
  assert_eq!(did.as_str(), "did:test:123");

  // does not modify fragment (1)
  did.set_fragment(Some("frag"));
  assert_eq!(did.query(), None);
  assert_eq!(did.fragment(), Some("frag"));
  assert_eq!(did.as_str(), "did:test:123#frag");

  // does not modify fragment (2)
  did.set_query(Some("query=true"));
  assert_eq!(did.query(), Some("query=true"));
  assert_eq!(did.fragment(), Some("frag"));
  assert_eq!(did.as_str(), "did:test:123?query=true#frag");

  // does not modify fragment (3)
  did.set_query(Some("foo="));
  assert_eq!(did.query(), Some("foo="));
  assert_eq!(did.fragment(), Some("frag"));
  assert_eq!(did.as_str(), "did:test:123?foo=#frag");

  // does not modify fragment (4)
  did.set_query(None);
  assert_eq!(did.query(), None);
  assert_eq!(did.fragment(), Some("frag"));
  assert_eq!(did.as_str(), "did:test:123#frag");

  // does not modify fragment (5)
  did.set_query(None);
  assert_eq!(did.query(), None);
  assert_eq!(did.fragment(), Some("frag"));
  assert_eq!(did.as_str(), "did:test:123#frag");

  // does not modify fragment (6)
  did.set_query(Some("other-query="));
  assert_eq!(did.query(), Some("other-query="));
  assert_eq!(did.fragment(), Some("frag"));
  assert_eq!(did.as_str(), "did:test:123?other-query=#frag");

  Ok(())
}

#[test]
fn test_fragment() -> Result<()> {
  // initial state
  let mut did: DID = did!("did:test:123/a/b/c?foo=bar");
  assert_eq!(did.fragment(), None);
  assert_eq!(did.as_str(), "did:test:123/a/b/c?foo=bar");

  // add fragment
  did.set_fragment(Some("frag"));
  assert_eq!(did.fragment(), Some("frag"));
  assert_eq!(did.as_str(), "did:test:123/a/b/c?foo=bar#frag");

  // change fragment
  did.set_fragment(Some("other-frag"));
  assert_eq!(did.fragment(), Some("other-frag"));
  assert_eq!(did.as_str(), "did:test:123/a/b/c?foo=bar#other-frag");

  // remove fragment
  did.set_fragment(None);
  assert_eq!(did.fragment(), None);
  assert_eq!(did.as_str(), "did:test:123/a/b/c?foo=bar");

  // set empty fragment
  did.set_fragment(Some(""));
  assert_eq!(did.fragment(), Some(""));
  assert_eq!(did.as_str(), "did:test:123/a/b/c?foo=bar#");

  // remove fragment
  did.set_fragment(None);
  assert_eq!(did.fragment(), None);
  assert_eq!(did.as_str(), "did:test:123/a/b/c?foo=bar");

  Ok(())
}

#[test]
fn test_relative() -> Result<()> {
  let did: DID = did!("did:example:123/a/b/c/d?q");

  // assert_eq!(did.join("g:h")?, "g:h");
  assert_eq!(did.join("g")?, "did:example:123/a/b/c/g");
  assert_eq!(did.join("./g")?, "did:example:123/a/b/c/g");
  assert_eq!(did.join("g/")?, "did:example:123/a/b/c/g/");
  // assert_eq!(did.join("/g")?, "did:example:123/a/g");
  // assert_eq!(did.join("//g")?, "did:example:123/g");
  assert_eq!(did.join("?y")?, "did:example:123/a/b/c/d?y");
  assert_eq!(did.join("g?y")?, "did:example:123/a/b/c/g?y");
  assert_eq!(did.join("#s")?, "did:example:123/a/b/c/d?q#s");
  assert_eq!(did.join("g#s")?, "did:example:123/a/b/c/g#s");
  assert_eq!(did.join("g?y#s")?, "did:example:123/a/b/c/g?y#s");
  // assert_eq!(did.join(";x")?, "did:example:123/a/b/c/;x");
  // assert_eq!(did.join("g;x")?, "did:example:123/a/b/c/g;x");
  // assert_eq!(did.join("g;x?y#s")?, "did:example:123/a/b/c/g;x?y#s");
  assert_eq!(did.join("")?, "did:example:123/a/b/c/d?q");
  assert_eq!(did.join(".")?, "did:example:123/a/b/c/");
  assert_eq!(did.join("./")?, "did:example:123/a/b/c/");
  assert_eq!(did.join("..")?, "did:example:123/a/b/");
  assert_eq!(did.join("../")?, "did:example:123/a/b/");
  assert_eq!(did.join("../g")?, "did:example:123/a/b/g");
  assert_eq!(did.join("../..")?, "did:example:123/a/");
  assert_eq!(did.join("../../")?, "did:example:123/a/");
  assert_eq!(did.join("../../g")?, "did:example:123/a/g");

  Ok(())
}
