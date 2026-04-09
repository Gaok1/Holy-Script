# Salms

Salms são funções. Cada salm declara explicitamente seus parâmetros e tipo de retorno, e retorna um valor com `reveal`.

---

## Declaração básica

```holy
salm add receiving a of atom, b of atom reveals atom
    reveal a plus b

salm greet reveals void
    hail proclaim praying "Hail!"
```

- `receiving param_list` — opcional; omita se não houver parâmetros.
- `reveals tipo` — obrigatório; use `void` quando o salm não produz valor.
- O corpo é um bloco indentado com pelo menos um statement.
- Em listas Holy, o separador final pode ser `and`: `a and b`, `a, b and c`.

---

## Chamando um salm — `hail`

```holy
-- sem argumentos
hail greet

-- com argumentos
let there result of atom be hail add praying 3, 5
let there result of atom be hail add praying 3 and 5   -- equivalente

-- como statement (ignora o retorno)
hail proclaim praying "done"
```

### Chamadas aninhadas

Um salm pode ser argumento de outro:

```holy
-- proclaim(word_of(42))
hail proclaim praying hail word_of praying 42
```

Quando a chamada interna **não é o último argumento** da externa, use `thus` para fechá-la:

```holy
-- add(double(3), 1) = 7
let there y of atom be hail add praying hail double praying 3 thus and 1
```

Sem o `thus`, `and 1` seria consumido como segundo argumento de `double`. Veja [Aninhamento](nesting.md) para todos os casos.

---

## `reveal` — retorno

`reveal expr` retorna um valor e encerra o salm imediatamente:

```holy
salm max receiving a of atom, b of atom reveals atom
    whether a greater than b
        reveal a
    reveal b
```

`reveal` pode aparecer em qualquer ponto do corpo — dentro de `whether`, `discern`, `litany`, etc.

Um salm `void` pode omitir `reveal` completamente (encerra ao chegar no fim do bloco).

---

## Parâmetros

Cada parâmetro tem nome e tipo explícitos. O separador final pode ser `and`:

```holy
salm describe receiving name of word, age of atom and score of fractional reveals word
    reveal name plus " (" plus hail word_of praying age plus ")"
```

Não existe sintaxe variádica. Se precisar de um número variável de elementos, passe uma `legion`.

---

## Salms genéricos

Declare parâmetros de tipo com `of` após o nome do salm:

```holy
salm identity of T receiving val of T reveals T
    reveal val

salm wrap of T receiving val of T reveals grace of T
    reveal manifest granted of grace of T praying val
```

Passe os tipos explicitamente na chamada:

```holy
let there g of grace of atom be hail wrap of atom praying 42
let there w of grace of word be hail wrap of word praying "hello"
```

Parâmetros de tipo são apagados em runtime — o interpreter aceita qualquer valor para um tipo abstrato sem checar. Tipos concretos (`atom`, `word`, scriptures/covenants registrados) continuam sendo verificados.

---

## Method salms

Declarados com `upon TipoAlvo`. Veja [Scriptures — Method salms](scriptures.md#method-salms).

```holy
salm area upon Rectangle reveals fractional
    reveal width from its times height from its

hail area upon rect
```

---

## Salms embutidos

Disponíveis em todo programa sem declaração:

### I/O

| Salm | Retorna | Descrição |
|------|---------|-----------|
| `proclaim` | `void` | Imprime com quebra de linha |
| `herald` | `void` | Imprime sem quebra de linha |
| `inquire` | `word` | Lê uma linha do stdin |
| `read_file` | `verdict of word and word` | Lê arquivo; righteous(conteúdo) ou condemned(erro) |
| `write_file` | `verdict of dogma and word` | Escreve arquivo; righteous(blessed) ou condemned(erro) |
| `args` | `legion of word` | Argumentos do programa passados na linha de comando |
| `exit` | `void` | Encerra o programa com código de saída |

### Conversão de tipo

| Salm | Retorna | Descrição |
|------|---------|-----------|
| `atom_of` | `atom` | Converte texto para inteiro (0 se inválido) |
| `parse_atom` | `verdict of atom and word` | Converte texto para inteiro com resultado |
| `fractional_of` | `fractional` | Converte para decimal |
| `word_of` | `word` | Converte qualquer valor para texto |

### Matemática

| Salm | Retorna | Descrição |
|------|---------|-----------|
| `abs` | `atom`/`fractional` | Valor absoluto |
| `floor` | `atom` | Arredonda para baixo |
| `ceil` | `atom` | Arredonda para cima |
| `round` | `atom` | Arredonda para o mais próximo |
| `min` | `atom`/`fractional` | Menor de dois valores |
| `max` | `atom`/`fractional` | Maior de dois valores |
| `pow` | `atom`/`fractional` | Potenciação |

### Coleções

| Salm | Retorna | Descrição |
|------|---------|-----------|
| `legion` | `legion of T` | Cria uma legion a partir dos argumentos |

```holy
-- I/O
hail proclaim praying "Hello, world!"
hail herald   praying "sem quebra"
let there line of word be hail inquire
let there a    of legion of word be hail args

-- arquivo
let there r of verdict of word and word be hail read_file praying "dados.txt"
hail write_file praying "saida.txt" and "conteúdo"

-- encerrar
hail exit praying 0

-- conversão
let there n  of atom         be hail atom_of praying "42"
let there r  of verdict of atom and word be hail parse_atom praying "abc"
let there f  of fractional   be hail fractional_of praying 7
let there s  of word         be hail word_of praying 3.14
let there xs of legion of atom be hail legion praying 1, 2 and 3

-- matemática
hail proclaim praying hail word_of praying hail abs praying negate 5   -- 5
hail proclaim praying hail word_of praying hail floor praying 3.9      -- 3
hail proclaim praying hail word_of praying hail pow praying 2 and 10   -- 1024
hail proclaim praying hail word_of praying hail min praying 4 and 9    -- 4
```

`proclaim` e `herald` recebem `word`. Para imprimir outros tipos, converta com `word_of` primeiro:

```holy
let there x of atom be 42
hail proclaim praying hail word_of praying x
```
