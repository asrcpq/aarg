# autoarg

dead simple argument parser:
parameters are split by keys `--key`,
simply call `aarg::parse().unwrap()`
to get a `HashMap<Key, Vec<Values>>`

## example

```
foo bar --foo foo bar --foo "foo bar" -- --foo
```

result:

```
{
	"": ["foo", "bar"],
	"--foo": ["foo bar"],
	"--": ["--foo"],
}
```
