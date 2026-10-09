# Caudal — Frente de trabajo para la red de pagos autorizada

La línea de trabajo societaria y de licenciamiento para constituir la **sociedad operadora de Caudal**, obtener la autorización de compensación y completar la calificación técnica y operativa de la **red de pagos Caudal** para fintech y cooperativas. No depende de operar primero bajo una red existente. Es la contraparte comercial del proyecto abierto Caudal: el software es Apache-2.0 para siempre; este frente se ocupa de la autorización, la operación y los contratos.

**El reloj**: las instituciones financieras deben hablar el estándar el **30 de noviembre de 2026**. La meta controlable es presentar el expediente completo, demostrar la operación y contratar futuros integrantes; obtener la autorización y la calificación para esa fecha depende de los tiempos del BCE — ver [`plan.md`](plan.md).

## Mapa

| Documento | Qué contiene |
|---|---|
| [`plan.md`](plan.md) | El plan de ejecución: frentes, fechas, calendario hacia la presentación y las dependencias externas con su mitigación |
| [`resumen-ejecutivo.md`](resumen-ejecutivo.md) | Panorama institucional para fintech, cooperativas y proveedores de core: esquema, oportunidad, estrategia de la red Caudal, requisitos y preguntas pendientes |
| [`business-plan.md`](business-plan.md) | Plan de negocio para socios e inversionistas: estrategia, referentes regionales (STP, Celcoin, Transfeera, Minka, Mono), modelo operativo con agentes de IA y proyección financiera a cinco años |
| [`regulatory/ruta-autorizacion-bce.md`](regulatory/ruta-autorizacion-bce.md) | Guía operativa desde las decisiones legales y la constitución hasta la autorización, calificación técnica y conexión de la red al SIP |
| [`regulatory/dossier-psap.md`](regulatory/dossier-psap.md) | El expediente de autorización ante el BCE (BCE-GG-024-2024) como lista de control, requisito por requisito |
| [`regulatory/open-questions.md`](regulatory/open-questions.md) | Las dudas legales y regulatorias abiertas, cada una con su ruta de respuesta y su responsable |
| [`commercial/pipeline.md`](commercial/pipeline.md) | Prospectos de miembros y seguimiento de contratos de adhesión — los contratos son un **requisito del expediente**, no un detalle |
| [`technical/readiness.md`](technical/readiness.md) | Lo que debe estar funcionando y ser demostrable para la visita técnica del BCE, componente por componente |
| [`technical/requerimientos.md`](technical/requerimientos.md) | Hoja de construcción: componentes, pruebas, puertas de salida y ruta crítica hacia la red Caudal |
| [`../../research/bce/README.md`](../../research/bce/README.md) | Fuentes primarias del BCE: PDF oficial y transcripción Markdown para búsqueda, con URL y SHA-256 |

## Reglas de trabajo en este frente

1. **Citar y verificar la norma.** Antes de una afirmación regulatoria, buscar en el `.md` correspondiente de [`../../research/bce/`](../../research/bce/) y verificar en su PDF oficial las citas, tablas y requisitos. Citar resolución + artículo (BCE-GG-024-2024, JPRM-2024-018-M, BCE-GG-008-2025, BCE-GG-007-2026) o Catálogo Operacional + sección. Las lecturas sin verificar se marcan **[supuesto]**. El Catálogo Operacional no sustituye el catálogo técnico: sin esquemas, campos y validaciones publicados, no se inventan reglas de interfaz.
2. **Idioma**: los documentos de este frente están en español, porque se trabajan con socios, abogados y contrapartes ecuatorianas. El código, los identificadores y los mensajes del protocolo (`pacs.008`, `ACSP`) quedan como están, igual que en el resto del repositorio, conforme a [`docs/language-policy.md`](../../docs/language-policy.md) y [`docs/spanish-style.md`](../../docs/spanish-style.md).
3. **El material sensible no entra a git.** Nunca subir contratos firmados, documentos notariados, cédulas o pasaportes, estados financieros ni datos de contrapartes. Aquí se los referencia por nombre; los archivos van en el repositorio privado de la sociedad operadora de Caudal. Antes de hacer push de este directorio, confirmar la visibilidad del repositorio — esto es estrategia de negocio y el repositorio puede ser público.
4. **La arquitectura técnica no se duplica aquí.** Lo que una red debe hacer está en [`docs/architecture/red.md`](../../docs/architecture/red.md); la especificación normativa, en el resumen del catálogo. Este frente guarda únicamente lo propio de *conseguir la autorización y operar como empresa*.

### Convención de nombres

- **Caudal** es el nombre del proyecto y la marca.
- **Red de pagos Caudal** es la primera mención pública de la red; después se usa **red Caudal**.
- **Conector de Caudal** nombra la capa de integración para los integrantes.
- **Sociedad operadora de Caudal** o **entidad solicitante** nombra la futura persona jurídica mientras su razón social no esté definida.
- Los términos normativos genéricos —administradora de red, partícipe, integrante o entidad— se conservan cuando no se refieren específicamente a Caudal.

## Relación con el proyecto abierto Caudal

Decidido (18-08-2026): la sociedad operadora de Caudal se forma **alrededor** del proyecto abierto para administrar la red Caudal. El núcleo sigue siendo Apache-2.0 y el cumplimiento del esquema nunca se cobra (GOVERNANCE.md); la sociedad vende membresía, operación, alojamiento, certificación y acuerdos de nivel de servicio. Posicionamiento: primero las fintech (SEDPES), las cooperativas como segundo segmento. La conectividad hacia una red existente solo se evalúa como oportunidad opcional, con autorización clara y sin exclusividad.
