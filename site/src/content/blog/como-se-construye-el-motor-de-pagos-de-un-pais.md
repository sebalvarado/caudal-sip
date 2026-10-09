---
title: "El motor que puede mover la plata de todo un país"
description: "Un sistema nacional de pagos no tiene que ser una obra incomprensible ni costar una fortuna. Te contamos, sin esconder la complejidad, cómo se puede construir por partes y por qué el corazón no está en los mensajes sino en una contabilidad que jamás mienta."
pubDate: 2026-08-11
lang: es
---

Imagina que un domingo, a las once de la noche, una enfermera quiere enviarle veinte dólares a su mamá. No sabe en qué banco tiene la cuenta. Tampoco debería importarle.

Escribe su número de celular, ve su nombre, confirma y recibe una respuesta antes de guardar el teléfono. Detrás de ese gesto sencillo, varias instituciones que quizá nunca han trabajado juntas tienen que ponerse de acuerdo sobre preguntas muy serias: ¿quién envió la orden?, ¿había fondos?, ¿se acreditó la cuenta correcta?, ¿el pago ya es definitivo?, ¿quién le debe dinero a quién?

Y tienen que hacerlo incluso si una conexión se corta justo después de mover la plata.

Ese es el motor de pagos de un país. Desde afuera debe sentirse casi invisible. Por dentro debe ser incapaz de inventar un dólar, perder una instrucción o confundir un silencio con un rechazo.

Durante años nos han dicho que construirlo es una obra faraónica: demasiada escala, demasiada complejidad, demasiado riesgo. La palabra *nacional* parece justificar cualquier presupuesto y cualquier demora.

Pero miremos los números. Bre-B atiende a Colombia, un país casi tres veces más grande que Ecuador, y procesa alrededor de cinco millones de transacciones al día. Eso equivale a unas **sesenta por segundo en promedio**. Aunque multipliquemos esa cifra para cubrir quincenas, feriados y crecimiento futuro, no estamos frente a un desafío que requiera una tecnología secreta.

Los requisitos difíciles son concretos: que ningún dólar aparezca o desaparezca, que dos instituciones no terminen con versiones distintas del mismo pago y que, cuando algo falle a mitad del camino, el sistema sepa si debe intentar de nuevo, devolver la plata o esperar una conciliación.

<!-- TODO(IMAGEN): Apertura emocional. Fotografía nocturna de una persona usando el celular en un entorno cotidiano ecuatoriano —una enfermera al terminar turno, un conductor esperando o una madre en casa— sin marcas bancarias. El foco es la tranquilidad de poder enviar dinero, no la tecnología. -->

## Dos sistemas que deben encontrarse

Cuando decimos que estamos construyendo infraestructura de pagos, hablamos de dos lados que conviene distinguir.

El primero es **el sistema central**: el motor que conecta las redes de pago del país. Encuentra la cuenta detrás de una llave, lleva una orden desde la red de quien paga hasta la red de quien cobra, registra las obligaciones que nacen y prepara su liquidación en el Banco Central.

El segundo es **el conector de cada institución**: la pieza que permite que el sistema de un banco o una cooperativa hable con esa infraestructura nacional. Recibe una instrucción sencilla del sistema bancario, la convierte al formato común, la firma, la envía y traduce la respuesta de vuelta.

Ambos comparten la mensajería, las reglas, las pruebas y buena parte del código. Construimos una base común y la desplegamos con funciones distintas según quién la opere.

<!-- TODO(IMAGEN): Ilustración en dos mitades que se encuentran. Izquierda: bancos y cooperativas con conectores. Derecha: el sistema central con redes y directorio. En medio: un pago atravesando de una persona a otra. Debe explicar la relación, no mostrar detalles de infraestructura. -->

## Los seis componentes del sistema central

### 1. El registro sabe quién puede participar

Guarda qué redes existen, qué instituciones pertenecen a cada una, cómo se identifican y si están habilitadas. Es la lista de participantes y también el control de acceso.

### 2. El directorio sabe dónde cobrar

Relaciona una llave —tu celular, tu cédula, un correo o un alias— con una cuenta. Guardar esa relación es fácil. Lo difícil es garantizar que una llave no termine registrada en dos lugares al mismo tiempo y que todos los directorios estén de acuerdo.

### 3. La resolución contesta una pregunta humana

Cuando escribes un número y aparece «María P.», acabas de usar el servicio de resolución. Encontró la institución correcta, consultó la cuenta y devolvió la información mínima para que confirmes que no vas a pagarle a la persona equivocada.

### 4. El enrutador lleva la orden

Recibe el pago, comprueba que esté bien formado, descubre el camino, lo entrega y devuelve la respuesta. No necesita recordar conversaciones anteriores: así podemos ejecutar varias copias y reemplazar una sin perder el hilo de ningún pago.

### 5. El control de liquidez evita promesas imposibles

Cada institución tiene un límite para las obligaciones que puede acumular antes de liquidarlas. El sistema vigila ese espacio disponible y detiene nuevos pagos salientes cuando se agota.

### 6. La compensación cierra las cuentas

Durante el día, miles de pagos generan deudas en ambas direcciones. En lugar de mover dinero del Banco Central por cada compra de cinco dólares, el sistema cruza lo que todos se deben y produce una posición final por institución.

Alrededor de estas piezas hay tres responsabilidades que atraviesan todo: proteger y firmar los mensajes, registrar cada acción para poder reconstruirla y ofrecer una consola para que una persona de operaciones entienda qué está pasando.

Los seis componentes deben mantener el mismo estado cuando la red funciona mal, llegan mensajes repetidos o una institución responde tarde.

## El corazón no son los mensajes

La tentación natural es empezar por lo visible: el XML, las firmas, las conexiones. Es lo primero que aparece en cualquier documento técnico y lo primero que un proveedor puede mostrar en una demostración.

Nosotros empezamos por otra parte: **la contabilidad**.

Piensa en varios restaurantes de un patio de comidas que pasan todo el día prestándose cambio. Podrían pagarse cada moneda al instante. O podrían anotar cada movimiento y, al cierre, calcular quién le debe cuánto a quién. El sistema ecuatoriano se parece más a lo segundo: los pagos se vuelven definitivos para los clientes, mientras las obligaciones entre instituciones se acumulan y se liquidan varias veces durante el día.

Ese lapso crea un riesgo real. Una institución podría prometer más pagos de los que tiene capacidad de cubrir. Por eso cada una recibe un cupo y el sistema debe bloquearla al llegar al límite.

Podríamos programarlo con una condición:

> Antes de aceptar un pago, calcula lo comprometido, compáralo con el cupo y decide.

Parece razonable. También es frágil. Basta con que un nuevo camino del código olvide hacer esa comprobación, o que dos pagos consulten el mismo saldo al mismo tiempo, para romper el límite.

Nuestra propuesta convierte el cupo en una **cuenta de control**. Cada pago consume saldo de esa cuenta. Si no alcanza, el libro mayor se niega a registrar la operación. No depende de que un programador recuerde preguntar; la estructura misma lo impide.

Es la diferencia entre colocar un letrero que diga «no pasar» y construir una pared. El letrero funciona mientras todos lo respeten. La pared funciona incluso cuando alguien se distrae.

Por eso el libro mayor va primero. Antes de conectar un banco, queremos demostrar que ninguna combinación de pagos, rechazos, repeticiones y caídas puede crear un asiento descuadrado, gastar un cupo dos veces o borrar una obligación.

<!-- TODO(IMAGEN): Gráfico conceptual de dos paneles. Izquierda: un letrero "No pasar" frente a un camino abierto, rotulado "regla en el código". Derecha: una pared sólida, rotulada "invariante del libro mayor". Debe volver memorable la diferencia entre comprobar y hacer imposible. -->

## Qué ocurre cuando una respuesta no llega

Una institución envía una orden. La cuenta receptora se acredita. La respuesta comienza el camino de vuelta y, justo entonces, se corta la conexión.

Quien paga ve una pantalla esperando. El dinero ya se movió, pero su aplicación todavía no lo sabe. ¿Debe intentar otra vez? Si lo hace sin cuidado, puede pagar dos veces. ¿Debe devolver los fondos? Si el beneficiario ya los recibió, también duplicaría el dinero.

La única respuesta honesta es: **todavía no sabemos cómo terminó**.

El pago entra en una bandeja de conciliación. Conserva la plata reservada, busca la versión de las otras instituciones y solo entonces adopta un estado final. Un silencio no significa «no». Significa «averígualo antes de tocar el dinero».

Esto explica por qué guardamos cada transición, por qué los identificadores no se repiten y por qué una operación nunca se edita para fingir que el error no ocurrió. En infraestructura financiera, la historia completa importa tanto como el saldo actual.

<!-- TODO(HISTORIA): Contar un incidente real y anonimizado que Sebastián haya visto en otro sistema de pagos: una respuesta tardía, una transferencia duplicada o una conciliación manual. Enfocarse en la sensación de operaciones buscando "dónde quedó la plata" y en la regla aprendida; omitir empresa, cliente y datos sensibles. -->

## ¿Esto hará que los pagos sean instantáneos, interoperables y gratuitos?

**Instantáneos, sí.** El usuario debe recibir una confirmación en segundos. Nuestra meta es que la infraestructura consuma una fracción del tiempo permitido, para que una cooperativa con sistemas modestos tenga margen y no quede condenada por el eslabón central.

**Interoperables, sí.** Ese es el cambio más profundo. «¿Tienes cuenta en el Pichincha?» deja de ser el inicio de una transferencia. La llave encuentra el destino y el sistema lleva el pago, aunque las dos personas pertenezcan a instituciones y redes distintas.

**Gratuitos, no necesariamente.** Sería irresponsable prometer algo que el código no decide. El diseño permite cobrar comisiones y, por defecto, contempla que las pague quien envía.

Pero la tecnología sí puede cambiar la calidad de esa discusión. Si cada comisión y cada impuesto quedan registrados como movimientos separados, podemos simular cuánto costaría eliminar el cobro entre personas, quién lo asumiría y qué efecto tendría. En Brasil, la gratuidad para las personas fue una de las razones por las que Pix pasó de ser infraestructura bancaria a convertirse en una costumbre nacional.

No queremos defender la gratuidad con una consigna. Queremos poner números verificables sobre la mesa para que sea una decisión pública y consciente.

## ¿Sirve para empresas y sueldos?

Para una ferretería que cobra a un cliente o paga a un proveedor local, sí. El sistema no necesita saber si detrás de la cuenta hay una persona o un negocio. Poder cobrar desde cualquier banco con una llave ya sería un cambio enorme para comercios pequeños.

Para pagar nóminas grandes y automatizar cuentas por pagar, todavía faltan piezas.

El límite por transferencia deja fuera operaciones empresariales de mayor tamaño. No hay una instrucción de pago masivo, así que una nómina de tres mil personas se convierte en tres mil pagos individuales. Y la referencia que acompaña al pago todavía es pobre para que una empresa identifique automáticamente qué factura quedó cancelada.

Hay además un riesgo menos visible. Una nómina consume en pocos segundos una parte enorme del cupo de la institución que la envía. Si ese cupo solo se recupera cuando cierra el siguiente ciclo de liquidación, una cooperativa podría pagar los sueldos de una empresa y quedar bloqueada para todos los demás pagos salientes. Sus socios no verían «cupo de liquidez agotado». Verían que su cooperativa dejó de funcionar.

¿Ocurrirá con los volúmenes reales del Ecuador? Todavía no lo sabemos. Pero podemos simularlo. Y un gráfico que muestre el comportamiento de una cooperativa durante una quincena vale más que veinte opiniones alrededor de una mesa.

<!-- TODO(IMAGEN): Visualización futura de una simulación de quincena: línea de cupo disponible durante el día, caída brusca al procesar una nómina y momentos de liquidación que lo restauran. No inventar números; generar con el simulador cuando exista. -->

## Por qué construirlo en abierto

Un sistema abierto permite que un banco revise cómo se calcula una posición. Que una cooperativa pruebe si su integración sobrevive a una conexión caída. Que una universidad intente encontrar una secuencia que rompa el libro mayor. Que el Banco Central compare opciones sobre algo que funciona, en lugar de evaluar únicamente propuestas comerciales.

No significa que cualquiera pueda entrar a la red ni cambiar un saldo. El código puede ser público mientras las credenciales, las llaves criptográficas, los datos y la infraestructura siguen protegidos. Linux, PostgreSQL y buena parte de la tecnología que sostiene internet funcionan de esa manera: la cerradura se conoce; lo que permanece secreto es la llave.

Y hay una razón adicional. Con las herramientas de desarrollo actuales, un equipo pequeño puede escribir pruebas, revisar decisiones y explorar alternativas a una velocidad que hace pocos años requería decenas de personas. Eso no elimina el criterio humano ni la responsabilidad. Nos permite dedicar menos tiempo a producir documentos y más a demostrar qué funciona.

## Dónde estamos de verdad

No queremos fingir que una arquitectura escrita ya es un sistema nacional.

Al publicar este artículo estamos construyendo los cimientos: el libro mayor, las cuentas de control, las reservas y las pruebas que intentarán romper cada invariante. Después vienen las comisiones, el control de liquidez y la compensación. Solo entonces conectaremos la mensajería sobre un núcleo contable que ya haya demostrado ser correcto.

Cada etapa debe terminar en algo que cualquiera pueda ejecutar y cuestionar. No diremos «escala» sin publicar una prueba de carga. No diremos «seguro» sin explicar contra qué fallas se probó. No diremos «interoperable» hasta que dos instituciones ficticias, en redes distintas, completen un pago de principio a fin.

Esa es la invitación.

Si trabajas en una cooperativa, cuéntanos qué parte sería imposible de operar con tu equipo actual. Si trabajas en un banco o una red, dinos dónde el modelo no se parece a la realidad. Si eres ingeniero, intenta romper las reglas del libro mayor. Si trabajas en el sector público, úsalo para hacer preguntas más exigentes.

[Lee el manifiesto](/manifiesto/), revisa [el estándar explicado](/estandar/) o acompaña la construcción desde el repositorio.
