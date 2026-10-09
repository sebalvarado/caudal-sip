# Red de pagos Caudal — Requisitos técnicos para producción

Este documento define qué se construye, sobre qué infraestructura, cómo se prueba cada pieza y qué condiciones —técnicas, contractuales y regulatorias— deben cerrarse antes de operar. Es el plan de ejecución técnica del frente; la arquitectura de fondo está en [`docs/architecture/red.md`](../../../docs/architecture/red.md) y la barra no funcional en [`docs/architecture/technical-requirements.md`](../../../docs/architecture/technical-requirements.md). El estado de evidencia para la visita técnica se sigue en [`readiness.md`](readiness.md).

## 1. Una salida principal

Los integrantes no se conectan al SIP: se conectan a una red de pagos, y la red se conecta al SIP. La única salida a producción planificada es la red de pagos Caudal.

|                    | **Producción — red de pagos Caudal**                                                                                                                                                                  |
| ------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Qué opera          | La red completa: conector de Caudal (multiempresa) para sus integrantes, directorio distribuido de llaves, enrutamiento, certificación, conciliación y compensación intra-red, conectada al SIP         |
| Qué la bloquea     | Autorización en compensación (JPRM-2024-029-M, art. 18.1.2); calificación técnica y operativa del BCE (BCE-GG-008-2025, art. 5); **[supuesto]** identificador de la red, enlaces, catálogo técnico y procedimiento de liquidación |
| Criterio de salida | El BCE califica la operación y la red procesa pagos reales de sus integrantes, incluidos inter-red vía SIP, con trazabilidad completa, conciliación diaria y guardia 24/7                             |

El conector de Caudal sigue siendo un componente central, pero es la cara sur de la red Caudal para fintech y cooperativas. Un adaptador hacia una red existente solo se construye ante una oportunidad concreta que tenga autorización, certificación y contrato sin exclusividad; no pertenece a la ruta crítica.

## 2. Infraestructura: dónde y cómo se despliega

### 2.1 Proveedor de nube y lo que la norma exige

Todo corre en un proveedor de nube grande. La recomendación es **AWS como proveedor primario** — es donde el equipo tiene más experiencia operativa y donde el catálogo de servicios gestionados (KMS, Aurora/RDS PostgreSQL, ECS/EKS, CloudHSM) cubre lo que necesitamos sin piezas exóticas.

Lo que la norma exige del despliegue, con su fuente:

| Exigencia                                                                                                                                           | Fuente                                      | Qué significa para la entidad solicitante                                                                                                                                                                                                                                                       |
| --------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Copias de los **contratos vigentes con proveedores críticos** de software, nube, redes y seguridad, nacionales o extranjeros                        | BCE-GG-024-2024, art. 3.13                  | El contrato enterprise con el proveedor de nube (y con el proveedor de monitoreo/seguridad si es tercero) entra al expediente; los documentos extranjeros se apostillan y traducen (art. 11)                                                                                                   |
| **Detalle de infraestructura tecnológica**: capacidad, desempeño y diagrama de red (Anexo 10)                                                       | BCE-GG-024-2024, art. 3.12                  | El diagrama de red y el dimensionamiento que describimos abajo son un entregable del expediente, no documentación interna                                                                                                                                                                      |
| La **visita técnica verifica el centro de datos** además de oficinas y servicios implementados                                                      | BCE-GG-024-2024, art. 12                    | **[supuesto]** que para infraestructura en nube esto se satisface con la documentación del proveedor (certificaciones ISO 27001/SOC 2 de las regiones usadas) más la demostración en vivo de la red Caudal — confirmar con los abogados y, de ser necesario, con el BCE antes de presentar        |
| Seguridad: cifrado en tránsito y reposo, autenticación fuerte, gestión de vulnerabilidades, monitoreo continuo, continuidad, DRP, trazabilidad      | BCE-GG-008-2025, art. 10 (a–g)              | Es la lista de control de las secciones 4 y 5 de este documento                                                                                                                                                                                                                                |
| Los planes de contingencia y continuidad, gestión de riesgos y seguridad de la información se **remiten al BCE anualmente** y en cada actualización | BCE-GG-024-2024, obligaciones del partícipe | Los planes se mantienen como documentos versionados en el repositorio privado, generados desde la realidad operativa, no redactados una vez para el expediente                                                                                                                                 |
| PCI-DSS                                                                                                                                             | BCE-GG-024-2024, arts. 5–6                  | Es condicional al procesamiento de datos de tarjetas. El diseño pretende excluirlos, pero debe confirmarse el alcance de todos los flujos y canales antes de marcar “no aplica”.                                                                                                                                                                               |

No hay requisito publicado de residencia de datos en Ecuador para partícipes del Sistema Auxiliar de Pagos **[supuesto — verificar con abogados junto con la LOPDP]**. Mientras se confirma, la región primaria se elige por latencia y oferta de servicios (us-east-1 o us-east-2 para AWS), con la posición de datos documentada en la política LOPDP (Anexo 15).

### 2.2 Organización de cuentas y red

Una organización (AWS Organizations o equivalente) con cuentas separadas por función — la separación de ambientes se hace **por cuenta, no por etiqueta**:

```text
org/
├── security          Bitácoras centralizadas, detección, acceso de auditoría (solo lectura)
├── shared-services   CI/CD, registro de artefactos, DNS interno
├── dev               Desarrollo y pruebas efímeras
├── sandbox           El ambiente público de integración para miembros (permanente)
├── cert              Certificación: donde corren las corridas del tck contra candidatos a release
└── prod              Producción. Nadie entra a mano
```

Reglas de red y cómputo:

- **VPC por ambiente**, sin peering entre no-productivos y producción. Subredes privadas para todo lo que procesa pagos; lo único expuesto es el borde (balanceador con TLS y WAF). Salida a internet por NAT con egress filtrado por destino — un nodo de pagos no necesita hablar con internet arbitrario.
- **Todo por infraestructura como código** (Terraform), revisado por PR, aplicado por el pipeline. Cambios manuales en producción bloqueados por política (SCPs); el acceso humano de emergencia existe, requiere aprobación, expira y queda en bitácora.
- Cómputo en contenedores (ECS Fargate o EKS) con imágenes firmadas desde CI; base de datos gestionada (RDS/Aurora PostgreSQL) **multi-AZ** con réplica y respaldo automático. Sin servidores mascota.
- El transporte hacia el BCE es una incógnita publicada como pendiente (BCE-GG-008-2025, Disp. Gral. TERCERA: los estándares de conexión al SIP/RPI son un documento interno aún no emitido). El diseño de red reserva el espacio: la conexión norte sale por un módulo de conectividad propio (VPN site-to-site, enlace dedicado o lo que el BCE mandate) sin tocar el resto de la topología.

### 2.3 Dimensionamiento honesto

Ecuador entero, en régimen, opera por debajo de 100 transacciones por segundo; la porción inicial de Caudal son unidades por segundo. El problema de ingeniería es **corrección y disponibilidad, no volumen**: dos nodos activos por servicio detrás del balanceador, Postgres multi-AZ, y capacidad para 100× el tráfico esperado sin cambiar de arquitectura. El Anexo 10 se llena con estos números y con la prueba de carga que los respalda — publicar el benchmark es parte del plan, no marketing.

## 3. Segregación de ambientes

| Ambiente  | Propósito                                                                                                               | Datos                                                 | Acceso                                          |
| --------- | ----------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------- | ----------------------------------------------- |
| `dev`     | Desarrollo diario, ramas, pruebas efímeras                                                                              | Solo sintéticos, generados por seed                   | Equipo de ingeniería                            |
| `sandbox` | Integración de miembros: idéntico a producción en binarios y configuración funcional                                    | Solo sintéticos; cada miembro ve únicamente su tenant | Miembros con credenciales propias, autoservicio |
| `cert`    | Corridas de conformidad (tck) contra candidatos a release; los informes que salen de aquí son los que firma una persona | Sintéticos, fixtures versionados                      | Pipeline + operador de certificación            |
| `prod`    | Pagos reales                                                                                                            | Datos reales, cifrados, enmascarados en toda salida   | Ningún acceso humano directo; ver abajo         |

Reglas que no se negocian:

1. **Los datos de producción nunca bajan a otro ambiente.** Ni "anonimizados", ni "solo un extracto para reproducir el bug". Depurar requiere telemetría, bitácora y el journal — que para eso existen. Los datos de prueba son sintéticos con generadores versionados (cédulas con dígito verificador válido, nombres ficticios, BICs de fantasía).
2. **Mismos binarios en los cuatro ambientes.** Lo que cambia es configuración y credenciales. Un sandbox que comparte cero código con producción no certifica nada — esta regla ya está en el diseño de Caudal y aquí se vuelve operativa.
3. **Credenciales, llaves y certificados por ambiente, sin excepción.** Una credencial de sandbox no puede autenticar en producción ni por accidente: emisores distintos, cuentas distintas, KMS distintos.
4. **Promoción solo por pipeline**: merge a main → build firmado → despliegue a `dev` → suite completa → `sandbox`/`cert` → corrida tck → aprobación humana registrada → `prod` con despliegue gradual y reversión automática si fallan las señales de salud.
5. **Producción sin manos.** Operar es usar la consola (auditada, con roles) o ejecutar un runbook por pipeline. La cuenta de emergencia (break-glass) pide dos personas, avisa a todos y expira en horas.

## 4. Seguridad de datos, mapeada al art. 10 de la BCE-GG-008-2025

El perfil de mensajería lleva la identidad completa de ordenante y beneficiario en cada `pacs.008`: una red concentra más datos personales financieros que cualquier otro actor del esquema salvo las instituciones mismas. Esto es lo que implementamos, componente por componente del art. 10:

**(a) Cifrado en tránsito y en reposo.**

- _Tránsito_: TLS 1.2+ en todo salto interno y externo; **mTLS con anclaje de certificado por contraparte** en las conexiones de miembros y del SIP; además **firma separada de mensajes** (estilo JWS, como Pix), para que la autenticidad sobreviva a la terminación TLS del balanceador. El transporte hacia el BCE queda enchufable hasta que publique su estándar.
- _Reposo_: cifrado en todo volumen, base, respaldo, cola y bucket, con llaves del KMS **por ambiente y por clase de dato**. Los campos de identidad (documento, nombre, cuenta) llevan además cifrado a nivel de aplicación con contexto por tenant, de modo que un vuelco de la base no expone credenciales de pago legibles.

**(b) Control de accesos y autenticación fuerte.** SSO con MFA obligatorio para humanos; roles de mínimo privilegio revisados trimestralmente; identidades de máquina por servicio (IAM roles), nunca llaves estáticas en configuración. Entre sistemas: mTLS + firma. En la consola: roles por función, y el personal del miembro ve solo su institución (aislamiento por tenant reforzado en la base con row-level security, no solo en la aplicación).

**(c) Gestión de vulnerabilidades.** Escaneo de dependencias e imágenes en CI que **bloquea el release**, no que abre un ticket; parcheo de base gestionado por el proveedor; ventana definida para críticos (72 h) **[política de la sociedad operadora de Caudal]**; una prueba de penetración externa anual y antes de la salida a producción — el informe alimenta el Anexo 12.

**(d) Monitoreo continuo de eventos de seguridad.** Bitácoras de la nube (CloudTrail o equivalente) hacia la cuenta `security`, inmutables para todos incluidos administradores; detección gestionada (GuardDuty o equivalente) con alertas al mismo canal de guardia que lo operativo; alertas específicas para lo que de verdad nos ataca: uso de credencial fuera de patrón, cambio de infraestructura fuera del pipeline, certificado por vencer, firmas inválidas repetidas de una contraparte.

**(e–f) Continuidad y DRP** — sección 5.

**(g) Registro y trazabilidad.** La bitácora de auditoría del producto: append-only, escrita en la misma transacción que el cambio que describe, encadenada por hash, suficiente para reconstruir cualquier pago años después sin acceso a producción. Las acciones de agentes de IA entran a la misma bitácora que las humanas. **Ningún dato personal en telemetría**: correlation ids y hashes, jamás cuentas ni documentos; enmascarado por defecto en consola y logs (política LOPDP, Anexo 15).

Las llaves privadas de firma viven en KMS/HSM detrás de una interfaz enchufable, nunca en base, configuración ni repositorio; rotación diseñada desde el primer release con vencimientos visibles en consola — un certificado vencido a las 3 a. m. es el incidente más probable, no el más sofisticado.

## 5. DRP y plan de continuidad del negocio

La norma acepta metodologías alineadas con ISO 22301 en lugar de la certificación (BCE-GG-024-2024, art. 3.17, Anexo 13), y los planes se remiten al BCE anualmente. Los objetivos, heredados de la barra técnica de Caudal:

| Objetivo                     | Valor                                                                                                                    | Justificación                                                                               |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------- |
| RPO (pérdida de datos)       | **Cero** para todo lo financiero                                                                                         | Ningún pago confirmado ni asiento puede perderse: durabilidad síncrona antes de responder   |
| RTO (tiempo de recuperación) | **< 15 minutos** ante falla de zona; **< 4 horas** ante falla regional **[supuesto de Caudal, presupuesto de ingeniería]** | Una red caída deja fuera a todos sus miembros a la vez; el 99,95 % mensual es piso, no meta |
| Disponibilidad               | ≥ 99,95 % mensual, sin ventanas de mantenimiento                                                                         | Actualizaciones continuas, migraciones compatibles hacia atrás por al menos un release      |

**Escalones de defensa técnica:**

1. **Falla de nodo**: dos instancias activas por servicio, sin estado de sesión que un reinicio pierda; el balanceador saca al nodo enfermo. Sin intervención humana.
2. **Falla de zona (AZ)**: cómputo repartido entre zonas; Postgres multi-AZ con conmutación automática. El RTO de 15 minutos se mide aquí, con simulacro trimestral.
3. **Falla regional**: respaldos continuos + réplica a una **segunda región**, con la infraestructura de recuperación definida en Terraform y levantable en horas (pilot light). La conmutación regional es decisión humana documentada en runbook, no un automatismo.
4. **Corrupción o error lógico** (el caso que los planes olvidan): respaldos point-in-time con retención de 35 días más archivos mensuales; el journal y la bitácora permiten reconstruir estado sin confiar únicamente en el respaldo binario.

**Un respaldo no probado no existe**: restauración completa ensayada trimestralmente en una cuenta aislada, con tiempo medido y reporte archivado — ese reporte es evidencia directa para el Anexo 13 y la supervisión.

**Continuidad del negocio (las personas y los procesos):**

- **Guardia 24/7** con rotación definida, escalamiento en dos niveles y un canal de comunicación fuera de la infraestructura primaria (si la nube está caída, el chat corporativo no puede ser la única vía).
- **Runbooks en español**, versionados junto al código, para los escenarios nombrados: miembro desconectado, contraparte sin responder, certificado vencido, conmutación de zona y de región, restauración, pago por conciliar que no cierra, incidente de seguridad con posible fuga de datos (este último con los pasos de notificación de la LOPDP).
- **Matriz de dependencia de personas**: con un equipo de pocos, cada función crítica tiene titular y respaldo nombrados; la pérdida de una persona no puede dejar sin operador la conciliación diaria ni la guardia.
- **Comunicación en incidente**: quién notifica a los miembros afectados, en qué plazo y por qué canal (obligación de los acuerdos de nivel de servicio), y quién notifica al BCE cuando el incidente lo amerita.
- **Simulacro anual de continuidad completo** (escenario de pérdida regional + indisponibilidad de una persona clave), además de los simulacros técnicos trimestrales. Las actas alimentan el plan que se remite al BCE.

## 6. Secuencia de construcción

Las semanas son objetivos de ejecución, no promesas regulatorias. Cada etapa termina en una condición verificable.

### Etapa 0 — Cimientos (semanas 1–2)

| Entregable                          | Detalle técnico                                                                                                                | Prueba de salida                                                                                                  |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------- |
| Organización de nube y landing zone | Cuentas por ambiente, SCPs, bitácoras centralizadas, Terraform con estado remoto y CI que aplica por PR                        | Un cambio de infraestructura llega a `dev` solo por pipeline; un intento manual en `prod` es bloqueado y alertado |
| Repositorio ejecutable              | Módulo Go, CI (build, lint, pruebas), migraciones Postgres, Compose local, `make check`                                        | Un clon limpio corre `make check` sin credenciales externas en menos de 15 minutos                                |
| Identidades de desarrollo           | Certificados y llaves sintéticos generados localmente; la configuración rechaza material de desarrollo en perfil de producción | Arranque con llave de desarrollo y perfil `prod` falla ruidosamente                                               |
| Simulador de contraparte            | Stubs de integrante y SIP: mensajes válidos, rechazos por código, lentitud, duplicados, desconexión                            | Los escenarios de caos son repetibles por comando                                                                 |
| Dependencias externas en marcha     | Solicitudes por catálogo técnico, estándares de conexión, BIC, cámara y calificación de la red                                 | Cada gestión con responsable, fecha y evidencia en [`open-questions.md`](../regulatory/open-questions.md)         |

### Etapa 1 — Núcleo de pagos (semanas 2–4)

| Entregable                  | Detalle técnico                                                                                                                           | Prueba de salida                                                                                             |
| --------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| Modelo de dominio y journal | `member`, conexión, pago con estado, mensaje con hash y correlación, bitácora append-only encadenada en la misma transacción              | Toda transición reconstruible desde el journal; pruebas de concurrencia sin estados imposibles               |
| Máquina de estados del pago | Estados y códigos del Catálogo Operacional §5; idempotencia por (BIC, identificador extremo a extremo); timeouts parametrizados por etapa | Archivos dorados por cada aceptación, rechazo e irrevocabilidad; el mismo mensaje dos veces = un solo efecto |
| Reservas y obligaciones     | Reserva en dos fases, confirmación, reversión solo donde la norma la permite; interfaz hacia la compensación intra-red                    | Pruebas de propiedades: nunca saldo negativo, asiento desbalanceado ni doble efecto                          |
| Cola de conciliación        | Ítems _por conciliar_ con dueño, plazo y cierre diario; un timeout jamás libera la reserva por sí solo                                    | Inyección de timeout posterior a irrevocabilidad: la reserva queda, el caso se crea, el cierre lo drena      |

### Etapa 2 — Conector de Caudal, multiempresa (semanas 3–6, en paralelo)

| Entregable               | Detalle técnico                                                                                                                | Prueba de salida                                                                                            |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------- |
| API sur (JSON)           | Autenticación por miembro, límites por cliente, webhooks firmados, contrato versionado con SDKs generados                      | Un cliente ficticio integra pagos de punta a punta sin leer una línea de ISO 20022                          |
| Adaptador ISO 20022      | Lectura/escritura/validación de `pacs.008`, `pacs.002`, `acmt.*` con firma separada; perfil nacional como configuración        | Golden files; pruebas negativas de campo, versión, firma y correlación; fuzzing del parser XML en CI        |
| Enrutamiento             | Reenvío síncrono con presupuesto por salto (p99 ≤ 5 s de punta a punta contra el techo de 20 s); sin colas en la ruta del pago | E2E en sandbox bajo presupuesto; timeout de contraparte termina en conciliación, nunca en rechazo inventado |
| Aislamiento multiempresa | Tenant por miembro en datos (row-level security), credenciales, configuración y consola                                        | Prueba de autorización cruzada: un miembro no puede leer nada de otro, ni por API ni por consola            |

### Etapa 3 — Llaves y directorio (semanas 5–8)

| Entregable                   | Detalle técnico                                                                                                                            | Prueba de salida                                                                             |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------- |
| Ciclo de vida de llaves      | Registro, resolución, suspensión, habilitación, anulación con los códigos del Catálogo §3; credenciales enmascaradas en toda respuesta     | Un caso por cada código y estado de las Tablas 2–9; ninguna identidad completa en logs       |
| Reserva en dos fases         | Máquina de estados explícita para el registro coordinado con el registro central (simulado); reintentos que jamás dejan reservas huérfanas | Inyección de falla en cada paso del protocolo: ninguna llave huérfana ni doble confirmación  |
| Sincronización y divergencia | Estado `Por conciliar` para llaves, reporte de divergencias, flujo de reparación                                                           | El simulador introduce discrepancias; la llave se aparta y se repara por el flujo, no a mano |

### Etapa 4 — Certificación, operación y seguridad (semanas 6–10)

| Entregable                 | Detalle técnico                                                                                                            | Prueba de salida                                                                                         |
| -------------------------- | -------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| Kit de conformidad (`tck`) | Casos ejecutables contra un endpoint candidato, informe versionado de aprobado/fallido; la atestación la firma una persona | Miembro conforme aprueba; timeout, XML inválido y duplicado fallan con explicación legible               |
| Consola y runbooks         | Búsqueda por pago, salud de conexiones, vencimientos de certificados, bandeja de conciliación; toda acción auditada        | Simulacro: un operador localiza cualquier pago en < 5 minutos sin tocar la base de datos                 |
| Observabilidad             | Métricas por salto contra el presupuesto de latencia, trazas con correlation id, alertas hacia la guardia                  | Caída provocada de un miembro dispara la alerta correcta; auditoría de telemetría: cero datos personales |
| Seguridad y DRP en vivo    | Todo lo de las secciones 4 y 5 desplegado: KMS, mTLS, detección, respaldos, restauración                                   | Primer simulacro de restauración y de conmutación de zona, con tiempos medidos y reporte archivado       |

### Etapa 5 — Preproducción de la red Caudal (semanas 10–12)

| Entregable                     | Detalle técnico                                                                                        | Prueba de salida                                                                                                        |
| ------------------------------ | ------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------- |
| Perfil de red contra simulador | Interfaz de integrantes, directorio, enrutamiento y compensación ejecutados de punta a punta           | Suite de conformidad propia aprobada; evidencia archivada                                                               |
| Operación ensayada             | Incorporación de dos miembros de diseño, acuerdos de nivel de servicio, guardia activa y cierre diario | Ensayo completo con datos y fondos sintéticos; ningún pago real antes de la calificación                                |
| Coherencia con el expediente   | La infraestructura desplegada coincide con el Anexo 10 y el esquema operativo presentados              | Revisión interna contra [`dossier-psap.md`](../regulatory/dossier-psap.md); ensayo de la demostración de visita técnica |

**Salida de preproducción** = software demostrable, dos integrantes precertificados y operación ensayada. No habilita pagos reales.

### Etapa 6 — Conexión y calificación (fecha dependiente del BCE)

| Entregable                                                             | Bloqueado por                                                                                 |
| ---------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| Cara norte SIP: transporte, certificados y perfil del catálogo técnico | Catálogo técnico y estándares de conexión sin publicar (BCE-GG-008-2025, Disp. Gral. TERCERA) |
| Enrutamiento de red con las cuatro excepciones del Catálogo §11        | Calificación técnica y operativa del BCE                                                      |
| Compensación intra-red: posiciones, malla, cierre, salida a la cámara  | Procedimiento e interfaz de liquidación (open-questions P4)                                   |
| Directorio sincronizado con el Sistema Central de Llaves               | Interfaz central real y autorización                                                          |

**Salida a producción** = autorización otorgada, calificación técnica y operativa completada, conexión real habilitada y puertas técnicas, contractuales y operativas cerradas. No fijamos fecha para dependencias que el BCE no ha entregado.

### Oportunidad opcional — Adaptador hacia otra red

No tiene etapa ni fecha dentro de esta secuencia. Solo se incorpora si una contraparte lo solicita y la revisión legal y comercial confirma autorización clara, certificación de la red, reutilización del producto y ausencia de exclusividad o barreras para la futura membresía. Su construcción nunca desplaza una entrega de las etapas 0–6.

## 7. Pruebas necesarias

| Nivel                | Pregunta que responde                            | Ejemplos                                                         |
| -------------------- | ------------------------------------------------ | ---------------------------------------------------------------- |
| Unidad y propiedades | ¿Se preservan los invariantes?                   | Asientos balanceados, idempotencia, aislamiento de tenants       |
| Archivos dorados     | ¿Cambió un byte de un mensaje conocido?          | `pacs.008`, `pacs.002`, `acmt.*`, códigos del catálogo           |
| Fuzzing              | ¿El parser aguanta entrada hostil?               | XML, QR, JSON — permanente en CI                                 |
| Integración          | ¿Los contratos entre componentes se cumplen?     | API ↔ adaptador ISO, directorio ↔ registro central simulado      |
| Extremo a extremo    | ¿El pago completa su recorrido bajo presupuesto? | Llave → confirmación → irrevocabilidad, p99 ≤ 5 s                |
| Caos                 | ¿Falla con corrección?                           | Timeout, duplicado, firma adulterada, caída de zona              |
| Operación            | ¿Una persona responde sin improvisar?            | Restauración, rotación de certificado, conmutación, conciliación |
| Conformidad externa  | ¿El BCE acepta la red y su operación?            | Calificación técnica y operativa del BCE                         |

## 8. Ruta crítica

```text
Landing zone + repositorio ejecutable
  → motor: estados, reservas, conciliación        (sin dependencia externa)
  → conector de Caudal + adaptador ISO + sandbox   (sin dependencia externa)
  → llaves y directorio contra simulador           (sin dependencia externa)
  → tck + consola + seguridad y DRP en vivo        (sin dependencia externa)
  → dos miembros de diseño precertificados
  → cara norte SIP + directorio conectado + compensación intra-red
  → [PUERTA EXTERNA] autorización + calificación técnica y operativa del BCE
  → PRODUCCIÓN — red Caudal
```

Todo lo controlable termina en la semana ~12 con dos miembros precertificados sobre una plataforma segura, respaldada y operable. El catálogo técnico, los estándares de conexión, el BIC, la cámara, la autorización y la calificación se persiguen desde el primer día y se revisan cada semana en [`../plan.md`](../plan.md) y [`../regulatory/open-questions.md`](../regulatory/open-questions.md).

## 9. Tablero semanal de decisión

Se responde con evidencia, no con porcentajes:

| Pregunta                                                                | Evidencia aceptable                                               | Dueño              |
| ----------------------------------------------------------------------- | ----------------------------------------------------------------- | ------------------ |
| ¿`make check` y la suite E2E pasan en limpio?                           | Corrida de CI del día                                             | Ingeniería         |
| ¿El tck aprueba al conforme y reprueba al no conforme?                  | Ambos informes                                                    | Ingeniería         |
| ¿Podemos reconstruir y conciliar cualquier pago del sandbox?            | Ejercicio de operador documentado                                 | Operaciones        |
| ¿El último simulacro de restauración/conmutación cumplió el RTO?        | Reporte con tiempos                                               | Operaciones        |
| ¿La infraestructura desplegada coincide con el Anexo 10 del expediente? | Revisión cruzada mensual                                          | Ingeniería + legal |
| ¿Dos miembros de diseño siguen comprometidos?                           | Contrato o carta de intención (referenciada, nunca en git)        | Comercial          |
| ¿Qué respondió el BCE esta semana?                                      | Oficio o correo registrado fuera de git; estado en open-questions | Legal / dirección  |

Una salida a producción se declara únicamente cuando sus puertas técnicas, contractuales y regulatorias están todas cerradas.
