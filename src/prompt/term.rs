use std::fmt;

pub fn bold(text: &str) -> String {
	return format!("%B{}%b", text);
}

pub fn underline(text: &str) -> String {
	return format!("%U{}%u", text);
}

pub fn standout(text: &str) -> String {
	return format!("%S{}%s", text);
}

pub fn foreground(text: &str, color: Color) -> String {
	return format!("%F{{{color}}}{text}%f", color=color, text=text);
}

pub fn background(text: &str, color: Color) -> String {
	return format!("%K{{{color}}}{text}%k", color=color, text=text);
}

pub enum Color {
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
}

impl fmt::Show for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::FormatError> {
		match *self {
			Black   => "black",
			Red     => "red",
			Green   => "green",
			Yellow  => "yellow",
			Blue    => "blue",
			Magenta => "magenta",
			Cyan    => "cyan",
			White   => "white"
		}.fmt(f)
	}
}
