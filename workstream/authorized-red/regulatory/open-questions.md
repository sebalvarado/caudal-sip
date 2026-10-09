# Caudal — Preguntas legales y regulatorias abiertas

Cada pregunta condiciona una decisión del plan. Ruta = cómo conseguimos la respuesta; el responsable se llena a medida que se involucra a la gente.

La secuencia operativa y la puerta que bloquea cada pregunta están consolidadas en [`ruta-autorizacion-bce.md`](ruta-autorizacion-bce.md), sección de dependencias externas.

## P1 — ¿Qué autorización te convierte en "red de pagos"? **Resuelta en la norma**

La correspondencia sí consta en la JPRM-2024-029-M, art. 18.1.2: pueden actuar como administradoras de redes de pagos los partícipes del Sistema Auxiliar de Pagos **previamente autorizados en el servicio de compensación** que procesan transferencias de pago. La BCE-GG-008-2025, art. 5, agrega una calificación técnica y operativa del BCE antes de la conexión al SIP.

Lo pendiente no es la categoría principal sino la estrategia completa de presentación: si conviene solicitar compensación junto con procesamiento y switch transaccional, y qué requisitos operativos adicionales impondrá el BCE para la cámara de compensación especializada (BCE-GG-024-2024, art. 9).

**Ruta**: criterio de los abogados + pregunta escrita al BCE sobre la combinación de servicios que debe cubrir el acto administrativo y la secuencia entre autorización, cámara especializada y calificación técnica.

## P2 — ¿Existe un capital mínimo? *(condiciona cuánto capital levantar)*

La BCE-GG-024-2024 no trae ninguno para los partícipes de switch o procesamiento; la vara es la proyección de viabilidad financiera (art. 3.10). Las SEDPES sí tienen reglas de capital por la Ley Fintech y las resoluciones de la JPRF, pero la entidad solicitante no aplicará como SEDPE.

**Ruta**: que los abogados confirmen que no hay un piso de capital escondido en el texto completo de la JPRM-2024-018-M o en la regulación de la JPRF.

## P3 — ¿Qué ruta exige la calificación previa como "entidad de servicios auxiliares del sistema financiero"? *(condiciona el cronograma)*

Para solicitar autorización de **procesamiento de medios de pago electrónicos**, la BCE-GG-024-2024, art. 5.1, exige documentación que acredite la calificación como entidad de servicios auxiliares del sistema financiero, emitida por el organismo de control correspondiente. El art. 3.21 exige lo mismo a las entidades de ese tipo dentro de los requisitos generales.

Falta determinar qué organismo califica a una sociedad fintech nueva, bajo qué categoría y si el trámite debe concluir antes de presentar al BCE. Esa respuesta define si existe un segundo regulador en la ruta crítica.

**Ruta**: abogados + consulta al organismo de control que corresponda, con urgencia.

## P4 — ¿Cómo llega la liquidación intra-red al dinero del banco central? **Resuelta en principio; falta la interfaz**

La JPRM-2024-029-M resuelve la arquitectura: cada red compensa sus operaciones intra-red (art. 48.14) y los pagos intra e inter-red se liquidan en las cuentas corrientes que mantienen los integrantes en el BCE (art. 48.15). Esto confirma que el perfil de red necesita registrar obligaciones, calcular posiciones y producir el resultado de compensación.

Lo que sigue sin publicarse es el procedimiento operativo: formato, frecuencia, transporte y controles para remitir esas posiciones a la cámara de compensación especializada.

**Ruta**: foros de interoperabilidad + pregunta escrita sobre la interfaz de liquidación; contrastar también con la operación actual de Pago Directo.

## P5 — ¿Qué exige operativamente la conexión al SIP? *(condiciona la salida a producción)*

Transporte, autoridad certificadora, direccionamiento, parámetros de timeout y el catálogo técnico (esquemas campo por campo): todo sin publicar. **[supuesto]** La administradora podría requerir un identificador propio, pero el art. 6 de la BCE-GG-008-2025 solo regula el BIC o código alternativo de los integrantes; falta confirmar cómo se identifica la red. El repositorio mantiene la cara norte detrás de una interfaz justamente por esto.

**Ruta**: solicitud LOTAIP (ya redactada en `docs/outreach/`), participación en los foros, y la pregunta por la versión del `pacs.008` como puerta de entrada.

## P6 — Los techos de liquidez de las SEDPES *(condiciona la propuesta de valor para los miembros fintech)*

Las SEDPES tienen un encaje del 0,5 % y un factor de riesgo del 100 %. Además, al encaje se le añade un porcentaje según su nivel de riesgo (Catálogo §10, tabla 19 y nota 6). La fórmula de ese porcentaje no está publicada. Si un integrante SEDPE llega al rojo, el SIP le bloquea los pagos salientes y la rehabilitación es manual en el BCE. Para la sociedad operadora de Caudal, el monitoreo, las alertas y el manual de manejo de liquidez son una funcionalidad central del producto.

**Ruta**: pregunta escrita al BCE; la simulación señalada en el resumen del catálogo (§8) es el artefacto para llevar a los foros.

## P7 — ¿Cuál es el perímetro AML de la red? *(condiciona cumplimiento y costos operativos)*

El KYC del cliente final es de cada integrante — la red no tiene clientes finales. Pero la red ve pasar la identidad completa de ordenante y beneficiario en cada pago (el perfil de mensajería la lleva en cada `pacs.008`, Catálogo §8), y la sociedad operadora de Caudal debe presentar el certificado UAFE para autorizarse (BCE-GG-024-2024, art. 3.23). Lo que falta precisar: si una red de pagos es **sujeto obligado** ante la UAFE con deberes propios de reporte (operaciones inusuales que observa en el tráfico, umbrales, ROII), o si esos deberes recaen exclusivamente en los integrantes y la red solo colabora a requerimiento. La respuesta define si la sociedad operadora de Caudal necesita un oficial de cumplimiento con funciones de reporte desde el día uno y qué monitoreo de tráfico debe montar.

**Ruta**: abogados (normativa UAFE aplicable a partícipes del Sistema Auxiliar de Pagos) + consulta directa a la UAFE; averiguar también qué hace BANRED hoy.

## P8 — ¿Puede una misma entidad ser SEDPE y red de pagos? *(condiciona la estructura societaria del grupo)*

Lectura preliminar: **no en una sola entidad**. La SEDPE tiene objeto social **único** — recepción de recursos con fines exclusivos de facilitar pagos (JPRM-2024-018-M, art. 25) — y solo puede prestar servicios de partícipe del Sistema Auxiliar de Pagos "exclusivamente para las operaciones con sus clientes" (art. 27, párrafo final), mientras que administrar una red exige autorización en compensación y consiste en servir a instituciones terceras (JPRM-2024-029-M, art. 18.1.2). En el esquema, las SEDPES son **integrantes** de las redes (art. 18.1.16), no administradoras. La figura de STP en México (IFPE conectada directamente al SPEI) no se traslada: en Ecuador el acceso al riel pasa por una red, y la SEDPE entra como miembro.

**[supuesto]** La consecuencia práctica: la posición de STP se replica con un **grupo de dos entidades** — la sociedad operadora de Caudal como PSAP/administradora (sin custodia) y una SEDPE hermana como integrante de la red de pagos Caudal (el H3 del plan de negocio). Falta confirmar si existe alguna restricción de vinculación societaria entre la administradora de una red y sus integrantes (conflicto de interés, neutralidad — la JPRM-2024-029-M exige neutralidad de la interoperabilidad respecto de la administradora, art. 48.4.d).

**Ruta**: abogados — objeto social de ambas entidades, restricciones de vinculación entre administradora e integrante, y el orden de constitución si el grupo se arma en fases.

## P9 — ¿Los montos máximos de billetera permiten tesorería corporativa en una SEDPE? *(condiciona el H3 del plan de negocio)*

El modelo de cuentas para clientes corporativos (dispersión de nómina, pagos a proveedores, cobranza — el modelo de STP en México) es viable en el texto: nada restringe las cuentas de pago electrónico a personas naturales (JPRM-2024-018-M, arts. 27–28; la definición de compensación contempla "personas naturales o jurídicas", y la Disp. Gral. Tercera obliga a todo titular de RUC a tener al menos un medio de pago electrónico o billetera). Los fondos quedan 100% en el BCE o en entidades con encaje (art. 31) — buen argumento de tesorería sin riesgo de crédito.

Lo que falta: el art. 12 remite los **montos máximos de operación** de las billeteras a los topes "establecidos por el BCE", y esos topes no constan en el corpus de Caudal. Si están calibrados para consumidores (saldos o montos diarios bajos), la tesorería corporativa no cabe en una billetera y el H3 necesita otra figura. También pesa la P6: los techos de liquidez de las SEDPES son los más estrechos del sistema, y una dispersión masiva de nómina puede llevar la SEDPE al bloqueo automático del SIP — modelarlo antes de vender el caso.

**Ruta**: abogados + consulta escrita al BCE por los montos máximos vigentes para cuentas de pago electrónico de personas jurídicas; capital y requisitos SEDPE con la regulación de la JPRF; la simulación de techos de liquidez de la P6 extendida al escenario de dispersión corporativa.
