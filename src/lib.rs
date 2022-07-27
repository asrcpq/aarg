use std::collections::HashMap;

pub type Adict = HashMap<String, Vec<String>>;

fn parse_strs<T>(args: T) -> Adict
	where T: Iterator<Item = String>
{
	let mut pending_key = String::new();
	let mut pending_value = Vec::new();
	let mut result = HashMap::new();
	let mut positional_flag = false;
	for arg in args {
		if arg.starts_with("--") && !positional_flag {
			if arg == "--" {
				positional_flag = true;
			}
			result.insert(pending_key, pending_value);
			pending_key = arg;
			pending_value = Vec::new();
			continue;
		}
		pending_value.push(arg);
	}
	result.insert(pending_key, pending_value);
	result
	
}

pub fn parse() -> Adict {
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

		let string = "foo bar --foo bar --foo foo foobar -- -- --foobar";
		let mut truth = HashMap::default();
		truth.insert("".to_string(), vs!["foo", "bar"]);
		truth.insert("--foo".to_string(), vs!["foo", "foobar"]);
		truth.insert("--".to_string(), vs!["--", "--foobar"]);
		let parsed = parse_strs(string.split_whitespace().map(|x| x.to_string()));
		assert_eq!(parsed, truth);
	}
}
