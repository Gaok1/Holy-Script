# Scriptures

Scriptures are pure data structures — named collections of typed fields with no behaviour of their own. Behaviour is added through [method salms](#method-salms).

---

## Declaration

```
scripture Point
    x of atom
    y of atom

scripture Person
    name of word
    age  of atom
```

- At least one field is required.
- Fields are declared in order; that order is used when instantiating.
- Field names must be unique within the scripture.

---

## Instantiation — `manifest`

```
let there p of Point be manifest Point praying 3, 4
let there u of Person be manifest Person praying "Gabriel", 30
```

Arguments are passed **in field declaration order**, separated by `,`.  
The last argument may use `and` instead of `,`:

```
let there p of Point be manifest Point praying 3 and 4
```

---

## Field access — `from`

```
let there px of atom be x from p
let there nm of word be name from u
```

`from` reads a single field by name. It does **not** mutate the value.

### Chain access

Fields can be chained when a field is itself a scripture:

```
scripture Address
    city of word

scripture Employee
    person of Person
    address of Address

let there city of word be city from address from emp
-- reads emp.address.city
```

### Inside a method salm — `its`

Within a `salm … upon SomeType` body, `its` refers to the instance the method was called on:

```
salm fullName upon Person reveals word
    reveal name from its plus " (age " plus hail word_of praying age from its plus ")"
```

---

## Method salms

A method salm is bound to a scripture type via `upon`. It is called on an instance of that type.

```
salm introduce upon Person reveals void
    hail proclaim praying "I am " plus name from its

-- call
hail introduce upon p
```

With parameters:

```
salm greetWith upon Person receiving greeting of word reveals void
    hail proclaim praying greeting plus ", " plus name from its plus "!"

hail greetWith upon p praying "Hail"
```

- Inside the body, `its` has the type of the bound scripture.
- `its` is read-only; you cannot reassign it.
- Method salms can also have type parameters (see [Generics](generics.md)).

---

## Generic scriptures

Scriptures can declare type parameters with `of`:

```
scripture Pair of A, B
    first  of A
    second of B

scripture Box of T
    value of T
```

Type parameters are abstract names resolved at the call site. They appear in field type annotations and are passed explicitly when instantiating.

```
let there b of Box of atom be manifest Box praying 42
let there p of Pair of atom, word be manifest Pair praying 1 and "x"
```

When a generic type appears before a comma that separates other arguments or type args, use `thus` to close it first:

```
-- Box<Stack<T>> — thus closes Stack<T> before the comma
let there x of Box of Stack of T thus be manifest Box praying s
```

See [Generics](generics.md) for full rules.

---

## Constructor convention

By convention, a salm with the same name as a scripture acts as a constructor:

```
scripture Point
    x of atom
    y of atom

salm Point receiving x of atom, y of atom reveals Point
    reveal manifest Point praying x, y

-- usage is cleaner
let there p of Point be hail Point praying 3, 4
```

---

## Default values

When declared without a value (`let there be`), a scripture variable is initialised to `void`. Accessing its fields before assigning a proper value will produce a runtime error.

```
let there be p of Point         -- p = void internally
p become manifest Point praying 0, 0   -- safe now
let there px of atom be x from p
```
