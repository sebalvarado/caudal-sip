# Caudal Naming Normalization Implementation Plan

**Goal:** Identificar inequívocamente a Caudal, su red y su conector en todos los documentos del frente autorizado.

**Architecture:** Aplicar una convención semántica: marca, red, producto y persona jurídica futura tendrán denominaciones distintas. Los roles regulatorios genéricos conservarán el vocabulario de la norma.

**Tech Stack:** Markdown, Mermaid, ripgrep y markdownlint.

## Global Constraints

- Caudal es la marca y el nombre público de la red, no una razón social confirmada.
- La primera mención será “red de pagos Caudal”; después podrá usarse “red Caudal”.
- La persona jurídica será “sociedad operadora de Caudal” o “entidad solicitante”.
- No modificar citas normativas ni convertir sujetos genéricos en Caudal.
- No crear commits.

### Task 1: Inventariar nombres ambiguos

**Files:**

- Read: `workstream/authorized-red/**/*.md`

- [x] Buscar títulos, diagramas y texto con “nuestra red”, “nuestro gateway”, “la empresa”, “la compañía”, “red propia” y nombres alternativos.
- [x] Clasificar cada aparición como referencia específica a Caudal o sujeto genérico.

### Task 2: Aplicar la convención

**Files:**

- Modify: documentos Markdown afectados dentro de `workstream/authorized-red/`

- [x] Actualizar títulos y primeras menciones.
- [x] Reemplazar referencias específicas por Caudal, red Caudal, conector de Caudal o sociedad operadora de Caudal.
- [x] Actualizar etiquetas Mermaid y tablas comerciales.
- [x] Añadir la convención al README.
- [x] Conservar sujetos normativos genéricos.

### Task 3: Verificar

**Files:**

- Verify: `workstream/authorized-red/**/*.md`

- [x] Buscar denominaciones ambiguas restantes y justificar únicamente las genéricas.
- [x] Confirmar que no se atribuye a Caudal una razón social definida.
- [x] Ejecutar markdownlint con las excepciones de estilo existentes.
- [x] Revisar enlaces y sintaxis Mermaid de los documentos modificados.
