# Covenants

Um covenant é um **tipo soma** (tagged union): um valor do tipo covenant é sempre exatamente uma de suas variantes nomeadas. Pense como `enum` em Rust ou `sealed class` em Kotlin — mas com sintaxe bíblica.

Scriptures perguntam "o que esse valor **tem**?". Covenants perguntam "o que esse valor **é**?". A resposta é sempre uma de suas variantes.

---

## Declarando um covenant

Variantes podem ser **unitárias** (sem dados) ou **com dados** (campos indentados abaixo do nome):

```holy
covenant Direction
    North
    South
    East
    West
```

```holy
covenant Shape
    Circle
        radius of fractional
    Rectangle
        width  of fractional
        height of fractional
    Point           -- variante unitária (sem campos)
```

- Pelo menos uma variante é obrigatória.
- Uma variante com bloco indentado é uma **variante com dados**.
- Uma variante sem bloco é uma **variante unitária**.

---

## Criando um valor

### Variante unitária

```holy
let there d of Direction be North of Direction
```

O sufixo `of CovenantName` é obrigatório — o interpreter precisa saber a qual covenant a variante pertence.

### Variante com dados — `manifest`

```holy
let there s of Shape be manifest Circle    of Shape praying 5.0
let there r of Shape be manifest Rectangle of Shape praying 3.0, 4.0
let there q of Shape be manifest Rectangle of Shape praying 3.0 and 4.0
```

Os argumentos seguem a **ordem de declaração dos campos**. O último separador pode ser `and`.

---

## Pattern matching — `discern`

`discern` inspeciona o valor e executa o ramo correspondente à variante atual:

```holy
discern d
    as North
        hail proclaim praying "indo para cima"
    as South
        hail proclaim praying "indo para baixo"
    otherwise
        hail proclaim praying "indo de lado"
```

- Pelo menos um ramo `as` é obrigatório.
- `otherwise` (opcional) captura qualquer variante não listada.
- Se nenhum ramo combina e não há `otherwise`, um sin `InvalidDiscern` é lançado em runtime.

### Ligando campos — `bearing`

Use `bearing nome1, nome2, …` após o nome da variante para ligar seus campos a variáveis locais:

```holy
discern s
    as Circle bearing r
        hail proclaim praying "círculo, raio " plus hail word_of praying r
    as Rectangle bearing w and h
        let there area of fractional be w times h
        hail proclaim praying "retângulo, área " plus hail word_of praying area
    as Point
        hail proclaim praying "só um ponto"
```

- As ligações são **posicionais** (mesma ordem dos campos declarados).
- Você pode ligar menos nomes do que campos existem — extras são ignorados.
- Variantes unitárias nunca usam `bearing`.

---

## Covenants genéricos

Covenants podem declarar parâmetros de tipo com `of`:

```holy
covenant Option of T
    Some
        value of T
    None

covenant Either of L and R
    Left
        val of L
    Right
        val of R
```

Ao instanciar, passe os tipos explicitamente:

```holy
let there o of Option of atom be manifest Some of Option of atom praying 42
let there n of Option of atom be None of Option of atom
let there e of Either of atom and word be manifest Left of Either of atom and word praying 7
```

---

## Covenants embutidos

Dois covenants são pré-carregados em todo programa. São genéricos e têm verificação de tipo em runtime.

---

### `grace of T` — valor opcional

Equivalente a `Option` / `Maybe` em outras linguagens. Representa "pode ter ou não ter um valor".

| Variante  | Campos | Significado |
|-----------|--------|-------------|
| `granted` | `T`    | um valor está presente |
| `absent`  | —      | nenhum valor (variante unitária) |

```holy
-- criando
let there g of grace of atom be manifest granted of grace of atom praying 42
let there n of grace of atom be absent of grace of atom

-- valor padrão quando declarado sem inicializador
let there be x of grace of word    -- x = absent
```

```holy
-- usando
discern g
    as granted bearing value
        hail proclaim praying hail word_of praying value   -- "42"
    as absent
        hail proclaim praying "nada aqui"
```

`manifest granted of grace of atom praying "texto"` lança `TypeError` — o valor interno deve ser do tipo `atom`.

---

### `verdict of T and E` — resultado falível

Equivalente a `Result` em outras linguagens. Representa "operação que pode ter sucesso ou falha".

| Variante     | Campos | Significado |
|--------------|--------|-------------|
| `righteous`  | `T`    | operação bem-sucedida |
| `condemned`  | `E`    | operação falhou |

```holy
-- criando
let there r of verdict of atom and word be manifest righteous of verdict of atom and word praying 99
let there e of verdict of atom and word be manifest condemned of verdict of atom and word praying "entrada inválida"
```

```holy
-- usando
discern r
    as righteous bearing value
        hail proclaim praying hail word_of praying value    -- "99"
    as condemned bearing reason
        hail proclaim praying reason
```

#### Tipos genéricos aninhados

Quando `T` ou `E` é ele mesmo genérico, use `thus` para fechar o tipo interno antes do separador externo:

```holy
-- verdict<Stack<atom>, word>
let there result of verdict of Stack of atom thus and word be hail pop of atom praying s
```

Veja [Generics — `thus`](generics.md#thus--disambiguação) para a regra completa.

---

## Exemplo completo

```holy
covenant Direction
    North
    South
    East
    West

salm move upon Direction receiving steps of atom reveals word
    discern its
        as North
            reveal "subiu " plus hail word_of praying steps
        as South
            reveal "desceu " plus hail word_of praying steps
        as East
            reveal "foi leste " plus hail word_of praying steps
        as West
            reveal "foi oeste " plus hail word_of praying steps

let there d of Direction be North of Direction
hail proclaim praying hail move upon d praying 3    -- "subiu 3"

d become East of Direction
hail proclaim praying hail move upon d praying 5    -- "foi leste 5"

amen
```
