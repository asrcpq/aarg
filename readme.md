# autoarg [=self]

dead simple argument parser:

* parameters are split by keys `--key`,

* simply call `aarg::parse()` to get a `HashMap<Key, Vec<Values>>`

* empty keys are for beginning of command

* same key will overwrite

* "--" starts positional values, raw strings are preserved

* example: see lib test

---

## todo

* values cannot start with "--"(like filename), maybe add arity to option
like "--option-3 arg1 arg2 arg3"
