use std::collections::HashMap;

pub type Adict = HashMap<String, Vec<String>>;

#[derive(Debug, PartialEq)]
pub enum AargError {
	OpenQuote,
	BrokenEscape,
}

fn parse_strs<T>(args: T) -> Result<Adict, AargError>
	where T: Iterator<Item = String>
{
	let mut pending_key = String::new();
	let mut pending_value = Vec::new();
	let mut result = HashMap::new();
	let mut positional_flag = false;
	for mut arg in args {
		if positional_flag {
			pending_value.push(arg);
			continue
		}
		if arg.starts_with("--") {
			if arg == "--" {
				positional_flag = true;
			}
			result.insert(pending_key, pending_value);
			pending_key = arg;
			pending_value = Vec::new();
			continue;
		} else if arg.starts_with('"') {
			let mut iter = arg.chars();
			iter.next();
			if Some('"') != iter.next_back() {
				return Err(AargError::OpenQuote);
			}
			let mut unescaped = Vec::new();
			let mut escape_flag = false;
			while let Some(ch) = iter.next() {
				if ch == '\\' && !escape_flag {
					escape_flag = true;
				} else {
					unescaped.push(ch);
					escape_flag = false;
				}
			}
			if escape_flag == true {
				return Err(AargError::BrokenEscape);
			}
			arg = unescaped.into_iter().collect();
		}
		pending_value.push(arg);
	}
	result.insert(pending_key, pending_value);
	Ok(result)
	
}

pub fn parse() -> Result<Adict, AargError> {
	parse_strs(std::env::args())
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_parse_strs() {
		macro_rules! vs {
			($($x:expr),*) => (vec![$($x.to_string()),*]);
		}

		let string = "foo bar --foo bar --foo \"--\\\\\\\"foo\" foobar -- -- --foobar";
		let mut truth = HashMap::default();
		truth.insert("".to_string(), vs!["foo", "bar"]);
		truth.insert("--foo".to_string(), vs!["--\\\"foo", "foobar"]);
		truth.insert("--".to_string(), vs!["--", "--foobar"]);
		let parsed = parse_strs(string.split_whitespace().map(|x| x.to_string())).unwrap();
		assert_eq!(parsed, truth);

		let string = "foo \"foo\\\"";
		let parsed = parse_strs(string.split_whitespace().map(|x| x.to_string())).unwrap_err();
		assert_eq!(parsed, AargError::BrokenEscape);

		let string = "foo \"foo";
		let parsed = parse_strs(string.split_whitespace().map(|x| x.to_string())).unwrap_err();
		assert_eq!(parsed, AargError::OpenQuote);
	}
}
