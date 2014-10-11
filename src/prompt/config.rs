use std::time::duration::Duration;
use super::term;
use super::term::Color;

pub const COLOR_SUCCESS: Color = term::Green;
pub const COLOR_FAILURE: Color = term::Red;
pub const COLOR_EXEC_TIME: Color = term::Magenta;
pub const COLOR_USER: Color = term::Cyan;
pub const COLOR_HOST: Color = term::Yellow;
pub const COLOR_PATH: Color = term::Blue;

#[inline]
pub fn max_exec_time() -> Duration {
	Duration::seconds(8)
}
