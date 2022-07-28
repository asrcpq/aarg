# autoarg

dead simple argument parser:

* parameters are split by keys `--key`,

* simply call `aarg::parse().unwrap()`
to get a `HashMap<Key, Vec<Values>>`

* empty key are for beginning of command

* same key will overwrite

* "--" starts positional values, raw strings are preserved

* quoted string will use backslash for escape, will not be considered as a key

* example: see lib test
