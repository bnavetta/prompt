use std::io::Error;
use std::io::prelude::*;

/// ZSHEscaper is a Write implementation which adds ZSH prompt literal escapes around formatting codes. These are
/// required so that the formatting codes don't count towards the size of the prompt, which affects where ZSH positions
/// the cursor, completions, and other stuff.
///
/// See the [zshmisc](https://www.manpagez.com/man/1/zshmisc/) man pages (search for `%{...%}`) and this [StackOverflow
/// question](https://stackoverflow.com/questions/7957435/zsh-auto-complete-screws-up-command-name/10644062#10644062).
pub struct ZSHEscaper<W: Write> {
    inner: W,
    in_escape: bool
}

impl <W: Write> ZSHEscaper<W> {
    pub fn new(inner: W) -> Self {
        ZSHEscaper {
            inner,
            in_escape: false,
        }
    }
}

impl <W: Write> Write for ZSHEscaper<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        for b in buf.iter() {
            match (b, self.in_escape) {
                (b'm', true) => {
                    self.in_escape = false;
                    self.inner.write_all(b"m%}")?;
                },
                (b'\x1B', false) => {
                    self.in_escape = true;
                    self.inner.write_all(b"%{\x1B")?;
                },
                (b, _) => {
                    self.inner.write_all(&[*b])?;
                }
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.inner.flush()
    }
}

// It would have been nicer to implement this as a wrapper around some of the ansi_term components, but that would
// require more access to the crate internals than it provides. Basically, this would need to reimplement/wrap
// ANSIGenericStrings::write_to_any. 

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use super::*;

    #[test]
    fn test_basic() {
        let mut inner = Vec::new();
        let mut w = ZSHEscaper::new(&mut inner);

        write!(w, "\x1B[31m");
        assert_eq!(inner, b"%{\x1B[31m%}");
    }
}