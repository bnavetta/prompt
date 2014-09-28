use std::time::duration::Duration;
use super::term;
use super::term::Color;

pub static COLOR_SUCCESS: Color = term::Blue;
pub static COLOR_FAILURE: Color = term::Red;
pub static COLOR_EXEC_TIME: Color = term::Yellow;

#[inline]
pub fn max_exec_time() -> Duration {
	Duration::seconds(8)
}
