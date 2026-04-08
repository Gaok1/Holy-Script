# Types & Variables

Holy is strongly typed: values must match their declared types. Type checks happen at runtime, but the language does not allow arbitrary type changes once a variable, field, parameter, or return type has been declared.

## Primitive types

| Keyword      | Meaning              | Default       | Literal examples      |
|--------------|----------------------|---------------|-----------------------|
| `atom`       | integer (i64)        | `0`           | `42`, `-7`, `0`       |
| `fractional` | float (f64)          | `0.0`         | `3.14`, `-0.5`        |
| `word`       | string (UTF-8)       | `""`          | `"hello"`, `""`       |
| `dogma`      | boolean              | `forsaken`    | `blessed`, `forsaken` |
| `void`       | no value             | —             | —                     |
| `legion of T`| typed collection     | empty legion  | `hail legion praying 1, 2 and 3` |

`blessed` is `true`; `forsaken` is `false`.

---

## Variables

### Declaration without a value

Zero-initialises to the type's default.

```holy
let there be x of atom          -- x = 0
let there be msg of word        -- msg = ""
let there be flag of dogma      -- flag = forsaken
```

### Declaration with a value

```holy
let there x of atom be 42
let there greeting of word be "Hail, world!"
let there active of dogma be blessed
let there ratio of fractional be 3.14
```

### `legion of T`

`legion of T` is Holy's built-in typed collection. Its element type is always written explicitly.

```holy
let there xs of legion of atom be hail legion praying 1, 2 and 3
let there names of legion of word be hail legion praying "Ava" and "Noah"
```

If declared without a value, a `legion` starts empty:

```holy
let there be xs of legion of atom
```

Runtime type checks still apply, so a `legion of atom` cannot receive a `word`.

### Reassignment

```holy
x become x plus 1
greeting become "Farewell"
```

The variable must already be declared. The new value must match the declared type.

---

## Operators

### Arithmetic

| Expression        | Meaning              | Works on              |
|-------------------|----------------------|-----------------------|
| `a plus b`        | addition             | `atom`, `fractional`, `word` (concat) |
| `a minus b`       | subtraction          | `atom`, `fractional`  |
| `a times b`       | multiplication       | `atom`, `fractional`  |
| `a over b`        | division             | `atom`, `fractional`  |
| `a remainder b`   | modulo               | `atom`                |
| `negate a`        | unary minus          | `atom`, `fractional`  |

Division of two `atom` values performs integer division.  
Mixing `atom` and `fractional` in an expression promotes both to `fractional`.

```holy
let there x of atom be 10 remainder 3       -- 1
let there y of fractional be 1 over 2       -- 0  (integer division!)
let there z of fractional be 1.0 over 2     -- 0.5
let there s of word be "Holy" plus " Lang"  -- "Holy Lang"
```

### Comparison

| Expression             | Meaning |
|------------------------|---------|
| `a is b`               | `==`    |
| `a is not b`           | `!=`    |
| `a greater than b`     | `>`     |
| `a lesser than b`      | `<`     |
| `a no greater than b`  | `<=`    |
| `a no lesser than b`   | `>=`    |

Comparisons do **not** chain. `a greater than b greater than c` is a syntax error.

`is` / `is not` work on any type. Ordered comparisons work on `atom` and `fractional`.

---

## Operator precedence

From lowest to highest:

| Level | Operators                         |
|-------|-----------------------------------|
| 1     | comparisons (`is`, `greater`, …)  |
| 2     | `plus`, `minus`                   |
| 3     | `times`, `over`, `remainder`      |
| 4     | `negate` (unary)                  |
| 5     | atoms (literals, calls, variables)|

```holy
-- without grouping: negate binds tightest
negate 10 remainder 3    -- (-10) % 3 = -1

-- 2 plus 3 times 4 = 2 + (3 * 4) = 14
let there x of atom be 2 plus 3 times 4
```

---

## Expression grouping — `after … thus`

`after` opens a grouped sub-expression; `thus` closes it. Equivalent to parentheses.

```holy
after 3 times 5 thus              -- (3 * 5) = 15
5 plus after 3 times 2 thus       -- 5 + (3 * 2) = 11
after 2 plus 3 thus times 4       -- (2 + 3) * 4 = 20
```

`thus` without a matching `after` (or outside a valid context) is a syntax error.

For ambiguity cases that combine `after` with nested calls or generic types, see [Disambiguation with `thus` and `after`](nesting.md).

---

## Type coercion

There is no implicit type coercion. Use built-in salms to convert:

```holy
-- word → atom
let there n of atom be hail atom_of praying "42"

-- anything → word
let there s of word be hail word_of praying 99
let there b of word be hail word_of praying blessed
```

See [Salms — Built-in salms](salms.md#built-in-salms) for the full list.
