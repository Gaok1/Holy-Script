# Holy Language

VS Code extension for [Holy Script](https://github.com/Gaok1/holy-script) — an interpreted language with archaic/biblical English syntax, implemented in Rust.

## Features

- Syntax highlighting for `.holy` files
- Code snippets for all language constructs (`salm`, `scripture`, `covenant`, `sin`, `whether`, `litany`, `confess`, `discern`, `grace`, `verdict`, `testament`, and more)
- File icon for `.holy` files (enable via **File → Preferences → File Icon Theme → Holy Language Icons**)
- **Run button** (▶) in the editor title bar — executes the current file with `holy`

## Requirements

Install the interpreter and add it to your PATH:

```sh
git clone https://github.com/Gaok1/holy-script
cd holy-script
cargo install --path .
```

> If you have the **Code Runner** extension installed, it will also be configured automatically to run `.holy` files using `holy`.

## Usage

Open any `.holy` file and click the **▶** button in the top-right corner of the editor.  
The output appears in a terminal panel named **Holy**.

## Language overview

```holy
-- line comment

testament MathUtils          -- import from MathUtils.holy (same directory)

scripture Point              -- struct
    x of atom
    y of atom

sin OutOfBounds              -- exception type
    message of word

covenant Direction           -- enum
    North
    South

salm add receiving a of atom, b of atom reveals atom
    reveal a plus b

-- variables
let there p of Point be manifest Point praying 3, 4
let there r of atom be hail add praying 1, 2
let there xs of legion of atom be hail legion praying 1, 2 and 3

-- built-in salms
hail proclaim praying "Hello!"
let there line of word be hail inquire
let there n    of atom be hail atom_of praying line
let there ok   of verdict of word and word be hail read_file praying "data.txt"

-- conditionals
whether r is 3
    hail proclaim praying "correct"
otherwise
    hail proclaim praying "wrong"

-- loop with break/continue
litany for r no greater than 10
    r become r plus 1
    whether r remainder 2 is 0
        ascend         -- continue
    whether r is 9
        forsake        -- break

-- pattern matching
discern ok
    as righteous bearing content
        hail proclaim praying content
    as condemned bearing reason
        hail proclaim praying reason

-- exceptions
confess
    transgress OutOfBounds praying "index too large"
answer for OutOfBounds as err
    hail proclaim praying message from err
absolve
    hail proclaim praying "done"

amen
```

## License

MIT
