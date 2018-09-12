use std::{ffi::OsStr, fmt, iter, str};

use crate::parse::{Parse, ParseError};

/// Invocation of an executable command.
///
/// This is called `Invoke` because the name `Cmd` was already taken.
#[derive(Clone, Debug)]
pub struct Invoke<'a> {
    /// The command to invoke.
    pub command: &'a OsStr,
    /// Zero or more arguments to pass to the command.
    pub args: iter::Map<str::SplitWhitespace<'a>, fn(&'a str) -> &'a OsStr>,
}

// ===== impl Invoke =====

impl<'a> Parse<'a> for Invoke<'a> {
    type Error = ();
    fn parse_from(s: &'a str) -> Result<Self, ParseError<Self::Error>> {
        // TODO(eliza): quoted args!
        let mut args = s
            .trim()
            .split_whitespace()
            .map(OsStr::new as fn(&'a str) -> &'a OsStr);
        let command = args.next().ok_or(ParseError::NoInput)?;
        let command = Invoke { command, args };
        Ok(command)
    }
}

impl<'a> fmt::Display for Invoke<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.command.to_string_lossy())?;
        for ref arg in self.args.clone() {
            write!(f, " {}", arg.to_string_lossy())?;
        }
        Ok(())
    }
}
