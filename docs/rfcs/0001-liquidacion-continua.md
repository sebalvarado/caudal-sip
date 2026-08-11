# RFC 0001: Liquidación continua para un sistema que ya se declara 24/7

- **Estado**: Borrador
- **Autor**: Proyecto Caudal (mantenedor fundador)
- **Creado**: 2026-08-11
- **Periodo de comentarios**: hasta el 2026-08-25
- **Implementación**: pendiente

> **English abstract.** Ecuador's operational catalogue defines the SIP as operating 24/7/365, but settlement occurs only in BCE accounts during parameterized operating hours, and payments ordered outside those hours settle on the next business day. Because an institution's liquidity headroom is only restored when a netting cycle settles, its entire weekend sending capacity is bounded by a weekly, reserve-derived ceiling — and when that ceiling is exhausted the SIP blocks the institution automatically, while re-enablement is manual and performed by a BCE administrator. A 24/7 system therefore has a business-hours recovery mechanism. This RFC documents the contradiction with citations, and proposes prefunded 24/7 settlement accounts at the central bank, following Pix's *Contas PI*, FedNow, and UK Faster Payments — every major 24/7 scheme solved this the same way. Two lower-cost fallbacks are also specified.

## Resumen

El catálogo operacional define el SIP como un sistema que opera **las 24 horas, los 7 días de la semana, todos los días del año**. Pero la liquidación ocurre en las cuentas de los integrantes en el BCE, dentro de un horario de operaciones parametrizado, y las órdenes de pago cursadas fuera de ese horario se liquidan **el primer día laborable siguiente**.

Como el cupo de liquidez de un integrante solo se recompone cuando liquida una malla, la consecuencia es que **toda la capacidad de envío de un fin de semana está acotada por un techo semanal**. Si ese techo se agota un sábado, el SIP bloquea a la institución de forma automática, y la rehabilitación es manual y la ejecuta el administrador del SIP.

Es decir: un sistema declarado 24/7 cuyo mecanismo de recuperación funciona en horario de oficina.

Este RFC propone **cuentas de liquidación prefondeadas que operen 24/7**, siguiendo el modelo que usaron Brasil, Estados Unidos y el Reino Unido para exactamente este problema.

## Motivación

### La contradicción, con citas

El catálogo afirma la operación continua en tres lugares distintos:

- **§2.1, funcionalidad 1**: "Opera en tiempo real las 24 horas al día, los 7 días a la semana, todos los días del año; procesando las órdenes de pago de los clientes."
- **Definiciones, *Pago Instantáneo***: "operando en tiempo real y disponible 24/7".
- **Definiciones, *Pago en tiempo real***: "Este proceso se realiza de forma continua, operando las 24 horas del día, los 365 días del año".

Y a la vez establece un horario de operaciones:

- **§5, regla 8**: el administrador del SIP parametriza "Fecha de inicio, Fecha de fin, Horario inicio, Horario fin; de operaciones, montos mínimos, montos máximos".
- **§5, regla 9**: "El monto máximo para pagos realizados en horario no laborable se parametrizará por franjas de horario."
- **§9, regla 4**: "Las OP exitosas e irrevocables ordenadas fuera de horario de operaciones, se liquidarán el primer día laborable siguiente."
- **§9, regla 7**: "La liquidación se efectúa en las cuentas de los IR en el BCE."
- **Tabla 11, estado `PDNG`**: "Diferido (Próximo Día): El pago fue recibido después del *Cut-off*. Se moverá a la malla del siguiente día hábil."

Las dos cosas son compatibles si se leen con cuidado: **los pagos son 24/7, la liquidación no**. El cliente recibe su dinero un domingo a las tres de la mañana; lo que se difiere es la transferencia entre instituciones. Así funciona la mayoría de los sistemas del mundo y no tiene nada de malo por sí solo.

El problema aparece al combinarlo con el motor de riesgo.

### La cadena de consecuencias

1. Un pago se vuelve **irrevocable** cuando el SIP lo entrega a la red beneficiaria (§11.3), independientemente del horario.
2. El valor comprometido de un integrante es "la sumatoria de las OP ordenadas" **pendientes de liquidar** (§10, regla 7).
3. Fuera de horario no liquida ninguna malla (§9, regla 4), así que el valor comprometido **solo sube**.
4. El nivel de riesgo aceptable se calcula una vez por semana, de jueves a miércoles (§10, Tabla 20, nota).
5. Al 100% del techo, el SIP **bloquea automáticamente** los pagos salientes (§10, regla 3).
6. La rehabilitación "la realizará el Administrador del Sistema; posterior al análisis y autorizaciones que correspondan y será de manera manual" (§10, regla 6).

De donde: la capacidad total de una institución para enviar pagos desde el cierre del viernes hasta la apertura del lunes es un número fijo, definido el jueves anterior. Cuando se agota, la institución queda a oscuras hasta que una persona en el BCE la habilite.

Conviene dimensionar el techo con el propio ejemplo del catálogo (Tabla 20): una entidad con captaciones de USD 11.141,71 tiene un encaje requerido de USD 557,09 y un nivel de riesgo aceptable de **USD 139,27**, frente a un saldo en cuenta corriente de USD 576,72. El techo es **cerca de la cuarta parte de lo que la institución efectivamente tiene depositado en el BCE**. No es un límite de fondos disponibles; es bastante más estrecho que eso.

### A quién golpea primero

A las instituciones más pequeñas. Una cooperativa del segmento 2 o 3 tiene factor de riesgo del 100%, que suena generoso, pero se aplica sobre una base de encaje chica (2,5% y 1% respectivamente). El techo absoluto resultante es pequeño, y un fin de semana largo de consumo minorista —Carnaval, Semana Santa, feriados de noviembre, fin de año— es precisamente cuando sus socios más usan el sistema.

El feriado es el caso extremo: entre un viernes por la tarde y un martes por la mañana pueden pasar cuatro o cinco días sin una sola liquidación.

### Qué pasa si no hacemos nada

El primer feriado largo con volumen real produce una institución bloqueada. Sus clientes no van a leer que el sistema funcionó según lo diseñado: van a ver que su banco no sirve. Y va a ocurrir en el momento más frágil de la adopción, cuando la confianza en el riel apenas se está formando.

La regla 9 de §5 —topes de monto por franja horaria fuera de horario laborable— sugiere que quienes diseñaron el esquema tenían presente este riesgo. Pero un tope por pago limita el tamaño de cada operación, no la **exposición agregada**, que es justamente lo que restringe el techo de riesgo. Son dos controles distintos y solo uno de los dos está atendiendo el problema.

## Anclaje normativo

Nada en la norma exige que la liquidación sea en horario de oficina. §9 regla 2 dice que la liquidación de las mallas "se procesa de manera automática, parametrizable el horario", y §9 regla 3 permite "durante el día efectuar varias liquidaciones". La restricción parece ser **operativa** —la disponibilidad de la infraestructura de cuentas del BCE y del sistema SLA— y no legal.

Esa distinción importa, porque una restricción operativa se resuelve con diseño y no con una reforma normativa. Confirmarla es la primera pregunta de este RFC.

## Antecedentes internacionales

Todos los sistemas de pagos instantáneos que operan de verdad 24/7 resolvieron esto de la misma manera: **prefondeo en el banco central**.

**Brasil — Pix.** El Banco Central do Brasil creó las *Contas PI* (Contas de Pagamentos Instantâneos), cuentas de liquidación dedicadas y prefondeadas que operan 24/7/365 con liquidación bruta en tiempo real. Como están prefondeadas, el banco central no toma ninguna decisión de crédito y por lo tanto no necesita personal en línea: la liquidación es una operación mecánica sobre saldo disponible. Los participantes recargan durante el horario del STR y el saldo queda utilizable todo el fin de semana. La exposición crediticia entre instituciones es **cero, siempre**.

**Estados Unidos — FedNow.** Opera 24/7/365 contra saldos en las cuentas maestras de la Reserva Federal, con un mecanismo de transferencias de liquidez para rebalancear fuera de horario.

**Reino Unido — Faster Payments.** Migró a un modelo de prefondeo con topes de envío neto garantizados en el Banco de Inglaterra, precisamente después de las observaciones sobre riesgo de liquidación en esquemas de neto diferido.

**India — UPI.** Opera 24/7 con compensación neta diferida, pero corre múltiples ciclos de liquidación incluidos fines de semana y feriados, y el RBI llevó su RTGS a operación 24/7 en diciembre de 2020 para sostenerlo.

El hilo común es doble: la exposición del neto diferido en ventanas largas es exactamente lo que el CPMI y las evaluaciones FSAP del FMI señalan como riesgo, y la solución estándar es prefondear.

## Diseño propuesto

### Opción A — Cuenta de liquidación prefondeada 24/7 *(recomendada)*

Cada integrante mantiene en el BCE un **saldo segregado destinado a la liquidación del SIP**, adicional a su cuenta corriente operativa.

- **Dentro del horario de operaciones**, todo funciona como está especificado hoy: el techo se deriva del encaje y las mallas liquidan varias veces al día.
- **Fuera del horario de operaciones**, el techo del integrante pasa a ser **su saldo prefondeado**. La ventana queda entonces completamente colateralizada: no hay riesgo de crédito, no hay decisión que tomar y, por lo tanto, no hace falta que haya nadie del BCE conectado.
- La liquidación fuera de horario puede ejecutarse **de forma continua o en ciclos cortos** contra ese saldo, lo que además recompone el cupo continuamente y elimina el modo de falla descrito arriba.
- La recarga se hace durante el horario de operaciones, con la opción de un barrido automático al cierre.

Lo relevante para la discusión es que esto **no cambia la mecánica de compensación ni la mensajería**. Cambia de dónde sale el cupo y con qué frecuencia se repone.

### Opción B — Ciclos automáticos fuera de horario *(intermedia)*

Mantener el modelo actual, pero ejecutar ciclos de liquidación automáticos también en fines de semana y feriados contra un límite preautorizado, y **rehabilitar automáticamente** al integrante cuando un ciclo le devuelva cupo.

Es mucho más barata que la opción A porque no requiere infraestructura de cuentas nueva, y resuelve lo más grave —sacar a la persona del circuito de recuperación—. No elimina la exposición crediticia, solo la acorta.

### Opción C — Piso mínimo de seguridad

Si ninguna de las dos anteriores es viable en el corto plazo, entonces al menos:

1. Los techos fuera de horario deben dimensionarse para **toda la ventana de cierre**, no para un día: el techo de un viernes previo a feriado no puede ser el mismo que el de un martes.
2. La semaforización debe proyectar **tiempo restante hasta el agotamiento** al ritmo de consumo observado, no solo el porcentaje actual. Un integrante al 60% un sábado a las diez de la mañana está en una situación muy distinta a uno al 60% un miércoles.
3. La rehabilitación debe tener una **guardia documentada con tiempo de respuesta comprometido** fuera de horario. Un control automático cuyo único desbloqueo es humano necesita que ese humano exista los domingos.

### Ruta de adopción

La opción A admite implementación gradual: primero como facilidad **voluntaria** para las instituciones que quieran mayor capacidad de fin de semana, y obligatoria después, una vez medido el comportamiento real. Eso permite empezar sin obligar a nadie a inmovilizar fondos antes de que exista evidencia de que le sirve.

## Consideraciones de seguridad, fraude y privacidad

El prefondeo reduce el riesgo de crédito pero concentra riesgo operativo: el saldo prefondeado debe ser visible en tiempo real para su titular y estar protegido contra error operativo, porque un saldo mal calculado un domingo deja a la institución sin operación sin que haya nadie para corregirlo.

Una ventana de 24/7 con menos supervisión humana es atractiva para el fraude, y el patrón de fraude de fin de semana está documentado en otros esquemas. Si se amplía la operación continua, debería venir acompañada de monitoreo de anomalías fuera de horario, y no solamente de topes de monto.

Esto vuelve más urgente algo que ya teníamos anotado como vacío: **el catálogo no contempla ningún mecanismo de devolución ni de disputa**. Bajo operación continua, un fraude ejecutado un viernes por la noche no tiene ningún camino de reversión hasta el lunes, y para entonces el dinero se movió.

## Desventajas y alternativas

**El costo de oportunidad del prefondeo es la objeción legítima.** Dinero inmovilizado en el BCE no rinde, y para una cooperativa pequeña eso pesa. La mitigación razonable es que el saldo prefondeado **compute para el requerimiento de encaje** en lugar de sumarse a él, de modo que no represente inmovilización adicional. Si eso es admisible bajo el régimen de encaje vigente es una pregunta abierta y probablemente la más importante de este RFC.

**Alternativa: una facilidad de crédito intradía del BCE.** Rechazada. Es el mismo riesgo de crédito con otro nombre, y requiere que el BCE esté disponible para otorgarla, que es exactamente lo que estamos tratando de evitar.

**Alternativa: aceptar la liquidación en horario de oficina.** Es el statu quo. Es defendible mientras el volumen sea bajo, pero contradice la definición que el propio catálogo da de pago instantáneo, y el problema crece justamente con el éxito del sistema.

## Preguntas abiertas

1. **¿Puede la infraestructura de cuentas del BCE registrar movimientos 24/7, o el límite es del sistema y no de la política?** Esto determina cuál de las tres opciones es viable y es la pregunta que hay que hacer primero.
2. **¿Cuáles son los horarios de operación efectivamente parametrizados?** El catálogo dice "parametrizable" y no publica los valores. Sin ellos no se puede dimensionar la ventana.
3. **¿Admite el régimen de encaje que un saldo segregado de liquidación compute para el requerimiento?**
4. **¿Cuál es el volumen real fuera de horario?** Es estimable con datos públicos y determina si esto es un problema de hoy o de dentro de dos años.
5. **¿Cómo interactúa esto con la prioridad de liquidación del SIP** sobre SPI, SCI, CCE y CCC (§9, regla 6) en días no laborables, cuando esas cámaras no operan?

## Nota sobre nuestra implementación

En la arquitectura de este proyecto el techo de riesgo está modelado como una **cuenta de control con saldo** ([ADR-0009](../architecture/adr/0009-risk-ceiling-as-funded-account.md)), no como una regla imperativa. La consecuencia práctica es que las tres opciones de este RFC son **el mismo código con distinta fuente de fondeo**: cambia qué acredita la cuenta de cupo y con qué frecuencia, no cómo se registran los pagos.

Eso nos permite simular las tres y publicar los números de exposición comparados, sobre volúmenes reconstruidos con datos públicos. La intención de este RFC no es tener la razón en una discusión de diseño, sino traer evidencia a una mesa donde hoy hay sobre todo supuestos.
