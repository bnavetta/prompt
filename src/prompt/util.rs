use std::time::duration;

pub fn human_time(duration: duration::Duration) -> String
{
	let mut result = String::new();

	let days = duration.num_days();
	let hours = duration.num_hours() % 24;
	let minutes = duration.num_minutes() % 60;
	let seconds = duration.num_seconds() % 60;

	// I feel like there has to be a better way than to_string().as_slice() - can I just add the actual String?
	if days > 0
	{
		result = result.append(days.to_string().as_slice()).append("d ");
	}

	if hours > 0
	{
		result = result.append(hours.to_string().as_slice()).append("h ");
	}

	if minutes > 0
	{
		result = result.append(minutes.to_string().as_slice()).append("m ");
	}

	result.append(seconds.to_string().as_slice()).append("s")
}

#[cfg(test)]
mod tests {
	use super::human_time;

	use std::time::duration::Duration;

	#[test]
	fn test_human_time()
	{
		assert_eq!(human_time(Duration::days(10)).as_slice(), "10d 0s");
		assert_eq!(human_time(Duration::seconds(4)).as_slice(), "4s");
		assert_eq!(human_time(Duration::seconds(165392)).as_slice(), "1d 21h 56m 32s");
	}
}
