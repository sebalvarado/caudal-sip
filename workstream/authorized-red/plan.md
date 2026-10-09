# Red de pagos Caudal — Plan de ejecución para la autorización

**Meta controlable**: expediente presentado ante el BCE a **mediados de octubre de 2026**; servicio demostrable, contratos comerciales y primeros procesos de incorporación antes del **30 de noviembre de 2026**, cuando toda red y todo integrante debe hablar el estándar ISO 20022 para pagos en tiempo real intra-red (BCE-GG-007-2026, disposición transitoria única). Obtener la autorización y la calificación técnica para esa fecha es una aspiración sujeta a los tiempos del BCE, no un hito que podamos prometer.

**Estrategia**: tramitar y construir la red de pagos Caudal desde el inicio. La conectividad hacia una red existente no es una fase ni una dependencia del plan; solo se evalúa ante una oportunidad concreta, regulatoriamente clara y sin exclusividad que perjudique la futura membresía en la red Caudal.

## Por qué este cronograma es viable

La propia BCE-GG-024-2024 lo permite:

1. **Una entidad que aún no opera puede solicitar.** Art. 3.6: las entidades con menos de dos años de operación — o que no han iniciado operaciones — presentan solo los estados financieros del mes inmediato anterior. La calificación no exige trayectoria.
2. **Una sola solicitud, varios servicios.** El art. 3.1 contempla pedir "el o los servicios" en una misma solicitud. La estrategia por validar con los abogados es presentar juntos **procesamiento de medios de pago electrónicos + switch transaccional para servicios de pago + compensación**. La última categoría es la que habilita a una administradora de red (JPRM-2024-029-M, art. 18.1.2); la secuencia con la cámara especializada y la calificación técnica todavía debe confirmarse.
3. **Las certificaciones ISO no son obligatorias.** Arts. 3.14, 3.16 y 3.17: se aceptan metodologías documentadas alineadas con ISO 31000 / 27001 / 22301 en lugar de la certificación. Semanas de redacción, no meses de auditorías.
4. **Los artefactos más difíciles del expediente ya existen en este repositorio.** El esquema operativo (art. 3.11) y el detalle de infraestructura (art. 3.12) se redactan a partir de `docs/architecture/red.md`, `blueprint.md` y `data-model.md`. La evidencia de "servicios implementados" para la visita técnica (art. 12) es la red simulada + conector de Caudal para sus integrantes + consola funcionando en vivo.
5. **PCI-DSS es condicional.** Los arts. 5 y 6.1 lo exigen cuando el servicio procesa datos de tarjetas. La expectativa es no incluirlos, pero debe cerrarse el alcance de todos los flujos y canales antes de marcar el requisito como “no aplica”.

## Lo que no depende de Caudal — y su mitigación

| Fuera del control de Caudal                                                        | Mitigación                                                                                                                                         |
| ---------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| La revisión de autorización tiene un término de 30 días, pero la calificación técnica y la salida a producción no tienen plazo publicado | Presentar temprano y completo; responder cualquier subsanación dentro del término de 10 días; mantener el rastro documental vía LOTAIP |
| El catálogo técnico sigue sin publicarse; el estándar de conexión no está asignado | La cara norte queda detrás de una interfaz con un simulador (la disciplina que el repositorio ya impone); asiento en los foros para verlo temprano |
| **[supuesto]** Identificador de la red, certificados y conectividad; el art. 6 solo regula el identificador de integrantes | Consultar temprano y por escrito qué identificación requiere la administradora y cómo se emite |
| El arreglo de liquidación con la CCE para compensación (art. 9)                    | Plantearlo como pregunta formal ahora, no después de la autorización                                                                               |

**Contingencia**: si la autorización de compensación y la calificación de la red Caudal se atrasan más allá de noviembre, vender software, sandbox, precertificación y acompañamiento sin operar pagos reales. La conectividad hacia una red existente solo se considera si una contraparte la propone, se obtiene la autorización aplicable, la red acepta certificarla y el contrato preserva la libertad de la institución para elegir la red Caudal después; no debe contarse como ingreso asegurado.

## Los cinco frentes en paralelo

### Frente A — Sociedad y legal (arranca ya, semanas 1–3)

- Ejecutar la secuencia de [`regulatory/ruta-autorizacion-bce.md`](regulatory/ruta-autorizacion-bce.md) y mantener actualizadas sus puertas y dependencias.
- Constituir la sociedad; **objeto social exclusivo para actividades fintech** (Ley Fintech, art. 7.2), redactado para cubrir todos los servicios del art. 23 que la entidad solicitante pueda solicitar (JPRM-2024-018-M).
- Contratar asesoría legal de pagos desde el primer día con las preguntas de `regulatory/open-questions.md` — confirmar la combinación y secuencia de servicios de la P1 y resolver la P2 (capital).
- El representante legal debe pasar la declaración juramentada del art. 3.2 (sin moras, sin antecedentes) — verificarlo antes de nombrarlo.
- Dejar listo el notario y recolectar los certificados mensuales (SRI, IESS, Superintendencia, buró) **al final**, porque cada uno debe ser del mes de la presentación o del anterior.

### Frente B — El expediente (semanas 1–8, la ruta crítica que sí controlamos)

La secuencia está en `regulatory/ruta-autorizacion-bce.md`; la lista completa con responsables y estado, en `regulatory/dossier-psap.md`. Lo que más redacción exige y arranca ya, en paralelo:

- Plan de negocios (Anexo 8) y proyección financiera (Anexo 9)
- Esquema operativo por servicio (art. 3.11) — a partir de los documentos de arquitectura del repositorio
- Riesgos operativos (alineado con ISO 31000, Anexo 11), seguridad de la información (con 27001, Anexo 12), continuidad (con 22301, Anexo 13)
- Metodología de riesgo de liquidez (art. 3.15) — aprovechar el análisis del techo de liquidez que ya está en el resumen del catálogo, §8
- Manual de buen gobierno corporativo (Anexo 14), manual de atención a clientes (art. 3.19), política de protección de datos (Anexo 15)
- Registro y certificado de la UAFE (art. 3.23) — empezar temprano, es una dependencia externa

### Frente C — Comercial (semanas 1–8, dentro de la ruta crítica regulatoria)

El art. 6.2 exige **contratos de adhesión firmados con partícipes del Sistema Auxiliar de Pagos a los que se prestará el switch, con acuerdos de nivel de servicio**, dentro de la solicitud. Un prospecto no elegible puede validar demanda, pero no cuenta para este requisito. Prospectos y seguimiento en `commercial/pipeline.md`: 2–3 fintech/SEDPES más una caja central o un proveedor de core bancario. Oferta principal: futura membresía en la red Caudal + precertificación (tck) + aceleración de licenciamiento SEDPE. Un conector hacia otra red no forma parte de la prospección activa.

### Frente D — Preparación técnica (semanas 1–12)

Lo que la visita técnica debe ver funcionando está mapeado en `technical/readiness.md`: conector de Caudal (multiempresa), red simulada de punta a punta, consola de operación, y la disciplina de auditoría y reportes que el expediente describe. El binario `red` sigue el orden de construcción del repositorio; su cara norte queda simulada hasta que salga el catálogo técnico.

### Frente E — Relación con el BCE (semana 1 en adelante)

Los tres contactos ya redactados en `docs/plan/action-plan.md`, enviados esta semana: la solicitud LOTAIP por el catálogo técnico; la pregunta por la inconsistencia de versión del `pacs.008`; la participación en los foros de interoperabilidad. Se suma un cuarto, propio de este frente: confirmar la secuencia entre autorización de compensación, acceso a la cámara especializada y calificación técnica como red (P1 de open-questions).

## Calendario

| Cuándo          | Hito                                                                                                                                                                           |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 18–31 ago       | Sociedad constituida, abogados contratados, cartas al BCE enviadas, representante legal verificado, redacción del expediente iniciada                                          |
| 1–30 sep        | Todos los documentos del expediente redactados; contratos de adhesión o cartas de intención firmados; sandbox + conector de Caudal demostrables de punta a punta               |
| 1–15 oct        | Certificados mensuales frescos; **solicitud presentada** (procesamiento + switch transaccional + compensación, sujeto a validación legal)                                           |
| 15 oct – 30 nov | Subsanación respondida dentro de los 10 días si la piden; visita técnica atendida; operación 24/7, runbooks y monitoreo en vivo; primeros clientes en onboarding en el sandbox |
| 30 nov          | Plazo de las instituciones. Definición de éxito de Caudal: **presentados, completos, demostrables y vendiendo** — con la autorización en mano si el reloj del BCE lo permite    |

## A quién involucrar ya

Abogados de pagos (con práctica en la BCE-GG-024-2024 — NMS y PBP han publicado sobre la norma), notario, el representante legal, enlace con la UAFE, la AEFI (presentaciones con fintech), una caja central o proveedor de core bancario (distribución + primeros contratos), y la Gerencia de Medios de Pago y Servicios Nacionales del BCE (por los canales escritos de arriba).
