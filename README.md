# Holy Lang

Uma linguagem de programação interpretada com sintaxe em inglês arcaico/bíblico, implementada em Rust.

---

## Instalação

```bash
git clone <repo>
cd holy-lang

# rodar direto
cargo run -- programa.holy

# instalar globalmente
cargo install --path .
holy-lang programa.holy
```

---

## CLI

```bash
holy-lang <arquivo.holy>          # executa
holy-lang --tree <arquivo.holy>   # mostra a parse tree (não executa)
holy-lang -t <arquivo.holy>       # idem
```

---

## Estrutura de um programa

Todo programa começa com declarações de topo (`scripture`, `sin`, `salm`) e termina **obrigatoriamente** com `amen`. Exatamente um `amen` por arquivo.

```
-- comentário de linha

scripture ...
sin ...
salm ...

-- statements principais

amen
```

---

## Tipos

| Keyword      | Tipo              | Exemplo de literal       |
|--------------|-------------------|--------------------------|
| `atom`       | inteiro (i64)     | `42`, `-7`               |
| `fractional` | float (f64)       | `3.14`, `-0.5`           |
| `word`       | string            | `"texto"`                |
| `dogma`      | bool              | `blessed` / `forsaken`   |
| `void`       | sem retorno       | —                        |
| `NomeCustom` | scripture criado  | —                        |

---

## Variáveis

```
-- declarar sem valor (recebe padrão do tipo)
let there be x of atom

-- declarar com valor
let there nome of word be "Gabriel"

-- reatribuir
x become 42
```

---

## Scriptures (structs)

```
scripture Pessoa
    nome of word
    idade of atom

-- instanciar (args na ordem dos campos)
let there p of Pessoa be manifest Pessoa praying "Ana", 30

-- acessar campo
let there n of word be nome from p
```

---

## Sins (exceções)

```
sin Falha
    mensagem of word

-- sin sem campos também é válido
sin FalhaSimples
```

---

## Salms (funções)

```
salm somar receiving a of atom, b of atom reveals atom
    reveal a plus b

-- chamar
let there resultado of atom be hail somar praying 3 and 5

-- sem parâmetros
salm cumprimentar reveals void
    hail proclaim praying "Salve!"
```

O último parâmetro pode usar `and` no lugar de `,`:
```
salm foo receiving a of atom, b of word and c of dogma reveals void
    reveal blessed
```

---

## Method Salms (métodos)

Bindados a um scripture via `upon`. No corpo, `its` referencia a instância.

```
salm apresentar upon Pessoa reveals void
    hail proclaim praying nome from its

-- chamar
hail apresentar upon p
```

---

## Construtor

Convenção: salm com o mesmo nome do scripture que retorna `manifest`.

```
salm Pessoa receiving nome of word and idade of atom reveals Pessoa
    reveal manifest Pessoa praying nome and idade

let there p of Pessoa be hail Pessoa praying "Ana" and 30
```

---

## Condicional

```
whether x greater than 10
    hail proclaim praying "grande"
otherwise so x is 10
    hail proclaim praying "exato"
otherwise
    hail proclaim praying "pequeno"
```

---

## Loop

Executa enquanto a condição for verdadeira (`blessed` / truthy).

```
let there be i of atom
i become 1
litany for i no greater than 5
    hail proclaim praying i
    i become i plus 1
```

---

## Confess (try / catch / finally)

```
sin Problema
    descricao of word

confess
    transgress Problema praying "algo errado"
answer for Problema as err
    hail proclaim praying descricao from err
absolve
    hail proclaim praying "sempre executa"
```

- `confess` → bloco try  
- `answer for TipoDoSin` → catch (um por tipo)  
- `as nome` → opcional, vincula a instância do sin a uma variável  
- `absolve` → finally (opcional)  

---

## Operadores

### Aritméticos
```
a plus b      -- adição / concatenação de word
a minus b     -- subtração
a times b     -- multiplicação
a over b      -- divisão
```

### Comparação
```
a is b              -- ==
a is not b          -- !=
a greater than b    -- >
a lesser than b     -- <
a no greater than b -- <=
a no lesser than b  -- >=
```

> Comparações numéricas funcionam com `atom` e `fractional`.  
> `is` / `is not` funcionam com qualquer tipo.

---

## Salms built-in

| Salm       | Descrição                                |
|------------|------------------------------------------|
| `proclaim` | imprime com quebra de linha              |
| `herald`   | imprime sem quebra de linha              |
| `inquire`  | lê uma linha do stdin → `word`           |
| `atom_of`  | converte `word` → `atom`                 |
| `word_of`  | converte qualquer valor → `word`         |

---

## Exemplo completo

```
scripture Apostata
    nome of word
    age of atom
    herege of dogma

sin HeresiaDetectada
    motivo of word

salm Apostata receiving nome of word, age of atom and herege of dogma reveals Apostata
    reveal manifest Apostata praying nome, age and herege

salm julgar upon Apostata reveals void
    whether herege from its
        transgress HeresiaDetectada praying nome from its plus " é herege"
    otherwise
        hail proclaim praying nome from its plus " está absolvido"

let there a of Apostata be hail Apostata praying "João", 33 and blessed

confess
    hail julgar upon a
answer for HeresiaDetectada as err
    hail proclaim praying motivo from err
absolve
    hail proclaim praying "julgamento encerrado"

amen
```

---

## Palavras reservadas

`testament` `revealing` `scripture` `sin` `salm` `upon` `receiving` `reveals`
`let` `there` `be` `of` `become` `hail` `praying` `reveal` `whether` `otherwise`
`so` `litany` `for` `confess` `answer` `absolve` `as` `transgress` `manifest`
`from` `its` `amen` `plus` `minus` `times` `over` `is` `not` `greater` `lesser`
`than` `no` `blessed` `forsaken` `and` `void` `atom` `fractional` `word` `dogma`
