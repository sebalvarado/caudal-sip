# El Catálogo Operacional, explicado

**Fuente**: *Catálogo Operacional para transferencias electrónicas de pago en tiempo real — Sistema Integrador de Pagos*, v1.0, marzo de 2026, Gerencia de Medios de Pago y Servicios Nacionales del BCE. El [PDF oficial local](../../research/bce/Catalogo-Operacional.pdf) y su [transcripción mecánica](../../research/bce/Catalogo-Operacional.md) están versionados junto con su URL, fecha de descarga y SHA-256 en [`research/bce/README.md`](../../research/bce/README.md).

> **English abstract.** This is the reference document for Ecuador's real-time payment scheme: the operational specification behind resolutions BCE-GG-007-2026 and BCE-GG-008-2025. It defines the payment state machine, rejection codes, llave (payment key) types and lifecycles, timing limits, ISO 20022 message versions and mandatory fields, deferred net settlement, and a liquidity-risk engine that automatically blocks an institution's outgoing payments when its committed obligations exceed a reserve-derived ceiling. Written in Spanish because its readers are Ecuadorian institutions working against a Spanish-language specification; section numbers refer to the catálogo itself so every claim can be verified. The companion *catálogo técnico* (field-level schemas) has not been published.

Este es el documento que da sustento operativo a las resoluciones BCE-GG-007-2026 y BCE-GG-008-2025, y el que dábamos por inaccesible cuando escribimos el resto de la documentación. Resuelve casi todo lo que teníamos marcado como **[assumption]**. Los números de sección corresponden al catálogo original, para que cualquier afirmación se pueda verificar en la fuente.

> **Estado de verificación (2026-08-11).** La primera versión de este resumen se hizo con texto plano extraído del PDF, y varias tablas se desordenaron al extraerlas: los valores quedaron corridos una fila. **Las tablas numéricas se volvieron a extraer con reconocimiento de estructura y se corrigieron** — en particular la Tabla 19 (riesgo de liquidez), el Anexo 1 (códigos de llave) y la Tabla 12 (responsables de los rechazos), que tenían errores materiales. El *catálogo técnico* —esquemas y validaciones campo por campo— sigue sin publicarse, así que todo lo que esté por debajo del nivel de "nombre de mensaje" continúa sin confirmar.

## 1. Lo que resuelve el catálogo

| Pregunta que teníamos abierta | Respuesta |
|---|---|
| Estados de una orden de pago | Seis estados, con códigos ISO (§5) |
| Códigos de rechazo | Lista completa, con el responsable de cada uno (§5, Tabla 12) |
| Tipos, límites y estados de las llaves | Cuatro tipos, cuatro estados, códigos por operación (§3) |
| Tiempos | 60 s para confirmar, 20 s de extremo a extremo (§5) |
| Versiones de mensajes | `head.001.001.04`, `pacs.008.001.08`, `pacs.002.001.10` y los `acmt.998.*` propios del BCE (§8, §11) |
| Formato del identificador del pago | BIC + fecha y hora + secuencial o hash, de 20 a 35 caracteres (§7) |
| Modelo de liquidación | Compensación neta diferida, malla multilateral, varias liquidaciones al día (§9) |
| Qué pasa cuando algo falla | Cuatro excepciones, cada una con su efecto sobre irrevocabilidad y malla (§11.3–§11.4) |
| Comisiones | Parametrizables, se le muestran al ordenante, `ChrgBr` por defecto `DEBT` (§5, §8, §9) |
| Riesgo de liquidez | Techo semanal calculado sobre el encaje, semaforización y bloqueo automático de pagos (§10) |
| Portabilidad de llaves | No existe: hay que anular y volver a registrar (§11.1) |

## 2. La topología, confirmada

Todo mensaje viaja **IR → RP → SIP → RP → IR**. Los integrantes nunca hablan directamente con el SIP, y el SIP nunca habla con un integrante: la red de pagos es intermediaria obligatoria en ambos sentidos. Esto confirma el modelo de cuatro perfiles de operación que planteamos en [`../architecture/blueprint.md`](../architecture/blueprint.md).

Hay un componente que no habíamos modelado: el **Sistema Central de Llaves**, un registro central administrado por el BCE que debe mantenerse **sincronizado en tiempo real** con el Directorio Distribuido de cada red (§3). El reparto es que el sistema central guarda lo necesario para garantizar unicidad y encaminar la resolución, mientras que el directorio distribuido guarda las credenciales del cliente. La conciliación entre ambos es un proceso definido y reportado (§12), no un detalle menor: una llave cuyas dos versiones no coinciden queda en estado `Por conciliar`.

Un integrante puede tomar el servicio con llaves de una red y el servicio con credenciales de otra, pero no el mismo servicio en dos redes (§6). Por eso existe un empadronamiento obligatorio: el SIP necesita saber a qué red encaminar los pagos que no usan llave.

## 3. Llaves

### Tipos (§3, Tabla 1)

| Código | Tipo | Formato |
|---|---|---|
| `0001` | Número celular | Alfanumérico, 10 caracteres |
| `0002` | Número de identificación | Numérico, de 10 (cédula) a 13 (RUC). **El pasaporte queda excluido** |
| `0003` | Correo electrónico | Alfanumérico, hasta 250; debe tener `@` y un dominio válido |
| `0004` | Alfanumérico | Hasta 20 caracteres |

Las llaves de tipo celular se vuelven únicas a nivel nacional anteponiéndoles un código que el BCE asigna a cada integrante, lo que las convierte en llaves alfanuméricas (§3, nota 2, y Anexo 1). El Anexo 1 trae esos códigos para las once entidades con activos superiores a USD 1.500 millones: Pichincha `10`, Pacífico `30`, Guayaquil `17`, Produbanco `36`, Bolivariano `37`, Internacional `32`, Austro `35`, Diners `126`, JEP `107`, Jardín Azuayo `100` y Alianza del Valle `124`. Tanto el código de la red como el BIC del integrante los asigna el BCE, con hasta 11 caracteres alfanuméricos.

**Corrección a nuestro glosario**: el pasaporte puede aparecer en una *credencial de pago*, pero no sirve como *llave*. Lo teníamos mal.

### Estados y operaciones

Los estados son `Activa` `0001`, `Anulada` `0002`, `Por conciliar` `0003` y `Suspendida` `0004`. Las operaciones son anulación `0001`, suspensión `0002` y habilitación `0003`, y cada una tiene su propia tabla de códigos de resultado donde el `0005` siempre es "Rechazada: Error por Time out" (§3, Tablas 2 a 9).

### Flujos y sus mensajes (§11.1)

| Operación | Mensajes |
|---|---|
| Registro | `acmt.998.001.bce01.in` → `acmt.998.002.bce01.out` |
| Anulación, suspensión y habilitación | `acmt.998.003.bce01.in` → `acmt.998.004.bce01.out` |
| Resolución (inter-redes) | `acmt.998.005.bce01.in` → `acmt.998.006.bce01.out` |

El registro es una **reserva en dos fases**: la red reserva la llave, el SIP verifica si ya existe y devuelve "Llave ya existe" o bien la reserva y la registra centralmente, y recién ahí la red y por último el integrante la confirman. Si la comunicación falla en cualquier punto, la creación se rechaza y **la llave no queda reservada**. Es un detalle decisivo para la implementación: un reintento ingenuo después de un timeout no puede dejar reservas huérfanas.

La resolución devuelve seis datos del integrante beneficiario: número de identificación, nombre, número de cuenta, **estado de la cuenta**, tipo de cuenta y BIC. Los pagos iniciados con QR resuelven por este mismo flujo. La suspensión es responsabilidad del integrante y bloquea de inmediato cualquier operación con esa llave.

**La portabilidad es un vacío.** El único camino documentado es anular y volver a registrar (§11.1, nota), sin ventana de reclamo ni confirmación por parte del integrante que pierde la llave. Existe una migración masiva entre redes (§4), pero es un proceso administrativo que ejecuta el BCE, debe hacerse fuera del horario de operación y deja las llaves del integrante congeladas en "EN PROCESO DE MIGRACIÓN" mientras dura. Esto convierte nuestra propuesta de un flujo de reclamo al estilo Pix en un vacío documentado y no en una especulación, y es el mejor candidato a RFC que tenemos.

## 4. Pagos

### Tiempos (§5)

Una vez resuelta la llave, el cliente tiene **60 segundos** para confirmar; pasado ese tiempo hay que reiniciar todo desde la resolución. Aparte, el tiempo entre el inicio y la confirmación del pago no puede superar los **20 segundos**, y si los supera el pago se rechaza con `DT01` / `0205 PGO RECHAZADO, POR NO CONFIRMACIÓN TIME OUT`. Cada etapa tiene además su propio timeout parametrizable, y los reintentos solo caben dentro de la ventana de su etapa.

Los reintentos nunca revierten un rechazo por timeout ni una irrevocabilidad ya declarada; sirven únicamente para resolver la condición de "por conciliar" (§2.2, regla 4).

### Estados (§5, Tabla 11)

| Código | Significado |
|---|---|
| `RCVD` | Aceptación técnica: el nodo destino recibió el mensaje y pasó las validaciones básicas de red y protocolo. No dice nada sobre su validez funcional |
| `ACTC` | Orden validada: pasó las validaciones de esquema y firma, y está en cola |
| `ACSP` | **Irrevocable** y dentro de la malla de compensación del día |
| `ACSC` | **Compensado**: la malla se cerró y los fondos se transfirieron legalmente |
| `RJCT` | Rechazado: el pago no pudo procesarse. El ejemplo que da el catálogo es "saldo insuficiente en malla" |
| `PDNG` | "Pago por conciliar" según el nombre; **"Diferido (Próximo Día): el pago fue recibido después del *cut-off*. Se moverá a la malla del siguiente día hábil"** según la explicación |

**El estado `PDNG` es ambiguo en la fuente.** Su nombre dice conciliación y su explicación dice diferimiento por cut-off; son dos cosas distintas. Si `PDNG` significa "llegó tarde, va a la malla de mañana", entonces la condición *por conciliar* de la excepción 3 —irrevocable con resultado desconocido— **no tiene estado propio en la tabla**, y es la situación operativamente más delicada de todo el esquema. Es una pregunta concreta y bien fundada para el BCE, y de las mejores candidatas a primer contacto técnico.

La diferencia entre `ACSP` y `ACSC` es exactamente la distinción entre firmeza para el cliente y firmeza entre instituciones que [`../architecture/data-model.md`](../architecture/data-model.md) insistía en modelar con dos marcas de tiempo separadas. El catálogo confirma el diseño y, de paso, nos da el vocabulario oficial.

La irrevocabilidad se produce "una vez que el SIP haya notificado y la RP beneficiaria haya recibido dicha OP" (§11.3): al **entregarse a la red beneficiaria**, no cuando se confirma la acreditación al cliente. Es antes de lo que casi cualquier implementador supondría, y es la razón de que la excepción 3 se comporte como se comporta.

### Manejo de fondos

El integrante ordenante **debita y reserva** los fondos apenas acepta la instrucción, y recién los **libera** con la confirmación final (§11.3, pasos 2 y 12). Si el pago se rechaza, se los devuelve al cliente. Es decir que el lado ordenante mantiene una reserva durante todo el flujo, y de ahí sale el techo de 20 segundos.

### Códigos de rechazo (§5, Tabla 12)

Responsabilidad del **integrante beneficiario**, todos con su código interno `02xx`: `AC04` cuenta cerrada (`0201`), `AC01` cuenta no existe (`0202`), `AC06` cuenta bloqueada (`0203`), `AC05` orden beneficiario (`0204`) y `AC07` cuenta inactiva (`0206`).

Responsabilidad de **todos**: `DT01` por no confirmación o timeout (`0205`).

Responsabilidad del **SIP**: `AM04` insuficiencia de fondos (`0207`), `MS01` información incorrecta (`0208`), `AM11` moneda no válida, `CH16` contenido incorrecto, `CH21` elemento obligatorio ausente, `DT02` fecha no válida, `DS0K` número de transacciones no permitido, `CH20` formato de monto incorrecto, `AM01` monto cero, `DU01` identificación del mensaje no única, `AM02` monto mayor al permitido, `AC02` número de cuenta inválido, `TA01` error de comunicación, `FF10` problemas técnicos, `FF02` sintaxis incorrecta.

**Corrección importante**: `AM04` (insuficiencia de fondos) y `MS01` son responsabilidad del **SIP**, no del integrante beneficiario, como decía la primera versión de este resumen. Tiene todo el sentido y confirma el modelo: la insuficiencia que el SIP detecta no es la del cliente sino la de la **malla** —el integrante ordenante agotó su nivel de riesgo—, tal como sugiere el propio ejemplo del estado `RJCT`. En nuestra implementación, `AM04` es el código que corresponde a un rechazo por falta de cupo de liquidez.

Son códigos externos estándar de ISO 20022, así que nuestra librería puede apoyarse en los conjuntos publicados en lugar de inventarse una enumeración propia.

### Iniciación e identificadores

Los canales se codifican `0001` Web y `0002` APP, y la forma de iniciación se marca como `Key`, `Cred` o `QR` (§5, Tablas 10 y 13). El identificador único del pago lo genera el integrante ordenante con la forma **BIC + `DDMMAAAAHHMMSS` + secuencial o hash criptográfico**, de 20 a 35 caracteres; por ejemplo `PACIECEG` `27012026094735` `XYZ12345`, generado el 27 de enero de 2026 a las 09:47:35 (§7, Tabla 14). La tabla no explicita el formato de la fecha; se deduce de ese ejemplo. No es editable, es obligatorio, aparece en el comprobante del cliente y es la referencia para toda la trazabilidad y conciliación.

La aplicación del integrante ordenante debe mostrar el nombre del beneficiario y el valor de la comisión con impuestos antes de que el cliente confirme, se haya iniciado el pago con llave o con credencial (§2.2, regla 6; §5, reglas 1 y 2).

## 5. Perfil de mensajería (§8)

El catálogo especifica los campos obligatorios de `head.001.001.04` (cabecera de aplicación), `pacs.008.001.08` (orden de pago) y `pacs.002.001.10` (confirmación y rechazo). Ojo con una inconsistencia de la fuente: en la sección de flujos (§11.3) la orden de pago aparece como `pacs.008.004.08 BCE in/out`, mientras que en las tablas de campos (§8) figura como `pacs.008.001.08`. Vale la pena consultarlo con el BCE.

Además de los elementos obligatorios por ISO, el BCE marca como obligatorios: `CdtTrfTxInf/PmtId/InstrId` (el código único del pago), los BIC de las **redes** ordenante y beneficiaria en `InstgAgt` e `InstdAgt`, la identidad completa de ordenante y beneficiario —nombre, número y tipo de identificación, número de cuenta—, y `InstrForCdtrAgt/InstrInf` (canal), `InstrForNxtAgt/InstrInf` (forma de iniciación) y `Purp/Prtry` (motivo del pago). Las extensiones opcionales del BCE van en `SplmtryData/Envlp`: `QrCode` y `TimeStamp`.

`ChrgBr` viene por defecto en `DEBT`, o sea que el ordenante asume el costo. El indicador de reintento va en `AppHdr/PssblDplct`, que es el campo que tiene que usar cualquier implementación seria de idempotencia.

Dos observaciones para nuestra librería. La primera es que el perfil lleva **la identidad completa del cliente en cada mensaje de pago**, lo que abre una pregunta real de minimización de datos que deberíamos plantear. La segunda es que el encaminamiento depende de BIC de red distintos de los BIC de integrante, así que hay que manejar ambos.

## 6. Qué pasa cuando algo falla (§11.3–§11.4)

Las cuatro excepciones con nombre, y la tabla de resultados que es la página más útil de todo el documento:

| # | Falla | Estado ISO | ¿Irrevocable? | Malla |
|---|---|---|---|---|
| 1 | La RP ordenante no alcanza al SIP | `RJCT` tras reintentos | No | No se genera |
| 2 | El SIP no puede entregar a la RP beneficiaria | `RJCT` | No | No se genera |
| 3 | La RP beneficiaria no logra devolver la confirmación al SIP | `ACSP` | **Sí** | **Por conciliar** |
| 4 | El SIP no logra devolver la confirmación final a la RP ordenante | `ACSC` | **Sí** | Generada |

Las peligrosas son la 3 y la 4: **el dinero ya está comprometido de forma irrevocable y el lado ordenante todavía no lo sabe**. En ambos casos el integrante ordenante le informa al cliente "pago en proceso" —justamente para que no reintente— y los fondos siguen reservados hasta que el estado se resuelva. Es exactamente la regla de que un timeout es un estado desconocido y no una falla, que ya teníamos en el modelo de datos, ahora con respaldo normativo.

La conciliación la ejecutan las redes con información del SIP, y todo pago "por conciliar" debe quedar con estado final **el mismo día, hasta las 18:00** (horario parametrizable) (§13.2, nota 1). Esto es un requisito operativo duro para la consola del integrante: la institución necesita una bandeja de trabajo con esos casos, atendida todos los días.

## 7. Compensación y liquidación (§9)

Compensación neta diferida, confirmada. El SIP acumula en tiempo real las obligaciones inter-redes exitosas e irrevocables en **mallas bilaterales y multilaterales**; **la liquidación se ejecuta sobre la malla multilateral** y la bilateral se conserva para reportes. La liquidación se hace en las cuentas corrientes que los integrantes mantienen en el BCE, con horario parametrizable y **varias veces al día**, y el administrador puede disparar una liquidación manual ante una situación de riesgo de liquidez. Los pagos ordenados fuera del horario de operación se liquidan el siguiente día laborable. Entre los sistemas de compensación del BCE (SPI, SCI, CCE, CCC), **la liquidación del SIP tiene prioridad**.

## 8. El motor de riesgo de liquidez (§10)

Esta parte no la teníamos en el radar, y es el diseño más interesante de todo el esquema: es la forma en que el BCE contiene el riesgo de crédito que genera liquidar en neto diferido.

Cada integrante recibe un **nivel de riesgo aceptable** derivado de su requerimiento de encaje. Estos son los valores de la Tabla 19, verificados contra el PDF:

| Código | Sector | Encaje | Nivel de riesgo |
|---|---|---|---|
| `0001` | Financiero privado y público, activos > USD 1.000 M | 5% | 25% |
| `0002` | Financiero privado y público, activos ≤ USD 1.000 M | 4% | 30% |
| `0003` | Cooperativas segmento 1 y cajas centrales | 4% | 40% |
| `0004` | Mutualistas de ahorro y crédito para la vivienda | 4% | 40% |
| `0005` | Cooperativas segmento 2 | 2,5% | 100% |
| `0006` | Cooperativas segmento 3 | 1% | 100% |
| `0007` | Cooperativas segmento 4 | *(sin valores en el catálogo)* | |
| `0008` | Cooperativas segmento 5 | *(sin valores en el catálogo)* | |
| `0009` | SEDPES (fintech) | 0,5% más un porcentaje según su nivel de riesgo | 100% |

Para las SEDPES, la nota 6 de la tabla ubica el porcentaje adicional en el **encaje**, no en el factor de riesgo. El catálogo no publica la fórmula de ese porcentaje.

Que los segmentos 4 y 5 aparezcan con las celdas vacías puede ser un descuido de edición o una decisión pendiente, pero son justamente las cooperativas más pequeñas del país. Vale preguntarlo.

El **saldo mínimo aceptable** es el encaje requerido multiplicado por el factor de riesgo, y el catálogo trae un ejemplo trabajado (Tabla 20): captaciones de USD 11.141,71 dan un encaje del 5% de USD 557,09, que al 25% dan un nivel de riesgo aceptable de **USD 139,27**, frente a un saldo en cuenta corriente en el BCE de USD 576,72. Conviene detenerse en esa proporción: **el techo de valor comprometido es una fracción pequeña del saldo que la institución efectivamente tiene depositado.** No es una regla de fondos disponibles, es un límite prudencial bastante más estrecho.

El nivel se recalcula **semanalmente, de jueves a miércoles**, siguiendo el ciclo de cálculo de encaje del sistema SBA, y el valor rige durante los siete días siguientes.

El control considera **tres cantidades a la vez** (§10, regla 7): el nivel de riesgo aceptable, el saldo de la cuenta corriente —que el SIP toma del sistema SLA cada 30 minutos, parametrizable— y el valor comprometido, que es la suma de las órdenes de pago enviadas pendientes de liquidar. La semaforización es parametrizable: verde bajo el 75%, amarillo entre 75% y 89%, naranja entre 90% y 99%, rojo al 100%. Sobre el ejemplo anterior, los umbrales en dólares son rojo USD 139,27, naranja 125,34, amarillo 104,45 y verde 103,06.

**En rojo, el SIP bloquea los pagos salientes del integrante**, de forma automática, y se lo notifica con un `pacs.002` en respuesta al `pacs.008`, o con un ACK `pacs.002` si el pago se originó con llave. La rehabilitación es manual, la hace el administrador del SIP después del análisis correspondiente. Si el integrante supera el nivel de riesgo **y/o** el saldo de su cuenta corriente, el administrador además bloquea manualmente esa cuenta en el BCE contra débitos.

El tablero de monitoreo debe mostrar seis datos (§10, regla 9): valor y número de órdenes enviadas, posición neta, saldo en cuenta corriente con fecha y hora de la captura, requerimiento de encaje, nivel de riesgo aceptable y saldo mínimo aceptable. La visibilidad está segregada: solo el propio integrante y el administrador del SIP ven su saldo, su encaje y su posición neta (regla 11), con perfiles de Administrador, Operador y Consulta BCE.

De aquí se desprenden dos cosas. En lo operativo, a una institución la puede sacar del riel nacional una regla automática a media mañana, y para volver a entrar hace falta que una persona en el BCE lo autorice; por eso la consola del integrante tiene que avisar de la cercanía al umbral mucho antes del rojo. En lo analítico, este diseño se puede modelar: ¿qué le pasa a una cooperativa del segmento 3, con un techo estrecho y una recomposición de cupo que solo llega cuando liquida la malla, un día de pago de sueldos? Esa simulación es la contribución técnica de mayor valor que podríamos llevar a los foros de interoperabilidad, y se puede construir con datos públicos.

## 9. Comisiones

Las comisiones se parametrizan en el SIP, las calcula el SIP, se le muestran al ordenante con impuestos antes de confirmar, y por defecto las asume quien paga (§2.1, ítem 18; §5, reglas 2 y 12; §8; §9, regla 5).

Esto cambia bastante nuestra postura pública. La gratuidad de los pagos entre personas fue determinante para la adopción de Pix, y el diseño ecuatoriano asume en cambio transacciones con costo desde el primer día. Pedir gratuidad ya no es hablar sobre un vacío sino argumentar contra un valor por defecto que está especificado; hay que hacerlo con evidencia y no con consignas.

## 10. Quiénes ya están adentro

El Anexo 2 nombra a las redes de pagos con sus nuevos BIC: **BANRED** `BANDECEQ`, **COONECTA** `COONECEQ`, **ETIKO's** `ETIKECEC` y **DEUNA** (Banco Pichincha) `DEUNECEQ`. En los ejemplos de reportes aparecen además `RPI (RPIEQ)` y `CHAS (CHASCUE)`, con códigos escritos de otra forma que en el anexo; conviene tomar el anexo como referencia y los ejemplos como ilustración.

Los once **integrantes obligados** (activos superiores a USD 1.500 millones) con sus nuevos BIC: Pichincha `PICHECEQ500`, Pacífico `PACIECEG`, Guayaquil `GUAYECEGNOS`, Produbanco `PRODECEQ`, Bolivariano `BBOLECEG`, Internacional `BINTECEQ`, Austro `AUSTECEQ`, Diners Club `DICDECEQ`, COOP JEP `CJEPECEQ`, Jardín Azuayo `CAJAECEC` y Alianza del Valle `CADVECEQ`. Las dos cooperativas y varios bancos cambian de BIC: Jardín Azuayo pasa de `CCJAECE1` a `CAJAECEC` y Alianza del Valle de `CAEZECE1` a `CADVECEQ`.

Dos cooperativas están en la primera ola de obligados. Sirve para el acercamiento: el sector cooperativo no está mirando este plazo desde afuera.

## 11. Qué cambia en nuestra propia documentación

| Documento | Cambio |
|---|---|
| [`../architecture/data-model.md`](../architecture/data-model.md) | Reemplazar los estados supuestos por `RCVD`/`ACTC`/`ACSP`/`ACSC`/`RJCT`/`PDNG`; adoptar los códigos de rechazo reales; incorporar la reserva y liberación de fondos; agregar el cierre de conciliación de las 18:00; incorporar el Sistema Central de Llaves y sus estados de conciliación |
| [`../architecture/technical-requirements.md`](../architecture/technical-requirements.md) | Cambiar el presupuesto inventado de 5 segundos por el techo de 20 y la ventana de 60, y volver a derivar los presupuestos por tramo |
| [`../architecture/scheme-design.md`](../architecture/scheme-design.md) | Liquidación: confirmar el neto diferido y sumar el motor de riesgo de liquidez; reescribir la postura sobre comisiones; reclasificar la portabilidad de llaves de propuesta a vacío documentado |
| [`../glossary.md`](../glossary.md) | El pasaporte no sirve como llave; agregar Sistema Central de Llaves, malla, nivel de riesgo aceptable y por conciliar |
| [`../plan/action-plan.md`](../plan/action-plan.md) | El primer punto de la Fase 0 queda a medias: tenemos el catálogo operacional, falta el técnico |
| [`../product/requirements.md`](../product/requirements.md) | La consola del integrante necesita bandeja diaria de conciliación con cierre a las 18:00 e indicador de cercanía al umbral de riesgo |

Todos estos cambios ya se aplicaron.

## 12. Lo que el catálogo deja abierto

Nuestra cartera de RFC, ahora apoyada en un documento real y no en suposiciones:

1. **Portabilidad de llaves** con ventana de reclamo y confirmación del integrante que la pierde, en lugar de anular y volver a registrar.
2. **Devoluciones y disputas**: no aparece ningún equivalente a `pacs.004` ni un mecanismo de devolución por fraude.
3. **Los valores concretos de cada timeout**; el catálogo dice "parametrizable" y no publica los parámetros.
4. **Minimización de datos**: la identidad completa del cliente viaja en cada `pacs.008`. ¿Cabe un perfil con datos enmascarados o tokenizados?
5. **La inconsistencia de versión** entre `pacs.008.001.08` y `pacs.008.004.08`.
6. **Política de comisiones**, y si la gratuidad entre personas es viable dentro de este diseño.
7. **La dinámica del bloqueo por liquidez**: qué debe hacer una institución cuando se acerca al rojo, y si un recálculo semanal alcanza para absorber un pico de un día de pagos.
8. **Solicitud de pago y pagos masivos**, que siguen sin aparecer.
