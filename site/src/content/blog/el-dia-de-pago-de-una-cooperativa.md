---
title: "El día de pago de sueldos de una cooperativa"
description: "Simulamos un día completo de pagos instantáneos con las reglas del Catálogo Operacional. Una cooperativa del segmento 3 paga 300 sueldos a las nueve de la mañana y queda bloqueada hasta la tarde. Te contamos qué pasó, por qué pasa y qué se puede hacer."
pubDate: 2026-10-11
lang: es
---

> Demostración con datos ficticios. Proyecto Caudal es independiente y no está afiliado al Banco Central del Ecuador.

En agosto escribimos que una nómina podía dejar a una cooperativa sin poder enviar pagos, y que eso no se resolvía opinando sino simulando. Ya lo simulamos.

Armamos una red de pagos instantáneos completa —el Sistema Integrador de Pagos (SIP), dos redes que compiten entre sí y cuatro instituciones ficticias— y le hicimos pasar un día entero de tráfico: más de veinte mil pagos entre las ocho de la mañana y las cinco y media de la tarde. Cada pago se firma, se codifica en ISO 20022, se reserva en el libro del SIP y se liquida cuando cierra la malla, con las reglas que publicó el Banco Central en el Catálogo Operacional.

Entre esas instituciones está coop-beta, una cooperativa del segmento 3. Ese día le toca pagar 300 sueldos.

## Qué pasó a las nueve de la mañana

A las 09:00 coop-beta empieza a enviar la nómina. Media hora después ya comprometió el 99,99 % de su techo de riesgo. De los 300 sueldos, 132 llegan. Los otros 168 se rechazan con el código `AM04`: cupo insuficiente.

No es una falla del sistema. Es la regla funcionando como está escrita.

El catálogo le asigna a cada integrante un **nivel de riesgo aceptable**, calculado sobre su encaje (§10, Tablas 19 y 20). Es el máximo que puede comprometer en pagos que todavía no se han liquidado. Para una cooperativa con USD 6 millones en captaciones, como coop-beta, ese techo queda en USD 60.000. Una nómina de 300 sueldos de entre 180 y 520 dólares suma más de cien mil.

Lo que hace interesante el caso es cuándo vuelve el cupo. Los pagos son irrevocables en segundos, pero las obligaciones entre instituciones se acumulan y se liquidan por mallas, varias veces al día. Recién cuando la malla se liquida, el cupo usado regresa. En nuestra simulación las mallas cierran a las 11:00, a las 15:00 y a las 17:30.

Así que coop-beta reintenta los sueldos pendientes a las 11:00, y la nómina vuelve a llenar el techo en cuestión de minutos. El último sueldo se acredita a las 15:00. La nómina debía terminar antes de las diez de la mañana.

## Lo que no se ve desde afuera

Mientras el techo está lleno, la cooperativa no puede enviar ningún pago a otra red. No solo los sueldos: también la transferencia del socio que quiere pagarle al proveedor o mandarle plata a su hija. En el día simulado hubo 1.061 rechazos por cupo, todos de coop-beta.

El resto de la red ni se entera. El banco y la mutualista nunca comprometen más del 6 % de su techo. La fintech, con un techo todavía más chico que el de la cooperativa, llega como máximo al 72 % y se queda en verde todo el día, porque su tráfico es parejo y no tiene una nómina que pagar.

Eso es lo que hace delicado este riesgo: le pega a una sola institución, justo a la más chica, y sus socios no ven un mensaje de "cupo de liquidez agotado". Ven que su cooperativa dejó de funcionar.

## Las tres liquidaciones

La compensación neta hace bien su trabajo. En la malla de las 11:00 se cruzaron 1.443 pagos por USD 359.212,02, y para cerrarlos bastaron cuatro movimientos en el banco central, uno por institución, que sumaron USD 223.235,12. A las 15:00 fueron 1.624 pagos por USD 427.344,09; a las 17:30, 1.150 por USD 216.073,38.

El ahorro no es mayor porque en nuestro día simulado la plata fluye sobre todo en una dirección: de las instituciones grandes hacia las pequeñas. Cuando los pagos van y vienen, el neto suele ser una fracción mucho más chica.

De los 19.407 pagos aceptados, 15.190 se quedaron dentro de su propia red y nunca pasaron por el SIP. Es lo esperable cuando las dos instituciones más grandes están en la misma red: la mayoría de sus clientes se pagan entre sí.

## La velocidad no es el problema

También medimos cuánto cómputo cuesta cada salto. Codificar o leer un mensaje ISO 20022 toma unos 5 microsegundos; firmarlo o verificarlo, unos 33; escribir en el libro del SIP, alrededor de 1. Son pisos, no promesas: no incluyen la red, el TLS ni el core bancario de cada institución. Pero dejan clara una cosa: un día entero de pagos de esta red se procesa en segundos en una laptop.

La dificultad de un sistema nacional de pagos nunca fue la velocidad. Está en reglas como esta, que tienen todo el sentido para proteger al sistema y que, mal calibradas, pueden dejar fuera justo a quien más necesita estar dentro.

## Qué se puede hacer

No tenemos la respuesta, pero ahora tenemos con qué discutirla:

- **Más liquidaciones al día.** Con una malla cada hora, el cupo de coop-beta habría vuelto antes. El catálogo deja el horario parametrizable.
- **Que la nómina salga después de una liquidación**, no antes. Es una decisión operativa de la cooperativa, y la simulación muestra cuánto pesa.
- **Avisar mucho antes del rojo.** El catálogo pide un semáforo; la consola de cada integrante tiene que hacerlo visible a las 09:15, no a las 15:00.
- **Revisar el techo para los días pico.** El cálculo es semanal. Vale la pena preguntarse si un recálculo semanal alcanza para absorber un pico de un solo día.

## Lo que todavía es supuesto

Somos claros con los límites. Las instituciones, sus captaciones y el tráfico son inventados; los techos salen del método del catálogo aplicado a esas cifras. Para la fintech usamos solo el 0,5 % base de encaje, porque el catálogo no publica el porcentaje adicional que le corresponde a una SEDPES. El formato de los mensajes es nuestro perfil **[supuesto]**, porque el catálogo técnico todavía no se publica. No modelamos el bloqueo en rojo con rehabilitación manual: aquí el pago que excede el cupo simplemente se rechaza. Y el horario de liquidación es un ejemplo.

## Pruébalo tú

Los resultados completos, hora por hora, están en [la página de la simulación](/simulacion/). La simulación es determinista: con la misma semilla da exactamente los mismos pagos. Puedes repetirla en tu computadora con un solo comando, cambiar el tráfico o el horario de las mallas, y ver qué le pasa a coop-beta.

Y si prefieres empezar por un solo pago, en [la demo](/demo/) puedes resolver una llave, enviar un pago entre redes y ver cada mensaje que viaja entre las instituciones.

El código está en [GitHub](https://github.com/sebalvarado/caudal-sip), bajo licencia Apache-2.0. Si una cooperativa, una red o el propio Banco Central quiere correr esta simulación con sus números reales, puede hacerlo sin pedirnos permiso. Para eso lo hicimos abierto.
