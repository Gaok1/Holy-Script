# Modules

Holy Lang supports modules through the `testament` declaration. Module imports appear at the very top of a program, before any top-level declarations or statements.

---

## Importing a module — `testament`

```
testament MathUtils
```

Imports all public symbols (scriptures, covenants, salms, sins) from the `MathUtils` module into the current program's scope.

---

## Selective import — `revealing`

```
testament Collections revealing Stack, Queue
testament MathUtils revealing add, multiply
```

Only the listed symbols are imported. All other symbols in that module remain inaccessible.

---

## Multiple imports

```
testament MathUtils
testament Collections revealing Stack
testament Strings revealing trim, split
```

Imports are processed in declaration order.

---

## Module declaration syntax

```
testament ModuleName
testament ModuleName revealing Symbol1, Symbol2, Symbol3
```

- `ModuleName` is an identifier.
- `revealing` is optional.
- The symbol list after `revealing` is comma-separated.

---

## Notes

- Modules correspond to other `.holy` files resolved by the runtime.
- Circular imports are not supported.
- All `testament` declarations must come before any `scripture`, `sin`, `covenant`, `salm`, or statement.
