use std::io;
use std::borrow::Cow;

pub use self::Attribute::*;
pub use self::Color::*;

#[derive(Debug,Clone)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
    Other(&'static str)
}

impl Color {
    fn to_raw(&self) -> &'static str {
        match *self {
            Black         => "black",
            Red           => "red",
            Green         => "green",
            Yellow        => "yellow",
            Blue          => "blue",
            Magenta       => "magenta",
            Cyan          => "cyan",
            White         => "white",
            Default       => "default",
            Other(color)  => color,
        }
    }
}

#[derive(Debug,Clone)]
pub enum Attribute {
    Bold,
    Underline,
    Standout,
    ForegroundColor(Color),
    BackgroundColor(Color)
}

impl Attribute {
    fn begin<'a>(&self) -> Cow<'a, str>{
        match *self {
            Bold                       => "%B".into(),
            Underline                  => "%U".into(),
            Standout                   => "%S".into(),
            ForegroundColor(ref color) => format!("%F{{{}}}", color.to_raw()).into(),
            BackgroundColor(ref color) => format!("%K{{{}}}", color.to_raw()).into(),
        }
    }

    fn end(&self) -> &'static str {
        match *self {
            Bold               => "%b",
            Underline          => "%u",
            Standout           => "%s",
            ForegroundColor(_) => "%f",
            BackgroundColor(_) => "%k",
        }
    }
}

pub fn style() -> StyleBuilder {
    StyleBuilder {
        attributes: Vec::new()
    }
}

pub struct StyleBuilder {

    attributes: Vec<Attribute>,
}

impl StyleBuilder {
    pub fn attr(&mut self, attr: Attribute) -> &mut StyleBuilder {
        self.attributes.push(attr);
        self
    }

    pub fn fg(&mut self, color: Color) -> &mut StyleBuilder {
        self.attr(ForegroundColor(color))
    }

    pub fn bg(&mut self, color: Color) -> &mut StyleBuilder {
        self.attr(BackgroundColor(color))
    }

    pub fn go(self) -> StyleWriter {
        StyleWriter::new(self.attributes)
    }
}

pub struct StyleWriter {
    attributes: Vec<Attribute>,
}

impl StyleWriter {
    fn new(attributes: Vec<Attribute>) ->StyleWriter {
        print!("%{{");
        for attr in attributes.iter() {
            print!("{}", attr.begin());
        }
        print!("%}}");

        StyleWriter {
            attributes: attributes
        }
    }
}

impl io::Write for StyleWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
       io::stdout().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
       io::stdout().flush()
    }
}

impl Drop for StyleWriter {
    fn drop(&mut self) {
        print!("%{{");
        let mut attrs = self.attributes.to_vec();
        attrs.reverse();
        for attr in attrs {
            print!("{}", attr.end())
        }
        print!("%}}");
    }
}

// let styles = StyledTerminal::new();

// {
//     let w = styles.style().attr(Bold).fg(Red).go();
//     write!(w, "Hello, World!");

//     {
//         let w = styles.fg(Red); // helper for common case
//         write!(w, "This is red and bold");
//     }
// }
