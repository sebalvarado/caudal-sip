# Caudal — Expediente de autorización ante el BCE

Esta lista reúne los requisitos para que la entidad solicitante sea autorizada como partícipe del Sistema Auxiliar de Pagos, según la **BCE-GG-024-2024** (norma que implementa la JPRM-2024-018-M, arts. 22–23 y 37). Estrategia de presentación por validar con los abogados: una sola solicitud pidiendo **procesamiento de medios de pago electrónicos + switch transaccional para servicios de pago + compensación** (el art. 3.1 admite varios servicios por solicitud). La compensación es la categoría que habilita a una administradora de red (JPRM-2024-029-M, art. 18.1.2).

Para ejecutar estos requisitos en orden —desde la constitución hasta la conexión al SIP— usar [`ruta-autorizacion-bce.md`](ruta-autorizacion-bce.md). Este documento conserva la lista normativa exhaustiva y su estado.

Leyenda de estado: ☐ sin empezar · ◐ en curso · ☑ listo · ⚠ bloqueado/externo

## Requisitos generales (art. 3 — aplican a todos los servicios)

| # | Art. | Requisito | Notas | Estado |
|---|---|---|---|---|
| 1 | 3.1 | Solicitud suscrita por el representante legal (Anexo 1), detallando cada servicio pedido | Los formatos de los anexos están en la web del BCE; se entregan firmados y en archivo editable | ☐ |
| 2 | 3.2 | Copia del RUC | Después de la constitución | ☐ |
| 3 | 3.2.1–3.2.2 | Declaración juramentada (notariada) de la entidad **y** del representante legal: sin moras mayores a 60 días ni cartera castigada (3 años), sin inhabilitación de cuentas corrientes, sin moras con el SRI ni el IESS, no contratista incumplido del Estado, sin incumplimientos ante la Superintendencia, no intervenida; el representante además: sin sentencias por delitos financieros | Matriz según el Anexo 3 + certificados de respaldo, cada uno del mes de la presentación o del anterior — **sacarlos al final** | ☐ |
| 4 | 3.3 | Documentos de constitución + estatuto con el servicio dentro del **objeto social**, inscripción en el Registro Mercantil (Anexo 4) | El objeto social debe ser exclusivo fintech (Ley Fintech, art. 7.2) y cubrir todos los servicios del art. 23 que la entidad solicitante pueda solicitar | ☐ |
| 5 | 3.4 | Nómina de accionistas desglosada hasta las **personas naturales** que controlan el capital (Anexo 5) | | ☐ |
| 6 | 3.5 | Formulario "Conozca a su partícipe" (Anexo 6) | | ☐ |
| 7 | 3.6 | Estados financieros suscritos por el representante legal y el contador. **Entidades que aún no operan: solo el mes inmediato anterior** | Clave del cronograma: la calificación no exige trayectoria | ☐ |
| 8 | 3.7 | Informes de auditoría externa | Solo para entidades con más de 2 ejercicios fiscales operando — **no aplica** | ☑ |
| 9 | 3.8 | Listado de oficinas y puntos de atención (Anexo 7); contratos de arriendo si el local es de terceros | Deben existir físicamente — se verifican en la visita técnica | ☐ |
| 10 | 3.9 | Plan de negocios (Anexo 8) | | ☐ |
| 11 | 3.10 | Proyección financiera: viabilidad, factibilidad y sostenibilidad (Anexo 9) | | ☐ |
| 12 | 3.11 | **Esquema operativo** por servicio: clientes, usuarios, comercios, entidades, proveedores, medios y canales; diagramas de flujo, información, enrutamiento y procesos; conexiones con otros partícipes; estructura del código QR | Redactar a partir de `docs/architecture/red.md`, `blueprint.md` y `scheme-design.md` | ☐ |
| 13 | 3.12 | Detalle de infraestructura tecnológica: capacidad, desempeño y diagrama de red (Anexo 10) | | ☐ |
| 14 | 3.13 | Contratos con proveedores críticos (nube, redes, seguridad, software), nacionales o extranjeros | Los documentos extranjeros, apostillados y traducidos (art. 11) | ☐ |
| 15 | 3.14 | Certificación ISO 31000 **o** metodología de riesgos operativos: mapa de procesos, funciones, identificación/medición/control/monitoreo + matriz de riesgos (Anexo 11) | Se acepta la metodología — redactarla | ☐ |
| 16 | 3.15 | **Metodología de riesgo de liquidez**: herramientas operativas y analíticas, indicadores con umbrales, apetito y tolerancia, acciones ante déficits imprevistos | Aprovechar el análisis del techo de liquidez del resumen del catálogo, §8 | ☐ |
| 17 | 3.16 | Certificación ISO 27001 **o** políticas y procedimientos de seguridad de la información alineados con estándares internacionales (Anexo 12) | Se acepta la metodología | ☐ |
| 18 | 3.17 | Certificación ISO 22301 **o** plan de continuidad del negocio (Anexo 13) | Se acepta la metodología | ☐ |
| 19 | 3.18 | Manual de Buen Gobierno Corporativo (Anexo 14) | | ☐ |
| 20 | 3.19 | Manual de atención a clientes y usuarios: canales, proceso de reclamos, resolución de controversias, niveles de escalamiento, tiempos de respuesta | | ☐ |
| 21 | 3.20 | Contratos de adhesión con proveedores relacionados con el servicio, con todos sus anexos: servicio, obligaciones, medios y canales | | ☐ |
| 22 | 3.21 | Calificación como entidad de servicios auxiliares del sistema financiero, emitida por el organismo de control | **[supuesto]** que aplica a la entidad solicitante — depende de su clasificación; confirmar con los abogados (open-questions P3) | ⚠ |
| 23 | 3.22 | Política de Protección de Datos Personales conforme a la LOPDP y su Reglamento (Anexo 15) | Alineada con la postura de protección de datos de `red.md` | ☐ |
| 24 | 3.23 | Certificado de cumplimiento de obligaciones de la **UAFE** | Dependencia externa — iniciar el registro temprano | ⚠ |
| 25 | 3.24 | Documentación de operaciones en el extranjero, si la entidad o su grupo empresarial presta servicios de pago fuera | Verificar afiliaciones y operación exterior del grupo antes de marcar “no aplica” | ☐ |

## Requisitos por servicio

### Procesamiento de medios de pago electrónicos (art. 5)

| Art. | Requisito | Notas | Estado |
|---|---|---|---|
| 5.1 | Calificación como servicio auxiliar (categorías pagos/transaccionales) del organismo de control | La misma dependencia del ítem 22 | ⚠ |
| 5 | Certificado PCI-DSS | Solo si se procesan datos de tarjetas; cerrar el alcance de todos los flujos y canales antes de marcar “no aplica” | ☐ |
| 5.2 | Contratos de adhesión firmados con las entidades financieras atendidas, con acuerdos de nivel de servicio | → `commercial/pipeline.md` | ☐ |

### Switch transaccional para servicios de pago (art. 6)

| Art. | Requisito | Notas | Estado |
|---|---|---|---|
| 6.1 | PCI-DSS | Solo si se procesan datos de tarjetas; requiere la misma confirmación de alcance | ☐ |
| 6.2 | Contratos de adhesión firmados con los partícipes atendidos, con acuerdos de nivel de servicio | El huevo y la gallina: los miembros firman antes de la autorización → `commercial/pipeline.md` | ☐ |

### Compensación (art. 9)

Cumplir lo que disponga el BCE para participar en la cámara de compensación especializada, antes de operar. Debe formar parte de la solicitud si el objetivo es obtener la autorización para administrar una red; falta confirmar con el BCE la secuencia y los requisitos operativos adicionales.

## El proceso después de presentar (arts. 11–14)

1. Documentos en orden, según los formatos de los anexos, firmados y en archivo editable; los extranjeros, apostillados y traducidos (art. 11).
2. Calificación (art. 12): revisión documental → **término de 10 días** para subsanar deficiencias (no llegar = NO CALIFICADA) → debida diligencia ampliada de lavado de activos → **visita técnica** que verifica oficinas, centro de datos y que los servicios implementados coincidan con el expediente (→ `technical/readiness.md`).
3. Acto administrativo de autorización (art. 13) con los servicios, medios y canales autorizados; inscripción en el Catastro público (art. 14).
4. Después: supervisión permanente, in situ y extra situ (arts. 15–18).
