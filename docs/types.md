# Tipos e Variáveis

Holy é fortemente tipado: todo valor tem um tipo declarado e o runtime recusa valores que não o correspondam. Não há conversão implícita.

---

## Tipos primitivos

| Tipo         | Significado          | Padrão        | Exemplos de literal     |
|--------------|----------------------|---------------|-------------------------|
| `atom`       | inteiro (i64)        | `0`           | `42`, `-7`, `0`         |
| `fractional` | decimal (f64)        | `0.0`         | `3.14`, `-0.5`          |
| `word`       | texto (UTF-8)        | `""`          | `"hello"`, `""`         |
| `dogma`      | booleano             | `forsaken`    | `blessed`, `forsaken`   |
| `void`       | ausência de valor    | —             | —                       |
| `legion of T`| coleção tipada       | vazio         | ver [Coleções](collections.md) |

`blessed` = verdadeiro · `forsaken` = falso

---

## Variáveis

### Declaração sem valor inicial

Inicializa com o padrão do tipo (`0`, `""`, `forsaken`, etc.):

```holy
let there be x of atom          -- x = 0
let there be msg of word        -- msg = ""
let there be flag of dogma      -- flag = forsaken
```

### Declaração com valor inicial

```holy
let there x of atom be 42
let there greeting of word be "Hail, world!"
let there active of dogma be blessed
let there ratio of fractional be 3.14
```

### Reatribuição

```holy
x become x plus 1
greeting become "Farewell"
```

A variável já deve existir. O novo valor deve bater com o tipo declarado.

---

## Operadores

### Aritméticos

| Expressão          | Operação      | Funciona com                            |
|--------------------|---------------|-----------------------------------------|
| `a plus b`         | adição        | `atom`, `fractional`, `word` (concatena)|
| `a minus b`        | subtração     | `atom`, `fractional`                    |
| `a times b`        | multiplicação | `atom`, `fractional`                    |
| `a over b`         | divisão       | `atom` (inteira), `fractional`          |
| `a remainder b`    | módulo        | `atom`                                  |
| `negate a`         | menos unário  | `atom`, `fractional`                    |

```holy
let there x of atom be 10 remainder 3       -- 1
let there y of atom be 7 over 2             -- 3  (divisão inteira!)
let there z of fractional be 7.0 over 2     -- 3.5
let there s of word be "Holy" plus " Lang"  -- "Holy Lang"
```

Misturar `atom` e `fractional` promove ambos para `fractional`.

### Comparação

| Expressão              | Significado |
|------------------------|-------------|
| `a is b`               | igual       |
| `a is not b`           | diferente   |
| `a greater than b`     | maior       |
| `a lesser than b`      | menor       |
| `a no greater than b`  | menor ou igual |
| `a no lesser than b`   | maior ou igual |

`is` / `is not` funcionam em qualquer tipo. Comparações ordenadas funcionam em `atom` e `fractional`.

---

## Precedência de operadores

Do menor para o maior:

| Nível | Operadores                          |
|-------|-------------------------------------|
| 1     | comparações (`is`, `greater`, …)    |
| 2     | `plus`, `minus`                     |
| 3     | `times`, `over`, `remainder`        |
| 4     | `negate` (unário)                   |
| 5     | átomos (literais, variáveis, calls) |

```holy
-- times tem mais precedência que plus:
-- 2 plus 3 times 4  →  2 + (3 * 4) = 14
let there x of atom be 2 plus 3 times 4
```

---

## Agrupamento — `after`

`after` aprofunda o parser para o nível de expressão completa, equivalente a abrir parênteses. O `thus` é **opcional**: só é necessário quando a expressão externa deve continuar após o grupo.

```holy
after 2 plus 3 thus times 4   -- (2 + 3) * 4 = 20  (thus fecha o grupo cedo)
5 plus after 10 minus 3       -- 5 + (10 - 3) = 12  (sem thus, fecha naturalmente)
a times after a plus b        -- a * (a + b)         (sem thus)
```

Veja mais casos em [Aninhamento](nesting.md).

---

## Conversão de tipo

Não há coerção implícita. Use os salms embutidos:

```holy
-- word → atom
let there n of atom be hail atom_of praying "42"

-- qualquer coisa → word
let there s of word be hail word_of praying 99
let there b of word be hail word_of praying blessed
```

---

## Métodos embutidos em `word`

| Método | Retorna | Descrição |
|--------|---------|-----------|
| `hail length upon s` | `atom` | número de caracteres |
| `hail is_empty upon s` | `dogma` | se o texto está vazio |
| `hail at upon s praying i` | `word` | caractere no índice `i` (base zero) |
| `hail slice upon s praying start and end` | `word` | substring `[start, end)` |
| `hail contains upon s praying sub` | `dogma` | se `sub` está contido em `s` |
| `hail starts_with upon s praying prefix` | `dogma` | se começa com `prefix` |
| `hail ends_with upon s praying suffix` | `dogma` | se termina com `suffix` |
| `hail index_of upon s praying sub` | `grace of atom` | posição de `sub` ou `absent` |
| `hail to_upper upon s` | `word` | maiúsculas |
| `hail to_lower upon s` | `word` | minúsculas |
| `hail trim upon s` | `word` | remove espaços nas bordas |
| `hail replace upon s praying old and new` | `word` | substitui todas as ocorrências |
| `hail split upon s praying sep` | `legion of word` | divide pelo separador `sep` |

```holy
let there s of word be "Hello, World!"
hail proclaim praying hail to_upper upon s                    -- "HELLO, WORLD!"
hail proclaim praying hail contains upon s praying "World"    -- blessed
hail proclaim praying hail slice upon s praying 0 and 5      -- "Hello"
hail proclaim praying hail replace upon s praying "World" and "Holy"  -- "Hello, Holy!"

let there parts of legion of word be hail split upon "a,b,c" praying ","
-- parts = ["a", "b", "c"]

let there idx of grace of atom be hail index_of upon s praying "World"
discern idx
    as granted bearing i
        hail proclaim praying hail word_of praying i    -- 7
    as absent
        hail proclaim praying "não encontrado"
```

`at`, `slice` lançam `IndexOutOfBounds` para índices inválidos.

---

## Métodos embutidos em `legion of T`

Veja [Coleções](collections.md) para a referência completa.
