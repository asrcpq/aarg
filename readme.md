# autoarg [=self]

dead simple argument parser:

* parameters are split by keys `--key`,

* simply call `aarg::parse()` to get a `HashMap<Key, Vec<Values>>`

* same key will overwrite

* example: see lib test

---

## todo

* distinguish begin and last(split by "--"), currently they are both indexed by empty string

* values cannot start with "--"(like filename), maybe add arity to option
like "--option-3 arg1 arg2 arg3"
