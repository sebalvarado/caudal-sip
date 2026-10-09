# BCE Authorization Route Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Crear una guía interna, detallada y accionable para obtener la autorización del BCE y llevar la red propia hasta su calificación técnica y conexión productiva.

**Architecture:** La guía nueva ordenará cronológicamente los requisitos que hoy están distribuidos entre el expediente, el plan, las preguntas abiertas y la preparación técnica. `dossier-psap.md` conservará el detalle requisito por requisito; `ruta-autorizacion-bce.md` será la vista operativa, con puertas de salida, responsables, evidencia y dependencias.

**Tech Stack:** Markdown, fuentes regulatorias versionadas en `research/bce/`, Mermaid para el flujo y markdownlint para validación.

## Global Constraints

- Todo el contenido dirigido a instituciones ecuatorianas estará en español nativo.
- Cada afirmación regulatoria citará resolución y artículo; lo no confirmado se marcará **[supuesto]** o como pregunta abierta.
- El objetivo principal es autorizar y calificar la red propia; el gateway hacia otra red no forma parte de esta ruta.
- No se incorporarán contratos firmados, declaraciones notariales, estados financieros, datos personales ni otros documentos sensibles.
- La autorización como partícipe y la calificación técnica y operativa de la red se mostrarán como etapas distintas.
- No crear commits durante la ejecución salvo instrucción expresa del usuario.

---

### Task 1: Verificar y redactar la ruta operativa

**Files:**

- Create: `workstream/authorized-red/regulatory/ruta-autorizacion-bce.md`
- Read: `research/bce/JPRM-2024-018-M.md`
- Read: `research/bce/JPRM-2024-029-M.md`
- Read: `research/bce/BCE-GG-024-2024.md`
- Read: `research/bce/BCE-GG-008-2025.md`
- Read: `workstream/authorized-red/regulatory/dossier-psap.md`
- Read: `workstream/authorized-red/regulatory/open-questions.md`
- Read: `workstream/authorized-red/technical/readiness.md`

**Interfaces:**

- Consumes: requisitos normativos, estados y preguntas abiertas existentes.
- Produces: una guía cronológica que será la entrada principal para ejecutar el trámite.

- [ ] **Step 1: Verificar las citas determinantes**

Comprobar en las transcripciones y sus PDF oficiales:

- JPRM-2024-018-M, arts. 22–23 y 37–38: partícipes, servicios, solicitud múltiple y término de autorización.
- JPRM-2024-029-M, art. 18.1.2: compensación como autorización que habilita a una administradora de red.
- JPRM-2024-029-M, arts. 48.10–48.15: obligaciones, compensación y liquidación.
- BCE-GG-024-2024, arts. 3, 5, 6, 9 y 11–14: expediente, requisitos por servicio, cámara, presentación, calificación y acto administrativo.
- BCE-GG-008-2025, art. 5: calificación técnica y operativa antes de conectarse al SIP.

Resultado esperado: ninguna cita depende únicamente de un resumen secundario.

- [ ] **Step 2: Redactar el mapa ejecutivo**

Abrir la guía con:

1. propósito y alcance;
2. diferencia entre autorización del partícipe y calificación de la red;
3. diagrama Mermaid de las siete etapas; y
4. tabla de puertas: “constituir”, “presentar”, “ser autorizado” y “operar”.

Resultado esperado: una persona puede identificar en menos de dos minutos dónde estamos y qué habilita cada puerta.

- [ ] **Step 3: Redactar los primeros diez días**

Incluir, en orden:

- contratar abogados de pagos;
- cerrar servicios a solicitar;
- constituir sociedad y objeto social;
- nombrar y verificar representante legal;
- obtener RUC;
- documentar accionistas y beneficiarios finales;
- preparar declaraciones juramentadas; y
- crear el índice del repositorio privado de evidencia.

Indicar que los certificados con vigencia mensual se solicitan cerca de la presentación.

- [ ] **Step 4: Construir la matriz de prerrequisitos**

Usar estas columnas:

| Etapa | Requisito | Fuente | Evidencia | Responsable | Dependencia | Estado | Momento | Bloquea |
|---|---|---|---|---|---|---|---|---|

Cubrir los 25 requisitos generales de `dossier-psap.md`, agrupados sin omitirlos, y los adicionales de procesamiento, switch y compensación. Enlazar el número de cada requisito al dossier cuando el detalle ya exista.

- [ ] **Step 5: Separar las dependencias externas**

Crear una sección específica para:

- calificación auxiliar previa;
- contratos elegibles por servicio;
- certificado y perímetro UAFE;
- cámara especializada;
- capital o garantías;
- catálogo técnico;
- BIC, certificados y transporte; y
- interfaz de compensación y liquidación.

Cada dependencia debe indicar qué sabemos, qué falta, quién debe responder y qué puerta bloquea.

- [ ] **Step 6: Describir el proceso después de presentar**

Documentar cronológicamente:

1. entrega en formatos del BCE;
2. revisión documental;
3. subsanación dentro de 10 días;
4. debida diligencia ampliada;
5. visita técnica;
6. término normativo de 30 días para autorizar o negar una solicitud completa;
7. acto administrativo por servicios, medios y canales; e
8. inscripción en el Catastro.

Explicar que los 30 días no incluyen la preparación previa ni garantizan la conexión productiva.

- [ ] **Step 7: Describir la ruta posterior a la autorización**

Separar como puertas posteriores:

- acceso a la cámara;
- calificación técnica y operativa;
- BIC y certificados;
- pruebas con el perfil técnico;
- integración con el Sistema Central de Llaves;
- conexión al SIP; y
- autorización interna de salida a producción.

No asignar fechas a procesos para los que el BCE no ha publicado plazo.

- [ ] **Step 8: Añadir cronograma relativo y tablero semanal**

Usar períodos relativos: “día 0”, “primeros 10 días”, “antes de presentar”, “durante la calificación” y “antes de producción”. Añadir un tablero con evidencia cerrada, dependencia externa, documento próximo a vencer, pregunta jurídica y puerta bloqueada.

### Task 2: Integrar la guía con el frente autorizado

**Files:**

- Modify: `workstream/authorized-red/README.md`
- Modify: `workstream/authorized-red/regulatory/dossier-psap.md`
- Modify: `workstream/authorized-red/regulatory/open-questions.md`
- Modify: `workstream/authorized-red/plan.md`

**Interfaces:**

- Consumes: `regulatory/ruta-autorizacion-bce.md`.
- Produces: una sola entrada visible desde el índice y enlaces coherentes desde los documentos especializados.

- [ ] **Step 1: Añadir la guía al índice**

Agregar en `README.md` una fila que la describa como la secuencia operativa desde constitución hasta conexión al SIP.

- [ ] **Step 2: Enlazar desde el dossier**

Añadir al inicio de `dossier-psap.md` una referencia a la guía para quien necesite orden, responsables y puertas, manteniendo el dossier como lista normativa exhaustiva.

- [ ] **Step 3: Enlazar las preguntas abiertas**

Añadir en `open-questions.md` una referencia a la sección de dependencias de la guía. No duplicar respuestas ni cambiar el estado de una pregunta sin nueva evidencia.

- [ ] **Step 4: Alinear el plan**

Enlazar la nueva guía desde los frentes legal y de expediente de `plan.md`. Confirmar que las metas de octubre y noviembre no prometan autorización o producción bajo control del equipo.

### Task 3: Verificar integridad normativa y documental

**Files:**

- Verify: `workstream/authorized-red/regulatory/ruta-autorizacion-bce.md`
- Verify: `workstream/authorized-red/README.md`
- Verify: `workstream/authorized-red/regulatory/dossier-psap.md`
- Verify: `workstream/authorized-red/regulatory/open-questions.md`
- Verify: `workstream/authorized-red/plan.md`

**Interfaces:**

- Consumes: guía y enlaces terminados.
- Produces: evidencia de que la documentación es consistente y navegable.

- [ ] **Step 1: Revisar cobertura contra el diseño**

Comprobar uno por uno los siete criterios de aceptación de `docs/superpowers/specs/2026-08-18-bce-authorization-route-design.md`.

- [ ] **Step 2: Buscar marcadores y contradicciones**

Buscar `TBD`, `TODO`, afirmaciones sin fuente, contratos presumidos como elegibles y cualquier texto que confunda autorización con calificación o conexión.

- [ ] **Step 3: Validar Markdown**

Ejecutar markdownlint sobre los cinco documentos modificados, desactivando únicamente las reglas de longitud de línea y alineación de tablas que el repositorio no aplica.

Resultado esperado: 0 errores.

- [ ] **Step 4: Validar enlaces internos**

Comprobar que todas las rutas relativas nuevas existan y que `README.md`, el dossier, las preguntas abiertas y el plan lleguen a la guía.

- [ ] **Step 5: Revisar español y límites de certeza**

Confirmar vocabulario normativo, español nativo y uso consistente de **[supuesto]** o “pregunta abierta” donde falte criterio del BCE, la UAFE, el organismo de control o la cámara especializada.
