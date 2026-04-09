# Coleções — `legion of T`

`legion of T` é a coleção tipada embutida de Holy. Pense nela como um array onde o tipo do elemento é sempre declarado explicitamente.

---

## Criando uma legion

Use o salm embutido `legion` com `hail`:

```holy
let there xs    of legion of atom be hail legion praying 1, 2 and 3
let there names of legion of word be hail legion praying "Ava" and "Noah"
```

Declaração sem valor inicial (legion vazia):

```holy
let there be xs of legion of atom      -- [] (vazio)
```

---

## Tipagem em runtime

Uma `legion of atom` só aceita elementos `atom`. Tentar inserir outro tipo lança `TypeError`:

```holy
-- OK
let there xs of legion of atom be hail legion praying 1 and 2
xs become hail push upon xs praying 3

-- TypeError: "oops" não é atom
let there ys of legion of atom be hail legion praying 1 and "oops"
```

---

## Métodos disponíveis

Todos os métodos seguem a sintaxe padrão:

```holy
hail método upon alvo
hail método upon alvo praying arg1, arg2
```

| Método | Retorna | Descrição |
|--------|---------|-----------|
| `hail length upon xs` | `atom` | número de elementos |
| `hail is_empty upon xs` | `dogma` | se a legion está vazia |
| `hail at upon xs praying i` | `T` | elemento no índice `i` (base zero) |
| `hail first upon xs` | `grace of T` | primeiro elemento ou `absent` |
| `hail last upon xs` | `grace of T` | último elemento ou `absent` |
| `hail contains upon xs praying v` | `dogma` | se `v` está na legion |
| `hail index_of upon xs praying v` | `grace of atom` | posição de `v` ou `absent` |
| `hail reverse upon xs` | `legion of T` | nova legion invertida |
| `hail push upon xs praying v` | `legion of T` | nova legion com `v` no final |
| `hail slice upon xs praying start and end` | `legion of T` | sub-legion `[start, end)` |
| `hail concat upon xs praying ys` | `legion of T` | nova legion com `ys` concatenada |

> **Importante:** todos os métodos que retornam `legion of T` (`push`, `slice`, `concat`, `reverse`) **não mutam** a legion existente — retornam uma nova. Reatribua se precisar:
> ```holy
> xs become hail push upon xs praying 99
> xs become hail reverse upon xs
> ```

---

## Exemplos práticos

### Iterar uma legion

```holy
let there xs of legion of atom be hail legion praying 10, 20 and 30
let there i  of atom be 0

litany for i lesser than hail length upon xs
    hail proclaim praying hail word_of praying hail at upon xs praying i
    i become i plus 1
```

### Construir uma legion dinamicamente

```holy
let there be result of legion of atom
let there i of atom be 1

litany for i no greater than 5
    result become hail push upon result praying i times i   -- quadrados: 1, 4, 9, 16, 25
    i become i plus 1

hail proclaim praying hail word_of praying result
```

### Fatiar e concatenar

```holy
let there xs   of legion of atom be hail legion praying 1, 2, 3, 4 and 5
let there head of legion of atom be hail slice upon xs praying 0 and 2   -- [1, 2]
let there tail of legion of atom be hail slice upon xs praying 2 and 5   -- [3, 4, 5]
let there all  of legion of atom be hail concat upon head praying tail    -- [1, 2, 3, 4, 5]
```

---

## `legion` genérico

Você pode declarar scriptures e salms que trabalham com `legion of T`:

```holy
salm first of T receiving xs of legion of T reveals verdict of T and word
    whether hail is_empty upon xs
        reveal manifest condemned of verdict of T and word praying "legion is empty"
    reveal manifest righteous of verdict of T and word praying hail at upon xs praying 0
```

`at` lança `IndexOutOfBounds` para índices fora do intervalo. `slice` com `start > end` ou `end > length` também lança `IndexOutOfBounds`.
