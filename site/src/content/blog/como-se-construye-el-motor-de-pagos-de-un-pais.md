---
title: "Cómo se construye el motor de pagos de un país"
description: "El sistema central que mueve los pagos instantáneos de un país no es una obra faraónica: son seis servicios, un libro mayor bien hecho y bastante menos tráfico del que la gente imagina. Esta es la arquitectura que estamos construyendo, en abierto, y por qué creemos que funciona."
pubDate: 2026-08-11
lang: es
---

Cada vez que se habla de pagos instantáneos en Ecuador aparece la misma frase: *es un proyecto enorme*. Y con la frase viene el presupuesto, y con el presupuesto vienen los años.

Vale la pena poner un número sobre la mesa antes de seguir. Bre-B, el sistema colombiano, atiende a un país tres veces más grande que el nuestro y procesa alrededor de cinco millones de transacciones al día. Eso son unas **sesenta transacciones por segundo** en promedio. Incluso si asumimos picos diez veces por encima del promedio —un viernes de quincena, un feriado— y aunque Ecuador se comportara como un país mucho más grande de lo que es, seguimos hablando de cifras que un servidor común procesa sin despeinarse. Una sola base de datos PostgreSQL bien configurada maneja ese tráfico con margen de sobra.

El sistema de pagos de un país no es un problema de volumen. Es un problema de **corrección**: que ningún dólar aparezca ni desaparezca, que dos instituciones nunca discrepen sobre quién le debe qué a quién, y que cuando algo falle a mitad de camino —porque va a fallar— el sistema sepa exactamente en qué estado quedó cada pago.

Eso no se resuelve con presupuesto. Se resuelve con diseño.

## Qué estamos construyendo

Dos cosas, y conviene separarlas bien porque suelen confundirse.

La primera es el **sistema central**: el motor que el Banco Central del Ecuador necesita para operar el Sistema Integrador de Pagos. Es el equivalente de lo que opera el Banco Central de Brasil para Pix o Banco de México para SPEI. Resuelve llaves, enruta pagos entre redes, acumula las obligaciones del día y las liquida. Esta parte la construimos como implementación de referencia: código que funciona, que se puede correr, auditar y medir.

La segunda es la **capa del integrante**: lo que un banco o una cooperativa necesita para conectarse al riel. Traduce entre el lenguaje interno de la institución y la mensajería ISO 20022 que exige la norma, maneja llaves, firma, códigos QR. Esta parte sí es software para producción, y es donde está la urgencia: unas cuatrocientas instituciones tienen que estar conectadas y la mayoría no tiene equipo de pagos para hacerlo.

Un solo código base sirve a las dos, con distintas configuraciones. Esa es la decisión de arquitectura que hace que el proyecto sea manejable por un equipo pequeño en vez de por un ejército.

## Las piezas

Lo que hace que esto parezca inabordable es que se lo trata como una sola cosa gigante. No lo es. El catálogo del BCE describe unas veinte funciones del SIP, y esas veinte funciones se acomodan en seis servicios con responsabilidades limpias:

**El registro** sabe quién está en el sistema: qué redes existen, qué integrantes pertenecen a cada una, qué BIC tiene cada quien, quién está habilitado y quién no.

**El directorio de llaves** guarda el vínculo entre una llave —tu celular, tu cédula, un alias— y una cuenta. Su trabajo difícil no es guardar sino garantizar que la misma llave no exista dos veces en el país, y hacerlo mientras varias redes intentan registrar llaves al mismo tiempo.

**La resolución** responde la pregunta "¿de quién es esta llave y a qué cuenta apunta?". Es lo que ocurre en los primeros milisegundos, antes de que veas el nombre del beneficiario en la pantalla.

**El enrutador de pagos** recibe la orden, la valida, la manda a la red del beneficiario y devuelve la respuesta. No guarda estado: eso es a propósito, y es lo que permite poner tres copias detrás de un balanceador y olvidarse del asunto.

**El motor de riesgo** vigila cuánto ha comprometido cada institución y la frena cuando se pasa del límite.

**La compensación y liquidación** acumula las obligaciones del día, las cruza entre todos los participantes y produce la instrucción de liquidación que se ejecuta en las cuentas del Banco Central.

Alrededor de esos seis van tres cosas transversales: el plano de mensajería —firma, mTLS, la traducción a ISO 20022—, la bitácora de auditoría y la consola de operación.

Ninguna de esas seis piezas es, por sí sola, un problema difícil de ingeniería. Lo difícil es que sean correctas juntas.

## La pieza que casi todos dejan para el final

Aquí está la decisión de diseño que más nos importa, y la que llevamos discutiendo estas últimas semanas.

Lo intuitivo es empezar por el protocolo. Es la parte visible: los mensajes, las firmas, el XML. Y entonces la contabilidad —el **libro mayor**, el registro de quién le debe qué a quién— queda para después, tratada como un asunto de registro que se resuelve al final.

Es exactamente al revés. La contabilidad es el corazón, y todo lo demás es cómo se le habla.

Lo decidimos por una razón concreta que salió del propio catálogo del BCE. El diseño ecuatoriano no liquida cada pago en el momento: acumula las obligaciones del día en una malla y las liquida varias veces al día. Eso es eficiente, pero crea riesgo, porque entre que un pago se vuelve irrevocable y que efectivamente se liquida, alguien está debiendo dinero que todavía no ha pagado. El BCE contiene ese riesgo asignándole a cada institución un techo de cuánto puede comprometer, y **bloqueando automáticamente sus pagos salientes cuando lo alcanza**.

Ese techo se puede programar de dos maneras. Una es escribir la regla en el código: calcular, comparar, decidir. Funciona hasta que alguien agrega un camino nuevo y se olvida de llamar a la validación.

La otra es convertir el techo en un **saldo**: darle a cada institución una cuenta de control que se acredita con su cupo, y hacer que cada pago la debite. Si la cuenta no alcanza, la operación no se puede registrar. No porque el código lo prohíba, sino porque el libro mayor no admite que una cuenta quede en negativo, igual que no admite un asiento descuadrado.

La diferencia entre las dos es la diferencia entre una regla y una ley física. Una regla se puede omitir; una ley no. Y con este diseño, el semáforo del catálogo deja de ser lógica que hay que mantener y pasa a ser una consulta de saldo.

Por eso el orden de construcción empieza por el libro mayor y no por el protocolo. Primero las cuentas, los asientos, las reservas y los invariantes. Después la compensación. Recién entonces la mensajería, escrita sobre un núcleo que ya es correcto, en vez de un protocolo al que se le van parchando errores de contabilidad mientras ya circula dinero.

Tiene además una ventaja práctica: esas primeras etapas **no dependen del catálogo técnico**, que es la parte de la norma que el BCE todavía no publica. Se pueden construir hoy.

## ¿Y de verdad va a ser instantáneo, interoperable y gratis?

Instantáneo, sí, y la norma lo obliga: el catálogo fija veinte segundos como techo entre el inicio y la confirmación de un pago. Nuestro trabajo es que la infraestructura no sea el cuello de botella, y a sesenta transacciones por segundo no lo va a ser.

Interoperable, también, y ahí está el cambio de fondo. Hoy en Ecuador el pago funciona si las dos personas están en el mismo banco. Con llaves y con un directorio nacional, la pregunta "¿tienes cuenta en el Pichincha?" simplemente deja de existir.

Gratis es más complicado, y prefiero decirlo con claridad. El catálogo contempla comisiones parametrizables, calculadas por el SIP y asumidas por defecto por quien paga. O sea que la gratuidad **no está en el diseño**: es una decisión de política, no un asunto técnico.

Lo que sí podemos hacer, y estamos haciendo, es tratar las comisiones como parte formal del sistema desde el primer día —cada comisión y cada impuesto son un asiento contable, no un cálculo suelto— con las tarifas en un módulo aparte que se puede simular. Eso permite responder con números la pregunta que importa: *cuánto costaría exactamente que los pagos entre personas fueran gratis*. La gratuidad de Pix fue determinante para su adopción en Brasil. Argumentarlo aquí con una simulación sobre volumen real es mucho más útil que argumentarlo con una consigna.

## ¿Y los pagos entre empresas y los roles de pago?

Es la pregunta que más nos han hecho, y la respuesta honesta tiene dos partes.

Para lo que una empresa hace todos los días —cobrarle a un cliente, pagarle a un proveedor local— el riel funciona igual que para una persona. A un mensaje de pago no le importa si quien paga es una señora o una ferretería. Eso ya es un cambio grande para el comercio pequeño y mediano.

Para nóminas y pagos empresariales grandes, todavía no. Faltan tres cosas y vale nombrarlas: el límite de USD 15.000 por transferencia deja fuera buena parte de los pagos entre empresas; no existe en el catálogo una figura de **pago masivo**, así que un rol de pago de tres mil empleados son tres mil mensajes sueltos; y la información de referencia que viaja en el mensaje es demasiado pobre para que una empresa concilie automáticamente cuál pago corresponde a cuál factura, que es precisamente la razón por la que una empresa adopta o no adopta un riel.

Hay un cuarto asunto, más sutil y más interesante. Un rol de pago consume el cupo de riesgo de la institución de golpe, en segundos, por el monto completo. Y como el cupo solo se recompone cuando liquida la malla —unas pocas veces al día, no continuamente—, una cooperativa pagando sueldos el día 15 podría agotar su techo y quedar **bloqueada para todos sus pagos salientes**, incluidos los de sus socios que no tienen nada que ver con la nómina. Sus clientes lo vivirían como que el banco se cayó.

¿Pasa eso con los volúmenes reales del Ecuador? No lo sé todavía, y ese es justamente el punto. Es simulable con datos públicos, y correr esa simulación es probablemente lo más útil que este proyecto puede llevar a una mesa de interoperabilidad: convierte una opinión sobre el diseño en un gráfico que se puede discutir.

## Dónde estamos

Con transparencia, porque es lo que hace útil una bitácora: **todavía no hay código publicado.** Lo que hay es la norma leída en detalle, la arquitectura decidida y documentada, y las decisiones difíciles tomadas y escritas con sus alternativas descartadas, para que quien llegue después herede el razonamiento y no solo el resultado. Todo está en el repositorio, incluido lo que aún no sabemos.

Lo que sigue es el libro mayor: cuentas, asientos, reservas, invariantes y una batería de pruebas que intente romperlos. Cuando eso esté, lo publicamos con los números de rendimiento medidos, no estimados.

Si trabajas en una cooperativa, en un banco o en el sector público y algo de esto te suena mal —o te suena bien y quieres ayudar a que salga— el repositorio está abierto y las discusiones también. [Lee el manifiesto](/manifiesto/) o revisa [el estándar explicado](/estandar/), que es la lectura del catálogo del BCE sobre la que está construido todo lo demás.
