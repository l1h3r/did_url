use core::ops::Range;
use core::ops::RangeFrom;
use core::ops::RangeTo;

use crate::did::DID;
use crate::error::Error;
use crate::error::Result;
use crate::input::Input;

#[derive(Clone, Debug)]
pub struct Core {
  pub(crate) method: u32,
  pub(crate) method_id: u32,
  pub(crate) path: u32,
  pub(crate) query: Option<u32>,
  pub(crate) fragment: Option<u32>,
}

impl Core {
  const fn new() -> Self {
    Self {
      method: 0,
      method_id: 0,
      path: 0,
      query: None,
      fragment: None,
    }
  }

  pub(crate) fn method<'a>(&self, data: &'a str) -> &'a str {
    self.slice(data, self.method..self.method_id - 1)
  }

  pub(crate) fn method_id<'a>(&self, data: &'a str) -> &'a str {
    self.slice(data, self.method_id..self.path)
  }

  pub(crate) fn path<'a>(&self, data: &'a str) -> &'a str {
    match (self.query, self.fragment) {
      (None, None) => self.slice(data, self.path..),
      (Some(index), _) | (None, Some(index)) => self.slice(data, self.path..index),
    }
  }

  pub(crate) fn query<'a>(&self, data: &'a str) -> Option<&'a str> {
    match (self.query, self.fragment) {
      (None, _) => None,
      (Some(query), None) => Some(self.slice(data, query + 1..)),
      (Some(query), Some(fragment)) => Some(self.slice(data, query + 1..fragment)),
    }
  }

  pub(crate) fn fragment<'a>(&self, data: &'a str) -> Option<&'a str> {
    self
      .fragment
      .map(|fragment| self.slice(data, fragment + 1..))
  }

  pub(crate) fn query_pairs<'a>(&self, data: &'a str) -> form_urlencoded::Parse<'a> {
    form_urlencoded::parse(self.query(data).unwrap_or_default().as_bytes())
  }

  fn slice<'a>(&self, data: &'a str, range: impl SliceExt) -> &'a str {
    range.slice(data)
  }

  /// Parse a DID URL adhering to the following format:
  ///
  ///   did                = "did:" method-name ":" method-specific-id
  ///   method-name        = 1*method-char
  ///   method-char        = %x61-7A / DIGIT
  ///   method-specific-id = *( *idchar ":" ) 1*idchar
  ///   idchar             = ALPHA / DIGIT / "." / "-" / "_"
  ///
  ///   did-url            = did path-abempty [ "?" query ] [ "#" fragment ]
  ///
  ///   path-abempty       = *( "/" segment )
  ///   segment            = *pchar
  ///   pchar              = unreserved / pct-encoded / sub-delims / ":" / "@"
  ///   unreserved         = ALPHA / DIGIT / "-" / "." / "_" / "~"
  ///   pct-encoded        = "%" HEXDIG HEXDIG
  ///   sub-delims         = "!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "="
  ///
  ///   query              = *( pchar / "/" / "?" )
  ///
  ///   fragment           = *( pchar / "/" / "?" )
  ///
  pub(crate) fn parse(input: impl AsRef<str>) -> Result<Self> {
    let mut this: Self = Self::new();
    let mut input: Input = Input::new(input.as_ref());

    this.parse_scheme(&mut input)?;
    this.parse_method(&mut input)?;
    this.parse_method_id(&mut input)?;
    this.parse_path(&mut input)?;
    this.parse_query(&mut input)?;
    this.parse_fragment(&mut input)?;

    Ok(this)
  }

  pub(crate) fn parse_relative(input: impl AsRef<str>) -> Result<Self> {
    let mut this: Self = Self::new();
    let mut input: Input = Input::new(input.as_ref());

    this.parse_path(&mut input)?;
    this.parse_query(&mut input)?;
    this.parse_fragment(&mut input)?;

    Ok(this)
  }

  fn parse_scheme(&mut self, input: &mut Input) -> Result<()> {
    if input.exhausted() {
      return Err(Error::InvalidScheme);
    }

    if !matches!(input.take(3), Some(DID::SCHEME)) {
      return Err(Error::InvalidScheme);
    }

    if matches!(input.next(), Some(':')) {
      Ok(())
    } else {
      Err(Error::InvalidScheme)
    }
  }

  fn parse_method(&mut self, input: &mut Input) -> Result<()> {
    self.method = input.index();

    loop {
      match input.peek() {
        Some(':') | None => break,
        Some('a'..='z') => {}
        Some('0'..='9') => {}
        _ => return Err(Error::InvalidMethodName),
      }

      input.next();
    }

    if self.method == input.index() {
      return Err(Error::InvalidMethodName);
    }

    input.next();

    Ok(())
  }

  fn parse_method_id(&mut self, input: &mut Input) -> Result<()> {
    self.method_id = input.index();

    loop {
      match input.peek() {
        Some('/' | '?' | '#') | None => break,
        Some('a'..='z' | 'A'..='Z') => {}
        Some('0'..='9') => {}
        Some('.' | '-' | '_') => {}
        Some(':') if input.index() > self.method_id => {}
        _ => return Err(Error::InvalidMethodId),
      }

      input.next();
    }

    // TODO: Return Err if colon is last parsed char

    if self.method_id == input.index() {
      return Err(Error::InvalidMethodId);
    }

    Ok(())
  }

  fn parse_path(&mut self, input: &mut Input) -> Result<()> {
    self.path = input.index();

    if matches!(input.peek(), Some('?' | '#') | None) {
      return Ok(());
    }

    loop {
      match input.peek() {
        Some('?' | '#') | None => break,
        Some('a'..='z' | 'A'..='Z') => {}
        Some('0'..='9') => {}
        Some('-' | '.' | '_' | '~') => {}
        Some('!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '=') => {}
        Some(':' | '@') => {}
        Some('/') => {}
        _ => return Err(Error::InvalidPath),
      }

      input.next();
    }

    Ok(())
  }

  fn parse_query(&mut self, input: &mut Input) -> Result<()> {
    if matches!(input.peek(), Some('#') | None) {
      return Ok(());
    }

    if matches!(input.peek(), Some('?')) {
      input.next();
    } else {
      return Err(Error::InvalidQuery);
    }

    self.query = Some(input.index() - 1);

    loop {
      match input.peek() {
        Some('#') | None => break,
        Some('a'..='z' | 'A'..='Z') => {}
        Some('0'..='9') => {}
        Some('-' | '.' | '_' | '~') => {}
        Some('!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '=') => {}
        Some(':' | '@') => {}
        Some('/' | '?') => {}
        _ => return Err(Error::InvalidQuery),
      }

      input.next();
    }

    Ok(())
  }

  fn parse_fragment(&mut self, input: &mut Input) -> Result<()> {
    if input.exhausted() {
      return Ok(());
    }

    if matches!(input.peek(), Some('#')) {
      input.next();
    } else {
      return Err(Error::InvalidFragment);
    }

    self.fragment = Some(input.index() - 1);

    loop {
      match input.peek() {
        None => break,
        Some('a'..='z' | 'A'..='Z') => {}
        Some('0'..='9') => {}
        Some('-' | '.' | '_' | '~') => {}
        Some('!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '=') => {}
        Some(':' | '@') => {}
        Some('/' | '?') => {}
        _ => return Err(Error::InvalidFragment),
      }

      input.next();
    }

    Ok(())
  }
}

// =============================================================================
//
// =============================================================================

pub trait SliceExt {
  fn slice<'a>(&self, string: &'a str) -> &'a str;
}

impl SliceExt for Range<u32> {
  fn slice<'a>(&self, string: &'a str) -> &'a str {
    &string[self.start as usize..self.end as usize]
  }
}

impl SliceExt for RangeFrom<u32> {
  fn slice<'a>(&self, string: &'a str) -> &'a str {
    &string[self.start as usize..]
  }
}

impl SliceExt for RangeTo<u32> {
  fn slice<'a>(&self, string: &'a str) -> &'a str {
    &string[..self.end as usize]
  }
}
