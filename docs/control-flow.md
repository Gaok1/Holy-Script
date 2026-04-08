# Control Flow

---

## Conditional — `whether`

```
whether condition
    -- true branch (required)
```

```
whether x greater than 10
    hail proclaim praying "large"
otherwise so x is 10
    hail proclaim praying "exactly ten"
otherwise
    hail proclaim praying "small"
```

- `whether` evaluates the condition; if truthy, the block runs.
- `otherwise so condition` is an else-if branch (zero or more).
- `otherwise` is the else branch (optional, at most one).
- Branches are evaluated top to bottom; the first truthy branch runs.

### Truthiness

| Type         | Truthy when          |
|--------------|----------------------|
| `dogma`      | `blessed`            |
| `atom`       | not `0`              |
| `fractional` | not `0.0`            |
| `word`       | non-empty string     |
| `void`       | never                |
| scripture    | always               |
| covenant     | always               |

---

## Loop — `litany for`

Executes the body repeatedly as long as the condition is truthy.

```
let there i of atom be 1
litany for i no greater than 5
    hail proclaim praying hail word_of praying i
    i become i plus 1
```

The condition is re-evaluated before each iteration. If it is falsy on first check, the body never runs.

---

## Loop control

### `forsake` — break

Exits the current `litany for` immediately.

```
let there i of atom be 1
litany for blessed
    whether i is 5
        forsake
    i become i plus 1
```

### `ascend` — continue

Skips the rest of the current iteration and re-evaluates the condition.

```
let there i of atom be 0
litany for i lesser than 10
    i become i plus 1
    whether i remainder 2 is 0
        ascend          -- skip even numbers
    hail proclaim praying hail word_of praying i
```

`forsake` and `ascend` are only valid inside a `litany for` body. Using them outside a loop is a runtime error.

---

## Nested loops

`forsake` and `ascend` only affect the **innermost** `litany for`:

```
let there i of atom be 0
litany for i lesser than 3
    let there j of atom be 0
    litany for j lesser than 3
        whether j is 1
            forsake         -- exits the inner loop only
        j become j plus 1
    i become i plus 1
```
