use std::collections::HashMap;

pub type Adict = HashMap<String, Vec<String>>;

fn parse_strs<T>(args: T) -> Adict
	where T: Iterator<Item = String>
{
	let mut pending_key = String::new();
	let mut pending_value = Vec::new();
	let mut result = HashMap::new();
	for arg in args {
		if let Some(key) = arg.strip_prefix("--") {
			result.insert(pending_key, pending_value);
			pending_key = key.to_string();
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
		let string = "foo bar --foo bar --bar foo --foo foobar --foobar --foobar";
		let mut truth = HashMap::default();
		truth.insert("".to_string(), vec!["foo".to_string(), "bar".to_string()]);
		truth.insert("foo".to_string(), vec!["foobar".to_string()]);
		truth.insert("bar".to_string(), vec!["foo".to_string()]);
		truth.insert("foobar".to_string(), vec![]);
		let parsed = parse_strs(string.split_whitespace().map(|x| x.to_string()));
		assert_eq!(parsed, truth);
	}
}
