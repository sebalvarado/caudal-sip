# Caudal — Preparación para la visita técnica del BCE

La calificación del BCE incluye una visita en sitio que verifica que **los servicios implementados coincidan con el expediente** — oficinas, centro de datos y el sistema funcionando (BCE-GG-024-2024, art. 12). Este documento mapea lo que debe estar en vivo y ser demostrable contra lo que el repositorio ya construye. La arquitectura como tal está en `docs/architecture/`; aquí solo se sigue el estado de preparación.

## Debe estar funcionando y ser demostrable al presentar (mediados de octubre)

| Capacidad | Componente del repositorio | Requisito del expediente que evidencia | Estado |
|---|---|---|---|
| Pago simulado de punta a punta: resolución de llave → pacs.008 → confirmación, en menos de 20 s | `sandbox/` (la red en una caja) | Esquema operativo (art. 3.11) | ☐ |
| Conector de Caudal (multiempresa): API JSON de entrada, ISO 20022 firmado de salida, aislamiento por cliente | `gateway/` | Servicio de procesamiento (art. 5) | ☐ |
| Consola de operación: búsqueda de pagos, conectividad de miembros, bandeja de conciliación | `console/` | Manual de atención a clientes (art. 3.19), metodología de riesgos operativos | ☐ |
| Corridas de conformidad con informes de aprobado/fallido | `tck/` | Esquema operativo; el flujo de certificación | ☐ |
| Bitácora de auditoría inmutable, pagos reconstruibles | `audit/`, el journal según `red.md` | Metodología de seguridad de la información (Anexo 12), preparación para la supervisión | ☐ |
| Monitoreo, alertas, guardia y runbooks (en español) | montaje operativo, con `docs/development/local-stack.md` de base | Plan de continuidad (Anexo 13) | ☐ |
| Cifrado en reposo y en tránsito, datos personales enmascarados en consola y logs | valores por defecto del conector y la consola según la sección de protección de datos de `red.md` | Política LOPDP (Anexo 15), Art. 10 de la BCE-GG-008-2025 | ☐ |

## Diferido a propósito (bloqueado por el BCE, simulado por diseño)

- **Los formatos de la cara norte** — el catálogo técnico sigue sin publicarse; el transporte y el perfil de mensajes quedan detrás de una interfaz con una contraparte simulada (la disciplina que el repositorio ya tiene, ver `red.md`, "What we can build now").
- **La conectividad real al SIP** — estándar de transporte y autoridad certificadora sin publicar; **[supuesto]** la administradora requerirá un identificador propio, pero el art. 6 de la BCE-GG-008-2025 solo define el de los integrantes (open-questions P5).
- **La integración de liquidación con la cámara especializada** — la JPRM-2024-029-M, arts. 48.14–48.15, confirma que la red compensa lo intra-red y que el BCE liquida las posiciones en cuentas de los integrantes; falta que el BCE publique la interfaz operativa.

El guion de la demostración para la visita: un pago iniciado por la API JSON de un cliente que cruza la red simulada, con la consola mostrando cada salto, más una corrida del tck que produce un informe de certificación. Es la respuesta más fuerte posible a "¿los servicios implementados coinciden con el expediente?" mientras el lado del BCE siga sin publicarse.
