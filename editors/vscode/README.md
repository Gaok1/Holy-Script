# Holy Language

VS Code extension for [Holy Lang](https://github.com/gaok1/holy-lang) — an interpreted language with archaic/biblical English syntax, implemented in Rust.

## Features

- Syntax highlighting for `.holy` files
- Code snippets
- File icon for `.holy` files (enable via **File → Preferences → File Icon Theme → Holy Language Icons**)
- **Run button** (▶) in the editor title bar — executes the current file with `holy`

## Requirements

Install the interpreter and add it to your PATH:

```sh
git clone https://github.com/gaok1/holy-lang
cd holy-lang
cargo install --path .
```

> If you have the **Code Runner** extension installed, it will also be configured automatically to run `.holy` files using `holy`.

## Usage

Open any `.holy` file and click the **▶** button in the top-right corner of the editor.  
The output appears in a terminal panel named **Holy**.

## Language overview

```
-- line comment

scripture Point       -- struct
    x of atom
    y of atom

sin OutOfBounds       -- exception type
    message of word

covenant Direction    -- enum
    North
    South

salm add receiving a of atom, b of atom reveals atom
    reveal a plus b

let there p of Point be manifest Point praying 3, 4
let there r of atom be hail add praying 1, 2

whether r is 3
    hail proclaim praying "correct"

litany for r no greater than 10
    r become r plus 1
    whether r is 7
        forsake        -- break
    whether r remainder 2 is 0
        ascend         -- continue

discern hail direction
    as North
        hail proclaim praying "going north"
    otherwise
        hail proclaim praying "other direction"

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
