# Controle de Fluxo

Holy oferece condicionais com `whether` e loops com `litany for`. Não há `for` com iterador nem `do-while` — apenas o loop condicional.

---

## Condicional — `whether`

```holy
whether condição
    -- bloco executado se verdadeiro
```

```holy
let there x of atom be 7

whether x greater than 10
    hail proclaim praying "grande"
otherwise so x is 10
    hail proclaim praying "exatamente dez"
otherwise
    hail proclaim praying "pequeno"
```

- `whether` avalia a condição. Se verdadeira, o bloco abaixo é executado.
- `otherwise so condição` é um else-if (zero ou mais).
- `otherwise` é o else final (opcional, no máximo um).
- O primeiro ramo verdadeiro é executado; os demais são ignorados.

### Valores verdadeiros e falsos

| Tipo         | Verdadeiro quando      |
|--------------|------------------------|
| `dogma`      | `blessed`              |
| `atom`       | diferente de `0`       |
| `fractional` | diferente de `0.0`     |
| `word`       | string não vazia       |
| `void`       | nunca                  |
| scripture    | sempre                 |
| covenant     | sempre                 |

---

## Loop — `litany for`

Repete o corpo enquanto a condição for verdadeira.

```holy
let there i of atom be 1

litany for i no greater than 5
    hail proclaim praying hail word_of praying i
    i become i plus 1
```

Saída: `1`, `2`, `3`, `4`, `5` (um por linha).

A condição é avaliada antes de cada iteração. Se for falsa na primeira checagem, o corpo nunca roda.

**Loop infinito** — use `forsake` para sair:

```holy
litany for blessed
    -- roda para sempre até um forsake
```

---

## Controle de loop

### `forsake` — break

Encerra o `litany for` imediatamente.

```holy
let there i of atom be 1

litany for blessed
    whether i is 5
        forsake
    hail proclaim praying hail word_of praying i
    i become i plus 1
```

Saída: `1`, `2`, `3`, `4`

### `ascend` — continue

Pula o restante da iteração atual e volta para checar a condição.

```holy
let there i of atom be 0

litany for i lesser than 10
    i become i plus 1
    whether i remainder 2 is 0
        ascend          -- pula números pares
    hail proclaim praying hail word_of praying i
```

Saída: `1`, `3`, `5`, `7`, `9`

`forsake` e `ascend` só são válidos dentro de um `litany for`. Fora de um loop causam erro de runtime.

---

## Loops aninhados

`forsake` e `ascend` afetam apenas o `litany for` **mais interno**:

```holy
let there i of atom be 1

litany for i no greater than 3
    let there j of atom be 1
    litany for j no greater than 3
        whether j is 2
            forsake             -- sai apenas do loop interno
        hail proclaim praying hail word_of praying i plus "," plus hail word_of praying j
        j become j plus 1
    i become i plus 1
```

Saída:
```
1,1
2,1
3,1
```

---

## Exemplo completo — FizzBuzz

```holy
let there i of atom be 1

litany for i no greater than 20
    whether i remainder 15 is 0
        hail proclaim praying "FizzBuzz"
    otherwise so i remainder 3 is 0
        hail proclaim praying "Fizz"
    otherwise so i remainder 5 is 0
        hail proclaim praying "Buzz"
    otherwise
        hail proclaim praying hail word_of praying i
    i become i plus 1

amen
```

---

## Exemplo completo — soma de uma legion

```holy
let there nums of legion of atom be hail legion praying 10, 20, 30, 40 and 50
let there sum  of atom be 0
let there i    of atom be 0

litany for i lesser than hail length upon nums
    sum become sum plus hail at upon nums praying i
    i become i plus 1

hail proclaim praying hail word_of praying sum   -- 150

amen
```
