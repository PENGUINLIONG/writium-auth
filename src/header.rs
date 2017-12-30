use std::fmt::Write;
use std::fmt::{Display, Formatter, Result as FormatResult};
use writium::hyper::header::{Header, Formatter as HyperFormatter, Raw};
use writium::hyper::{Error as HyperError, Result as HyperResult};

#[derive(Clone)]
pub struct Challenge {
    scheme: &'static str,
    params: Vec<(String, String)>,
}
impl Challenge {
    pub fn new(scheme: &'static str) -> Challenge {
        Challenge {
            scheme: scheme,
            params: Vec::new(),
        }
    }
    pub fn with_param(mut self, key: &str, val: &str) -> Challenge {
        self.params.push((key.to_string(), val.to_string()));
        self
    }
}
impl Display for Challenge {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        f.write_str(&self.scheme)?;
        let mut it = self.params.iter();
        if let Some(&(ref key, ref val)) = it.next() {
            f.write_char(' ')?;
            f.write_str(&format!("{}=\"{}\"", key, val))?;
            for &(ref key, ref val) in it {
                f.write_str(&format!(",{}=\"{}\"", key, val))?;
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct WwwAuthenticate {
    challenges: Vec<Challenge>,
}
impl WwwAuthenticate {
    pub fn new() -> WwwAuthenticate {
        WwwAuthenticate {
            challenges: Vec::new(),
        }
    }
    pub fn with_challenge(mut self, challenge: Challenge) -> WwwAuthenticate {
        self.challenges.push(challenge);
        self
    }
    pub fn with_challenges(mut self, challenges: Vec<Challenge>) -> WwwAuthenticate {
        self.challenges = challenges;
        self
    }
}
impl Header for WwwAuthenticate {
    fn header_name() -> &'static str {
        "WWW-Authenticate"
    }
    /// This header should never present in a request.
    fn parse_header(_raw: &Raw) -> HyperResult<Self> {
        Err(HyperError::Header)
    }
    #[inline]
    fn fmt_header(&self, f: &mut HyperFormatter) -> FormatResult {
        f.fmt_line(self)
    }
}
impl Display for WwwAuthenticate {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        let mut it = self.challenges.iter();
        if let Some(ch) = it.next() {
            ch.fmt(f)?;
            for ch in it {
                f.write_char(',')?;
                ch.fmt(f)?;
            }
        }
        Ok(())
    }
}
