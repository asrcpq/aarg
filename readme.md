# autoarg [=self]

dead simple argument parser:

* parameters are split by keys `--key`,

* simply call `aarg::parse()` to get a `HashMap<Key, Vec<Values>>`

* same key will overwrite

* example: see lib test
