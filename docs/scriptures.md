# Scriptures

Scriptures são estruturas de dados — coleções nomeadas de campos tipados. Pense nelas como `struct` em outras linguagens. Uma scripture só guarda dados; comportamento é adicionado através de [method salms](#method-salms).

---

## Declarando uma scripture

```holy
scripture Point
    x of atom
    y of atom

scripture Person
    name of word
    age  of atom
```

- Pelo menos um campo é obrigatório.
- Campos são declarados em ordem; essa ordem importa na hora de criar valores.
- Nomes de campos devem ser únicos dentro da scripture.

---

## Criando um valor — `manifest`

```holy
let there p of Point  be manifest Point  praying 3, 4
let there u of Person be manifest Person praying "Gabriel", 30
```

Os argumentos são passados **na ordem de declaração dos campos**, separados por `,`. O último separador pode ser `and`:

```holy
let there p of Point be manifest Point praying 3 and 4
```

---

## Lendo campos — `from`

```holy
let there px of atom be x from p
let there nm of word be name from u
```

`from` lê um campo pelo nome. Não muda o valor.

Campos podem ser encadeados quando o campo é ele mesmo uma scripture:

```holy
scripture Address
    city of word

scripture Employee
    name    of word
    address of Address

let there emp of Employee be manifest Employee praying "Ava", manifest Address praying "São Paulo"

let there city of word be city from address from emp
-- lê emp.address.city
```

---

## Semântica de valor (imutabilidade)

Scriptures em Holy são **valores imutáveis** — não é possível atribuir diretamente a um campo. Para "mudar" uma scripture, cria-se um novo valor e reatribui a variável:

```holy
scripture Person
    name of word
    age  of atom

let there p of Person be manifest Person praying "Gabriel", 30

-- proibido: atribuição direta a campo não existe
-- age from p become 31   ← erro de sintaxe

-- correto: criar um novo valor
p become manifest Person praying name from p, 31
```

O mesmo vale para scriptures aninhadas: construa um novo valor interno, depois um novo valor externo, e reatribua.

---

## Method salms

Um method salm é declarado com `upon TipoAlvo` e é chamado em uma instância daquele tipo:

```holy
salm introduce upon Person reveals void
    hail proclaim praying "I am " plus name from its

-- chamada
hail introduce upon p
```

Com parâmetros:

```holy
salm greetWith upon Person receiving greeting of word reveals void
    hail proclaim praying greeting plus ", " plus name from its plus "!"

hail greetWith upon p praying "Hail"
```

### `its` — a instância atual

Dentro do corpo de um method salm, `its` refere-se à instância sobre a qual o método foi chamado:

```holy
salm fullName upon Person reveals word
    reveal name from its plus " (age " plus hail word_of praying age from its plus ")"

let there fn of word be hail fullName upon p
hail proclaim praying fn    -- "Gabriel (age 31)"
```

- `its` tem o tipo da scripture ligada ao método.
- `its` é somente leitura; não é possível reatribuí-lo.

---

## Convenção de construtor

Por convenção, um salm com o mesmo nome da scripture funciona como construtor:

```holy
scripture Point
    x of atom
    y of atom

salm Point receiving x of atom, y of atom reveals Point
    reveal manifest Point praying x, y

-- uso mais limpo
let there p of Point be hail Point praying 3, 4
```

---

## Scriptures genéricas

Scriptures podem declarar parâmetros de tipo com `of`:

```holy
scripture Box of T
    value of T

scripture Pair of A and B
    first  of A
    second of B
```

Ao criar um valor, passe os tipos explicitamente:

```holy
let there b of Box of atom      be manifest Box  praying 42
let there p of Pair of atom and word be manifest Pair praying 1 and "x"
```

Veja [Generics](generics.md) para as regras completas de tipos genéricos e uso de `thus`.

---

## Valor padrão

Quando declarada sem valor inicial (`let there be`), a variável de scripture é inicializada como `void`. Acessar seus campos antes de atribuir um valor real causará erro de runtime:

```holy
let there be p of Point         -- p = void internamente
p become manifest Point praying 0, 0   -- seguro agora
let there px of atom be x from p       -- 0
```

---

## Exemplo completo

```holy
scripture Rectangle
    width  of fractional
    height of fractional

salm area upon Rectangle reveals fractional
    reveal width from its times height from its

salm describe upon Rectangle reveals void
    let there a of fractional be hail area upon its
    hail proclaim praying "Área: " plus hail word_of praying a

let there r of Rectangle be manifest Rectangle praying 5.0 and 3.0
hail describe upon r    -- "Área: 15"

amen
```
