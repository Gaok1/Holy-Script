# Holy Script — Guia de Início

Holy é uma linguagem interpretada e fortemente tipada com sintaxe bíblica/arcaica, implementada em Rust.

---

## Estrutura de um programa

Todo arquivo `.holy` segue a mesma estrutura:

```
[testaments]         ← imports (opcional)
[declarações]        ← scripture, sin, covenant, salm (opcional)
[statements]         ← código executável
amen                 ← obrigatório, encerra o arquivo
```

O `amen` é obrigatório e deve ser a última coisa no arquivo.

---

## Olá, mundo

```holy
hail proclaim praying "Hail, world!"

amen
```

Execute com:

```bash
holy hello.holy
```

---

## Um programa completo

```holy
-- Estrutura de dados
scripture Person
    name of word
    age  of atom

-- Função (salm)
salm greet upon Person reveals void
    hail proclaim praying "Hail, " plus name from its plus "!"

-- Código executável
let there p of Person be manifest Person praying "Gabriel", 30
hail greet upon p

amen
```

---

## Conceitos centrais

| Conceito | Palavra-chave | Equivalente em outras linguagens |
|----------|--------------|----------------------------------|
| Variável | `let there`  | `let`, `var`, `int x`            |
| Função   | `salm`       | `func`, `def`, `fn`              |
| Struct   | `scripture`  | `struct`, `class` (só dados)     |
| Enum     | `covenant`   | `enum`, `sealed class`           |
| Exceção  | `sin`        | `exception`, `error`             |
| Loop     | `litany for` | `while`                          |
| If       | `whether`    | `if`/`else if`/`else`            |
| Throw    | `transgress` | `throw`, `raise`                 |
| Try/Catch| `confess`    | `try`/`catch`/`finally`          |
| Return   | `reveal`     | `return`                         |
| Print    | `hail proclaim praying` | `print`, `console.log` |

---

## Tipos primitivos

| Tipo         | Significado    | Exemplo         |
|--------------|----------------|-----------------|
| `atom`       | inteiro (i64)  | `42`, `-7`      |
| `fractional` | decimal (f64)  | `3.14`, `-0.5`  |
| `word`       | texto (UTF-8)  | `"hello"`       |
| `dogma`      | booleano       | `blessed`, `forsaken` |
| `void`       | sem valor      | —               |
| `legion of T`| coleção tipada | `hail legion praying 1, 2 and 3` |

`blessed` = verdadeiro, `forsaken` = falso.

---

## Documentação por tópico

| Tópico | Descrição |
|--------|-----------|
| [Tipos e Variáveis](types.md) | Primitivos, literais, variáveis, operadores, agrupamento |
| [Coleções](collections.md) | `legion of T`, criação e métodos |
| [Salms](salms.md) | Funções, parâmetros, retorno, salms embutidos |
| [Controle de Fluxo](control-flow.md) | `whether`, `litany for`, `forsake`, `ascend` |
| [Scriptures](scriptures.md) | Structs, acesso a campos, métodos |
| [Covenants](covenants.md) | Tipos soma, pattern matching com `discern` |
| [Sins](sins.md) | Exceções, `transgress`, `confess`/`answer for`/`absolve` |
| [Generics](generics.md) | Parâmetros de tipo, `thus` para desambiguação |
| [Aninhamento](nesting.md) | Referência completa do `thus` e `after` |
| [Módulos](modules.md) | `testament`, imports seletivos com `revealing` |
