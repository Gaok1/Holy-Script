# Collections & Built-in Methods

Holy includes a built-in generic collection type:

```holy
legion of T
```

It is the primitive array-like collection in the language. Like other Holy types, it is explicit and strongly typed at runtime.

## Creating a `legion`

Use the built-in salm `legion`:

```holy
let there xs of legion of atom be hail legion praying 1, 2 and 3
let there names of legion of word be hail legion praying "Ava" and "Noah"
```

If declared without a value, a `legion` starts empty:

```holy
let there be xs of legion of atom
```

That is equivalent to an empty typed collection.

## Runtime type checks

`legion of atom` only accepts `atom` elements. `legion of word` only accepts `word` elements.

```holy
let there xs of legion of atom be hail legion praying 1 and 2
xs become hail push upon xs praying 3
```

This is valid.

```holy
let there xs of legion of atom be hail legion praying 1 and "oops"
```

This raises `TypeError`.

## Built-in methods on `legion`

Methods are invoked with the normal method-call syntax:

```holy
hail method upon target
hail method upon target praying args
```

So `legion` methods are not parser exceptions. They are runtime built-ins dispatched on the target value.

### `length`

Returns the number of elements.

```holy
let there xs of legion of atom be hail legion praying 10, 20 and 30
let there size of atom be hail length upon xs
```

### `is_empty`

Returns whether the collection has no elements.

```holy
let there be xs of legion of atom
let there empty of dogma be hail is_empty upon xs
```

### `at`

Returns the element at the given zero-based index.

```holy
let there xs of legion of atom be hail legion praying 10, 20 and 30
let there second of atom be hail at upon xs praying 1
```

If the index is invalid, Holy raises `IndexOutOfBounds`.

### `push`

Returns a new `legion` with one extra element appended.

```holy
let there xs of legion of atom be hail legion praying 1 and 2
xs become hail push upon xs praying 3
```

`push` does not mutate the inner storage in place. It follows the same value-style update model used elsewhere in Holy: the method returns a new value, and you reassign it.

## Built-in methods on `word`

Holy also provides built-in methods on `word`, using the same syntax.

### `length`

```holy
let there s of word be "holy"
let there size of atom be hail length upon s
```

### `is_empty`

```holy
let there s of word be ""
let there empty of dogma be hail is_empty upon s
```

### `at`

Returns the character at a zero-based index as a `word`.

```holy
let there s of word be "holy"
let there ch of word be hail at upon s praying 2
```

If the index is invalid, Holy raises `IndexOutOfBounds`.

## Notes

- Built-in methods are resolved by the interpreter, not by the parser.
- The syntax is the same as any other method call.
- `legion` is a built-in type available everywhere; it does not need a user declaration.
- `push` returns a new collection; it does not update the old value in place.
