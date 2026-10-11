---
title: "El día de pago de sueldos de una cooperativa"
description: "Simulamos un día completo de pagos instantáneos con las reglas del Catálogo Operacional. Una cooperativa del segmento 3 paga 300 sueldos a las nueve de la mañana y sus socios no pueden pagar a otra red hasta la tarde. Te contamos qué pasó, por qué pasa y qué se puede hacer."
pubDate: 2026-10-11
lang: es
---

> Demostración con datos ficticios. Proyecto Caudal es independiente y no está afiliado al Banco Central del Ecuador.

En agosto escribimos que una nómina podía dejar a una cooperativa sin poder enviar pagos, y que eso no se resolvía opinando sino simulando. Ya lo simulamos.

Armamos una red de pagos instantáneos completa —el Sistema Integrador de Pagos (SIP), dos redes que compiten entre sí y cuatro instituciones ficticias— y le hicimos pasar un día entero de tráfico: más de veinte mil pagos entre las ocho de la mañana y las cinco y media de la tarde. Cada pago se firma, se codifica en ISO 20022, se reserva en el libro del SIP y se liquida cuando cierra la malla, con las reglas que publicó el Banco Central en el Catálogo Operacional.

Entre esas instituciones está coop-beta, una cooperativa del segmento 3. Ese día le toca pagar 300 sueldos.

## Qué pasó a las nueve de la mañana

A las 09:00 coop-beta empieza a enviar la nómina. Media hora después ya comprometió el 99,96 % de su techo de riesgo. De los 300 sueldos, 130 llegan. Los otros 170 se rechazan con el código `AM04`: cupo insuficiente.

No es una falla del sistema. Es la regla funcionando como está escrita.

El catálogo le asigna a cada integrante un **nivel de riesgo aceptable**, calculado sobre su encaje (§10, Tablas 19 y 20). Es el máximo que puede comprometer en pagos que todavía no se han liquidado. Para una cooperativa con USD 6 millones en captaciones, como coop-beta, ese techo queda en USD 60.000. Una nómina de 300 sueldos de entre 180 y 520 dólares suma más de cien mil.

Lo que hace interesante el caso es cuándo vuelve el cupo. Los pagos son irrevocables en segundos, pero las obligaciones entre instituciones se acumulan y se liquidan por mallas, varias veces al día. Recién cuando la malla se liquida, el cupo usado regresa. En nuestra simulación las mallas cierran a las 11:00, a las 15:00 y a las 17:30.

Así que coop-beta reintenta los sueldos pendientes apenas se liquida la malla de las 11:00. Esta vez entran todos: el último se acredita a las 11:02, cuando la nómina debía terminar antes de las diez de la mañana. Pero esos 170 sueldos vuelven a llenar el techo, y la cooperativa queda otra vez al 99,99 % hasta la malla de las 15:00.

## Lo que no se ve desde afuera

Mientras el techo está lleno, la cooperativa no puede enviar ningún pago a otra red. No solo los sueldos: también la transferencia del socio que quiere pagarle al proveedor o mandarle plata a su hija. Entre las 09:30 y las 11:00, y otra vez entre las 11:02 y las 15:00, todos esos pagos se rechazan. En el día simulado hubo 1.085 rechazos por cupo, todos de coop-beta.

El resto de la red ni se entera. El banco y la mutualista nunca comprometen más del 5 % de su techo. La fintech, con un techo todavía más chico que el de la cooperativa, llega como máximo al 67 % y se queda en verde todo el día, porque su tráfico es parejo y no tiene una nómina que pagar.

Eso es lo que hace delicado este riesgo: le pega a una sola institución, justo a la más chica, y sus socios no ven un mensaje de "cupo de liquidez agotado". Ven que su cooperativa dejó de funcionar.

## Las tres liquidaciones

La compensación neta hace bien su trabajo. En la malla de las 11:00 se cruzaron 1.427 pagos por USD 319.914,25, y para cerrarlos bastaron cuatro movimientos en el banco central, uno por institución, que sumaron USD 184.614,53. A las 15:00 fueron 1.566 pagos por USD 393.601,28; a las 17:30, 1.275 por USD 252.031,62.

El ahorro no es mayor porque en nuestro día simulado la plata fluye sobre todo en una dirección: de las instituciones grandes hacia las pequeñas. Cuando los pagos van y vienen, el neto suele ser una fracción mucho más chica.

De los 19.385 pagos aceptados, 15.117 se quedaron dentro de su propia red y nunca pasaron por el SIP. Es lo esperable cuando las dos instituciones más grandes están en la misma red: la mayoría de sus clientes se pagan entre sí.

## La velocidad no es el problema

También medimos cuánto cómputo cuesta cada salto. Codificar o leer un mensaje ISO 20022 toma unos 5 microsegundos; firmarlo o verificarlo, unos 32; escribir en el libro del SIP, menos de 1. Son pisos, no promesas: no incluyen la red, el TLS ni el core bancario de cada institución. Pero dejan clara una cosa: un día entero de pagos de esta red se procesa en segundos en una laptop.

La dificultad de un sistema nacional de pagos nunca fue la velocidad. Está en reglas como esta, que tienen todo el sentido para proteger al sistema y que, mal calibradas, pueden dejar fuera justo a quien más necesita estar dentro.

## Qué se puede hacer

No tenemos la respuesta, pero ahora tenemos con qué discutirla:

- **Más liquidaciones al día.** Con una malla cada hora, el cupo de coop-beta habría vuelto antes. El catálogo deja el horario parametrizable.
- **Que la nómina salga después de una liquidación**, no antes. Es una decisión operativa de la cooperativa, y la simulación muestra cuánto pesa.
- **Avisar mucho antes del rojo.** El catálogo pide un semáforo, pero el de coop-beta pasó de verde a naranja en menos de media hora. La consola de cada integrante tiene que avisar cuando una nómina no cabe en el cupo, antes de enviarla.
- **Reintentar por partes.** Reenviar los 170 sueldos de golpe volvió a llenar el techo por cuatro horas. Repartirlos entre las mallas deja espacio para los pagos del resto de los socios.
- **Revisar el techo para los días pico.** El cálculo es semanal. Vale la pena preguntarse si un recálculo semanal alcanza para absorber un pico de un solo día.

## Lo que todavía es supuesto

Somos claros con los límites. Las instituciones, sus captaciones y el tráfico son inventados; los techos salen del método del catálogo aplicado a esas cifras. Para la fintech usamos solo el 0,5 % base de encaje, porque el catálogo no publica el porcentaje adicional que le corresponde a una SEDPES. El formato de los mensajes es nuestro perfil **[supuesto]**, porque el catálogo técnico todavía no se publica. No modelamos el bloqueo en rojo con rehabilitación manual: aquí el pago que excede el cupo simplemente se rechaza. Y el horario de liquidación es un ejemplo.

## Pruébalo tú

Los resultados completos, hora por hora, están en [la página de la simulación](/simulacion/). La simulación es determinista: con la misma semilla da exactamente los mismos pagos.

Y no hace falta instalar nada para cambiarla. En [el simulador](/simulador/?plantilla=national) está el guion de este mismo día: puedes mover las mallas, agregar una a las 10:00, bajar el techo de la cooperativa o repartir la nómina, y correrlo en tu navegador. La red completa, con cada mensaje firmado y verificado, corre en tu computadora; nada viaja a un servidor.

El código está en [GitHub](https://github.com/sebalvarado/caudal-sip), bajo licencia Apache-2.0. Si una cooperativa, una red o el propio Banco Central quiere correr esta simulación con sus números reales, puede hacerlo sin pedirnos permiso. Para eso lo hicimos abierto.
