---
title: "Ecuador ya decidió tener pagos instantáneos. Nadie te lo contó."
description: "Entre 2024 y 2026, el Banco Central del Ecuador emitió el marco completo para pagos en tiempo real interoperables, con fecha límite en noviembre de 2026. Esto es lo que dice la norma, explicado para desarrolladores y para ciudadanos."
pubDate: 2026-08-05
lang: es
---

Si le preguntas a cualquier persona en la industria, te dirá que Ecuador lleva más de una década "estudiando" los pagos instantáneos. Lo que casi nadie sabe es que la discusión ya terminó: **la norma existe, es pública y tiene fecha límite.**

## Lo que ya está decidido

Entre septiembre de 2024 y marzo de 2026, la Junta de Política y Regulación Financiera y Monetaria y el Banco Central del Ecuador (BCE) emitieron el marco regulatorio completo para transferencias electrónicas de dinero en tiempo real:

- **Resolución JPRM-2024-018-M** (septiembre 2024): regula los medios y sistemas de pago y las actividades fintech.
- **Resolución JPRM-2024-029-M** (diciembre 2024): incorpora las obligaciones de interoperabilidad y ordena al BCE construir el Sistema Integrador de Pagos en 18 meses.
- **[Resolución BCE-GG-008-2025](https://www.bce.fin.ec/storage/BCE/resoluciones/BCE-GG-008-2025.pdf)** (mayo 2025): define las etapas, hitos, plazos, requisitos de seguridad y los foros de interoperabilidad.
- **[Resolución BCE-GG-007-2026](https://www.bce.fin.ec/storage/BCE/resoluciones/BCE-GG-007-2026.pdf)** (marzo 2026): fija el estándar de mensajería — ISO 20022 — para las órdenes de pago y la gestión de llaves.

## La arquitectura que eligió Ecuador

El modelo ecuatoriano se parece más al Bre-B colombiano que al Pix brasileño: en lugar de un único switch central, habrá **redes de pagos** privadas (como BANRED) que enrutan pagos entre sus miembros, interconectadas por dos piezas del BCE:

- El **Sistema Integrador de Pagos (SIP)**: resuelve llaves y compensa los pagos *entre* redes.
- La **Red de Pagos Instantáneos (RPI)**: la red de pagos del propio Banco Central.

Las **llaves de pago** son el equivalente de las llaves Pix: un identificador público — tu número de celular, tu cédula, un alias — que apunta a la cuenta que tú elijas. El que te paga no necesita saber tu banco ni tu número de cuenta; el sistema resuelve la llave y el banco receptor valida que las credenciales correspondan a su cliente.

Los mensajes son ISO 20022: `pacs.008` para la orden de pago, `pacs.002` para la respuesta y la declaración de irrevocabilidad, y mensajes `acmt` para consultar llaves. Aplica a pagos de hasta USD 15.000, con códigos QR que deben llevar la llave del beneficiario.

## La fecha que importa

> **Hasta el 30 de noviembre de 2026**, las redes de pago y sus integrantes deberán implementar el estándar de mensajería para los pagos intra-redes.

Eso significa que unos **24 bancos privados, más de 360 cooperativas de ahorro y crédito, mutualistas y fintechs** tienen que implementar mensajería ISO 20022, resolución de llaves, firmas digitales y QR estandarizado. La gran mayoría — especialmente las cooperativas, que sirven a millones de ecuatorianos — no tiene equipos de pagos para hacerlo, y hoy no existe ni una sola herramienta open source, ni un sandbox público, ni documentación para desarrolladores en español.

## Por qué esto es una oportunidad y no un problema

Brasil nos enseñó el camino dos veces. Primero con el sistema: Pix superó los 170 millones de usuarios y hoy mueve más transferencias que ningún otro medio de pago del país. Y segundo con el método: el Banco Central do Brasil publica las especificaciones de Pix [en GitHub](https://github.com/bacen/pix-api), con licencia abierta, versiones y issues públicos.

Colombia acaba de confirmarlo: Bre-B salió a producción en octubre de 2025 y en menos de cuatro meses procesó 370 millones de transacciones con 218 entidades conectadas y 99 millones de llaves registradas.

Ecuador tiene la norma. Le falta el software. Nosotros lo estamos construyendo, en abierto, y este blog es la bitácora de esa construcción: cada componente que publiquemos vendrá acompañado de una explicación de cómo funciona un sistema de pagos por dentro — liquidación bruta en tiempo real, directorios de llaves, irrevocabilidad, prevención de fraude — en español y con código que puedes correr en tu computadora.

Si eres desarrollador, trabajas en una cooperativa o un banco, o simplemente crees que tu plata debería llegar en segundos a cualquier cuenta del país: [lee el manifiesto](/manifiesto/) y súmate.
