use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Field {
  Authority,
  Fragment,
  MethodId,
  MethodName,
  Path,
  Query,
  Scheme,
}

impl Field {
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Authority => "Authority",
      Self::Fragment => "Fragment",
      Self::MethodId => "Method Id",
      Self::MethodName => "Method Name",
      Self::Path => "Path",
      Self::Query => "Query",
      Self::Scheme => "Scheme",
    }
  }
}

impl Display for Field {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_str(self.as_str())
  }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
  ParseError(Field),
  JoinError(Field),
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    match self {
      Self::ParseError(field) => {
        f.write_fmt(format_args!("Parse Error: Invalid {}", field.as_str()))?;
      }
      Self::JoinError(field) => {
        f.write_fmt(format_args!("Join Error: Invalid {}", field.as_str()))?;
      }
    }

    Ok(())
  }
}

#[cfg(feature = "std")]
impl ::std::error::Error for Error {}
