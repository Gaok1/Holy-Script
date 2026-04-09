# Módulos

Holy suporta módulos através da declaração `testament`. Imports aparecem no topo do arquivo, antes de qualquer declaração ou statement. Cada módulo corresponde a um arquivo `.holy` no **mesmo diretório** do arquivo que o importa.

---

## Importando um módulo — `testament`

```holy
testament MathUtils
```

Importa todos os símbolos públicos (scriptures, covenants, salms, sins) de `MathUtils.holy` para o escopo do programa atual.

O arquivo `MathUtils.holy` deve estar no mesmo diretório:

```
projeto/
  main.holy
  MathUtils.holy   ← resolvido automaticamente
```

---

## Import seletivo — `revealing`

```holy
testament MathUtils revealing square, cube
testament Collections revealing Stack and Queue
```

Apenas os símbolos listados são importados. Os demais permanecem inacessíveis. O último separador pode ser `and`.

```holy
-- MathUtils.holy exporta square, cube e is_even
-- mas só square foi importado:
testament MathUtils revealing square

let there s of atom be hail square praying 5    -- ok
let there c of atom be hail cube praying 3      -- UndefinedSalm: cube não foi importado
```

---

## Múltiplos imports

```holy
testament MathUtils
testament Collections revealing Stack
testament Strings   revealing trim, split
```

Imports são processados em ordem de declaração. Módulos já carregados são ignorados automaticamente (sem re-importação).

---

## Estrutura de um arquivo de módulo

Um arquivo de módulo é um arquivo `.holy` normal — pode conter scriptures, covenants, sins, salms e até imports de outros módulos. Statements executáveis no topo do arquivo de módulo **não são executados** — apenas as declarações são importadas.

```holy
-- MathUtils.holy
salm square receiving n of atom reveals atom
    reveal n times n

salm cube receiving n of atom reveals atom
    reveal n times n times n

salm is_even receiving n of atom reveals dogma
    whether n remainder 2 is 0
        reveal blessed
    reveal forsaken

amen
```

```holy
-- main.holy
testament MathUtils

hail proclaim praying hail word_of praying hail square praying 7   -- 49

amen
```

---

## Módulos importando módulos

Um módulo pode importar outros módulos. O interpreter resolve as dependências recursivamente, respeitando a ordem e evitando re-importações:

```holy
-- Vectors.holy
testament MathUtils   -- Vectors pode usar funções de MathUtils

scripture Vector2D
    x of fractional
    y of fractional

salm magnitude upon Vector2D reveals fractional
    reveal hail pow praying x from its times x from its plus y from its times y from its and 0.5

amen
```

---

## Sintaxe completa

```holy
testament NomeDoModulo
testament NomeDoModulo revealing Simbolo1, Simbolo2 and Simbolo3
```

- `NomeDoModulo` é um identificador — deve corresponder ao nome do arquivo (sem `.holy`).
- `revealing` é opcional.
- Todos os `testament` devem vir antes de qualquer `scripture`, `sin`, `covenant`, `salm` ou statement.

---

## Limitações atuais

- Apenas arquivos no **mesmo diretório** são suportados. Subpastas serão implementadas em versão futura.
- Imports circulares (`A` importa `B` que importa `A`) não são detectados e podem causar comportamento inesperado.
