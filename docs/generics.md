# Generics

Generics permitem escrever scriptures, covenants e salms que funcionam com **qualquer tipo**, sem duplicar código. O tipo concreto é informado em cada uso.

Em Holy tudo é **explícito** — não há inferência de tipos. Os argumentos de tipo são sempre escritos em cada chamada e instanciação.

---

## Declarando parâmetros de tipo

Use `of` seguido de nomes de parâmetro após o nome da declaração. O último separador pode ser `and`:

```holy
-- scripture genérica
scripture Box of T
    value of T

scripture Pair of A and B
    first  of A
    second of B

-- covenant genérico
covenant Option of T
    Some
        value of T
    None

-- salm genérico
salm identity of T receiving val of T reveals T
    reveal val

salm wrap of T receiving val of T reveals grace of T
    reveal manifest granted of grace of T praying val
```

Os nomes dos parâmetros são convencionais — letras maiúsculas por convenção (`T`, `E`, `A`, `B`, …).

---

## Passando argumentos de tipo

Em cada chamada ou instanciação, passe os tipos explicitamente com `of`:

```holy
-- instanciando scriptures
let there b of Box of atom      be manifest Box  praying 42
let there p of Pair of atom and word be manifest Pair praying 1 and "x"

-- chamando salms
let there x of atom         be hail identity of atom praying 99
let there g of grace of atom be hail wrap of atom praying 42

-- instanciando variantes de covenant
let there o of Option of atom be manifest Some of Option of atom praying 7
let there n of Option of atom be None of Option of atom
```

---

## `thus` — desambiguação

`thus` é um **marcador de fechamento**. Ele sinaliza ao parser que a lista de argumentos de um tipo genérico aninhado terminou, de modo que o próximo `,` ou `and` pertence ao contexto externo.

### O problema sem `thus`

O parser consome avidamente todos os `,` após um tipo, tratando-os como mais argumentos do tipo mais interno sendo parseado:

```holy
-- ERRADO: parser lê "Stack<T, word>" → verdict não tem segundo argumento
verdict of Stack of T, word

-- ERRADO: mesma situação
verdict of Stack of atom, word
```

### A solução — `thus`

```holy
-- CORRETO: thus fecha Stack<T>, depois "and word" vai para verdict
verdict of Stack of T thus and word

-- CORRETO: thus fecha Stack<atom>
verdict of Stack of atom thus and word
```

### Tipos simples nunca precisam de `thus`

```holy
verdict of atom and word     -- ok: atom não tem argumentos de tipo
grace of word                -- ok
verdict of T and E           -- ok: T e E são nomes simples sem "of"
```

`thus` só é necessário quando o argumento de tipo é ele mesmo genérico **e** é seguido por um separador que pertence ao contexto externo.

---

## Onde `thus` aparece

### 1. Anotações de tipo

Em qualquer posição onde um tipo é escrito — `let there`, `reveals`, `receiving`, campos:

```holy
-- tipo de retorno
salm pop of T receiving s of Stack of T reveals verdict of Stack of T thus and word
    -- ...

-- parâmetro: "s of Stack<T>" depois "and val of T" é o próximo param
salm push of T receiving s of Stack of T thus and val of T reveals Stack of T
    -- ...

-- declaração de variável
let there result of verdict of Stack of atom thus and word be hail pop of atom praying s
```

### 2. Instanciação de variantes

Quando o argumento de tipo de um covenant/variante é ele mesmo genérico:

```holy
-- righteous carrega Stack<T>, E é word
manifest righteous of verdict of Stack of T thus and word praying newStack

-- granted carrega StackNode<T> (thus não necessário: único argumento de grace)
manifest granted of grace of StackNode of T praying node
```

### 3. Argumentos de chamadas aninhadas

Quando uma chamada é usada como argumento de outra e **não é o último argumento**, `thus` fecha a lista de argumentos da chamada interna:

```holy
-- add(double(3), 1) — thus fecha os args de double antes de "and 1"
hail add praying hail double praying 3 thus and 1

-- a(b(c(1)), 2) — primeiro thus fecha c, segundo thus fecha b
hail a praying hail b praying hail c praying 1 thus thus and 2
```

Sem `thus`, `and 1` seria parseado como segundo argumento de `double`.

### 4. Agrupamento de expressões — `after`

`after` aprofunda o parser para o nível de expressão completa. O `thus` é **opcional**: fecha o grupo cedo quando a expressão externa precisa continuar após ele.

```holy
after 3 times 5              -- (3 * 5) = 15  (sem thus)
5 plus after 3 times 2       -- 5 + (3 * 2) = 11  (sem thus)
after a plus b thus times c  -- (a + b) * c  (thus necessário aqui)
```

Veja [Aninhamento](nesting.md) para todos os casos de desambiguação.

---

## Regras do `thus`

Cada `thus` fecha exatamente **um** contexto aberto. Contextos possíveis:

| Aberto por | Fechado por |
|------------|-------------|
| `of` em argumento de tipo genérico | `thus` dentro da análise de tipo |
| `praying` em uma chamada | `thus` após a lista de argumentos |
| `after` | `thus` opcional — fecha o grupo cedo se presente |

Um `thus` sem contexto aberto correspondente é erro de sintaxe.

---

## Apagamento de tipos em runtime

Parâmetros de tipo são apagados em runtime. O interpreter **não** verifica constraints genéricos para:

- Scriptures e covenants definidos pelo usuário com parâmetros de tipo
- Parâmetros de salm tipados com um parâmetro abstrato (ex: `T`)

Ele **verifica** tipos concretos:

```holy
-- TypeError: granted of grace of atom espera atom, recebeu word
manifest granted of grace of atom praying "olá"

-- OK: T é abstrato, sem verificação
salm identity of T receiving val of T reveals T
    reveal val

hail identity of atom praying "qualquer valor"   -- runtime: aceito
```

`grace` e `verdict` embutidos têm verificação de tipo em runtime — seus argumentos de tipo concretos são checados na instanciação.

---

## Exemplo completo — pilha genérica

```holy
scripture Stack of T
    top  of grace of StackNode of T
    size of atom

scripture StackNode of T
    value of T
    next  of grace of StackNode of T

salm emptyStack of T reveals Stack of T
    reveal manifest Stack praying absent of grace of StackNode of T and 0

salm push of T receiving s of Stack of T thus and val of T reveals Stack of T
    let there node of StackNode of T be manifest StackNode praying val and top from s
    reveal manifest Stack praying manifest granted of grace of StackNode of T praying node thus and size from s plus 1

salm peek of T receiving s of Stack of T reveals verdict of T and word
    discern top from s
        as granted bearing node
            reveal manifest righteous of verdict of T and word praying value from node
        as absent
            reveal manifest condemned of verdict of T and word praying "pilha vazia"

let there s of Stack of atom be hail emptyStack of atom
s become hail push of atom praying s and 10
s become hail push of atom praying s and 20

let there peeked of verdict of atom and word be hail peek of atom praying s
discern peeked
    as righteous bearing value
        hail proclaim praying hail word_of praying value   -- "20"
    as condemned bearing reason
        hail proclaim praying reason

amen
```
