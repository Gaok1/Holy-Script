# Sins — Exceções

Sins são o mecanismo de exceção de Holy. Um sin é lançado com `transgress` e capturado com `confess`/`answer for`. Se não capturado, o programa encerra com uma mensagem de erro.

---

## Declarando um sin

```holy
sin Failure
    message of word

sin OutOfBounds
    index of atom
    max   of atom
```

Um sin sem campos também é válido — o tipo já é informação suficiente:

```holy
sin NotFound
```

---

## Lançando — `transgress`

```holy
transgress Failure praying "algo deu errado"
transgress OutOfBounds praying index, max
transgress OutOfBounds praying index and max
transgress NotFound
```

- Os argumentos seguem a **ordem de declaração dos campos**.
- O último separador pode ser `and`.
- `praying` é omitido se o sin não tem campos.

`transgress` interrompe a execução do bloco atual e propaga o sin pela pilha de chamadas até ser capturado ou encerrar o programa.

---

## Capturando — `confess` / `answer for` / `absolve`

```holy
confess
    -- bloco try
    transgress Failure praying "oops"
answer for Failure
    -- sem ligar a instância: só sabe o tipo
    hail proclaim praying "uma falha ocorreu"
answer for OutOfBounds as err
    -- 'as nome' liga o sin à variável err
    hail proclaim praying "fora dos limites: índice " plus hail word_of praying index from err
absolve
    -- bloco finally: sempre executa, com ou sem erro
    hail proclaim praying "limpeza"
```

Regras:
- `confess` abre o bloco try.
- Pelo menos um `answer for TipoDoSin` é obrigatório.
- `as nome` é opcional; use quando precisar acessar os campos do sin.
- `absolve` (finally) é opcional e vem por último.
- Múltiplos `answer for` cobrem tipos diferentes; o primeiro que combinar é executado.
- Se nenhum combinar, o sin continua propagando pela pilha.

---

## Acessando campos do sin

Dentro de um bloco `answer for … as nome`, a variável ligada se comporta como uma scripture:

```holy
sin ParseError
    input  of word
    column of atom

confess
    transgress ParseError praying "abc" and 3
answer for ParseError as e
    hail proclaim praying "entrada inválida: " plus input from e
    hail proclaim praying "na coluna: " plus hail word_of praying column from e
```

---

## Sins embutidos

O runtime lança estes sins automaticamente para erros comuns. Todos podem ser capturados com `answer for`:

| Nome do Sin               | Quando é lançado |
|---------------------------|-----------------|
| `DivisionByZero`          | `a over 0` ou `a remainder 0` |
| `TypeError`               | valor não corresponde ao tipo declarado |
| `InvalidArgumentCount`    | número errado de argumentos |
| `UndefinedVariable`       | variável não declarada no escopo |
| `UndefinedSalm`           | `hail` de um salm não declarado |
| `UndefinedField`          | `from` em um campo inexistente |
| `IndexOutOfBounds`        | `at` em `word` ou `legion` com índice inválido |
| `UndefinedSin`            | `transgress` de um sin não declarado |
| `UndefinedType`           | anotação de tipo referencia tipo desconhecido |
| `InvalidDiscern`          | `discern` em valor não-covenant, ou nenhum ramo combinou |
| `InvalidContext`          | `its` usado fora de um method salm |

```holy
confess
    let there n of atom be hail atom_of praying "não é número"
    let there result of atom be 100 over n
answer for DivisionByZero
    hail proclaim praying "não é possível dividir por zero"
answer for TypeError
    hail proclaim praying "tipo inválido"
```

---

## Propagação de sins

Se um sin não for capturado em nenhum ponto da pilha, o programa encerra:

```
error: unhandled sin: Failure (algo deu errado)
```

---

## Exemplo completo

```holy
sin DivisionError
    message of word

salm safeDivide receiving a of atom, b of atom reveals atom
    whether b is 0
        transgress DivisionError praying "divisor não pode ser zero"
    reveal a over b

confess
    let there result of atom be hail safeDivide praying 10, 0
    hail proclaim praying hail word_of praying result
answer for DivisionError as e
    hail proclaim praying "Erro: " plus message from e
absolve
    hail proclaim praying "fim da operação"

amen
```

Saída:
```
Erro: divisor não pode ser zero
fim da operação
```
