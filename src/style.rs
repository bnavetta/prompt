use std::io::{Write, Result};
use term;

pub use self::Attribute::*;
pub use self::Color::*;

pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Other(u16)
}

impl Color {
    fn to_raw(&self) -> term::color::Color {
        match *self {
            Black         => term::color::BLACK,
            Red           => term::color::RED,
            Green         => term::color::GREEN,
            Yellow        => term::color::YELLOW,
            Blue          => term::color::BLUE,
            Magenta       => term::color::MAGENTA,
            Cyan          => term::color::CYAN,
            White         => term::color::WHITE,
            BrightBlack   => term::color::BRIGHT_BLACK,
            BrightRed     => term::color::BRIGHT_RED,
            BrightGreen   => term::color::BRIGHT_GREEN,
            BrightYellow  => term::color::BRIGHT_YELLOW,
            BrightBlue    => term::color::BRIGHT_BLUE,
            BrightMagenta => term::color::BRIGHT_MAGENTA,
            BrightCyan    => term::color::BRIGHT_CYAN,
            BrightWhite   => term::color::BRIGHT_WHITE,
            Other(color)  => color as term::color::Color
        }
    }
}

pub enum Attribute {
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Standout,
    Reverse,
    ForegroundColor(Color),
    BackgroundColor(Color)
}

impl Attribute {
    fn as_attr(&self) -> term::Attr {
        match *self {
            Bold                   => term::Attr::Bold,
            Dim                    => term::Attr::Dim,
            Italic                 => term::Attr::Italic(true),
            Underline              => term::Attr::Underline(true),
            Blink                  => term::Attr::Blink,
            Standout               => term::Attr::Standout(true),
            Reverse                => term::Attr::Reverse,
            ForegroundColor(ref color) => term::Attr::ForegroundColor(color.to_raw()),
            BackgroundColor(ref color) => term::Attr::BackgroundColor(color.to_raw()),
        }
    }
}

pub struct StyledTerminal {
    term: Box<term::StdoutTerminal>,
}

impl StyledTerminal {
    pub fn new() -> Option<StyledTerminal> {
        term::stdout().map(|t| StyledTerminal{ term: t })
    }

    pub fn style<'a>(&'a mut self) -> StyleBuilder<'a> {
        StyleBuilder {
            term: &mut *self.term,
            attributes: Vec::new(),
        }
    }
}

pub struct StyleBuilder<'term> {
    term: &'term mut term::StdoutTerminal,
    attributes: Vec<Attribute>,
}

impl<'term> StyleBuilder<'term> {
    pub fn attr(&'term mut self, attr: Attribute) -> &'term mut StyleBuilder {
        self.attributes.push(attr);
        self
    }

    pub fn fg(&'term mut self, color: Color) -> &'term mut StyleBuilder {
        self.attr(ForegroundColor(color))
    }

    pub fn bg(&'term mut self, color: Color) -> &'term mut StyleBuilder {
        self.attr(BackgroundColor(color))
    }

    pub fn go(self) -> Result<StyleWriter<'term>> {
        StyleWriter::new(self.term, self.attributes)
    }
}

pub struct StyleWriter<'term> {
    term: &'term mut term::StdoutTerminal
}

impl<'term> StyleWriter<'term> {
    fn new(term: &'term mut term::StdoutTerminal, attributes: Vec<Attribute>) -> Result<StyleWriter<'term>> {
        for attr in attributes {
            try!(term.attr(attr.as_attr()));
        }

        Ok(StyleWriter {
            term: term
        })
    }
}

impl <'term> Write for StyleWriter<'term> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.term.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.term.flush()
    }
}

impl<'term> Drop for StyleWriter<'term> {
    fn drop(&mut self) {
        self.term.reset();
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
