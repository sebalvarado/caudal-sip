# Manifiesto

## Mover nuestra plata no debería depender del banco de la otra persona

En Ecuador, antes de hacer una transferencia, todavía preguntamos:

—¿Tienes cuenta en el Pichincha?

Si la respuesta es no, buscamos otra cuenta, otra aplicación o dejamos el pago para después. Un negocio publica números de varias instituciones. Una cooperativa puede ofrecer un buen servicio y aun así quedar fuera de la forma en que sus clientes prefieren pagar. La persona que cobra termina adaptándose al banco de quien paga.

No debería funcionar así.

El país ya usa el teléfono para comunicarse, vender, estudiar y trabajar. No falta adopción digital. Falta una infraestructura común que permita enviar dinero entre instituciones con la misma facilidad con la que enviamos un mensaje.

Brasil lo hizo con Pix. Colombia lo hizo con Bre-B. India lo hizo con UPI. Cada país eligió una arquitectura distinta, pero los tres resolvieron el mismo problema: la persona elige dónde guardar su dinero sin quedar aislada del resto.

Ecuador ya tomó la misma dirección. Existen reglas para conectar bancos, cooperativas, mutualistas, fintechs y redes de pago. Hay una fecha de implementación y una arquitectura definida. Todavía faltan detalles técnicos y decisiones importantes, pero ya no estamos esperando permiso para comenzar.

## Por qué existe Proyecto Caudal

Queremos demostrar cuánto de este problema se puede resolver con ingeniería clara, código abierto y pruebas que cualquiera pueda repetir.

No creemos que un sistema nacional de pagos sea sencillo. Tiene que seguir moviendo dinero cuando una conexión falla, impedir pagos duplicados, proteger datos personales, mantener una contabilidad exacta y operar todos los días. Pero estos problemas ya se han resuelto en otros países y se pueden dividir en componentes que un equipo pequeño pueda construir, probar y explicar.

Las herramientas de desarrollo también cambiaron. Los agentes de inteligencia artificial ayudan a escribir pruebas, revisar código y explorar alternativas con mayor rapidez. No reemplazan la experiencia ni la responsabilidad de quienes diseñan un sistema financiero. Sí reducen el trabajo repetitivo y permiten que un equipo pequeño produzca resultados que antes exigían mucho más tiempo y presupuesto.

En lugar de empezar con un contrato y una presentación, empezamos con código. En lugar de pedir que se confíe en una promesa, publicamos una prueba.

## Lo que creemos

1. **Una persona debe poder enviar dinero a cualquier cuenta del país, a cualquier hora**, sin averiguar primero qué banco usa quien cobra.
2. **La institución más pequeña debe poder conectarse.** La interoperabilidad falla si solo pueden pagarla los bancos con grandes equipos tecnológicos.
3. **La implementación de referencia debe ser pública.** El código, las decisiones y las pruebas deben poder revisarse y reproducirse.
4. **Los datos y las credenciales siguen siendo privados.** Abrir el código no significa abrir cuentas, llaves criptográficas ni información personal.
5. **La gratuidad para las personas debe discutirse con números.** El software puede medir el costo y simular alternativas; convertirla en política corresponde a las instituciones y al país.
6. **La documentación debe estar en español** y servirle tanto a un ingeniero como a una persona de operaciones.

## Lo que estamos construyendo

El proyecto cubrirá las principales piezas del sistema:

- **El sistema central** que conecta redes, resuelve llaves, registra obligaciones, controla la liquidez y prepara la liquidación.
- **El software de una red de pagos** para enrutar operaciones, mantener su directorio de llaves, certificar integrantes y resolver pagos pendientes.
- **El conector de bancos y cooperativas** para comunicarse con una red sin construir mensajes financieros ni manejar todos los casos de falla desde cero.
- **Una red de prueba local** con instituciones ficticias, para enviar un pago completo desde una computadora.
- **Pruebas de conformidad** que permitan detectar errores antes de una certificación oficial.
- **Documentación en español** sobre la arquitectura, la operación y las decisiones pendientes.

Cada componente se podrá instalar en infraestructura propia. Quien no tenga equipo para operarlo podrá contratar a un tercero, sin quedar atado: la institución conserva la opción de migrar porque el código y los formatos son abiertos.

## Quién lo usaría

- **El Banco Central** podría evaluar el núcleo central, usarlo como ambiente de pruebas o adoptarlo total o parcialmente.
- **Una red de pagos** podría operar su enrutamiento, directorio y proceso de certificación.
- **Un banco o una cooperativa** podría instalar el conector o usarlo como servicio administrado.
- **Un proveedor de sistemas bancarios** podría integrar una vez y ofrecer la conexión a varias instituciones.
- **Un desarrollador** podría levantar el sistema completo para aprender, probar una integración o contribuir.

La persona que paga no instalará nada. Seguirá usando la aplicación de su banco o cooperativa. La diferencia estará detrás: podrá enviar dinero a otra institución sin cambiar de aplicación ni abrir otra cuenta.

## ¿Es peligroso publicar el código?

Publicar el código no publica contraseñas, certificados, llaves criptográficas ni datos de clientes. Esos elementos siguen protegidos y fuera del repositorio.

Lo público es el funcionamiento del sistema: cómo valida un mensaje, cómo registra una obligación y qué hace cuando algo falla. Eso permite que instituciones, investigadores y auditores encuentren errores antes de que lleguen a producción.

Linux, PostgreSQL y los estándares criptográficos que protegen internet se desarrollan y revisan públicamente. Su seguridad no depende de esconder cómo funcionan, sino de proteger las credenciales y corregir las fallas que se descubren.

Un repositorio abierto tampoco garantiza seguridad por sí solo. Hace falta revisión, pruebas, respuesta a vulnerabilidades y una operación responsable. Proyecto Caudal publicará esos procesos junto con el código.

## Lo que pedimos

**A quienes usan el sistema financiero:** exijan poder pagar y cobrar entre instituciones sin abrir cuentas adicionales ni esperar al siguiente día hábil.

**A bancos, cooperativas, fintechs y redes de pago:** prueben el software y señalen dónde no representa su operación. Necesitamos casos reales, no aprobación de cortesía.

**Al Banco Central del Ecuador:** publique las especificaciones técnicas pendientes en un repositorio con versiones y abra espacios de trabajo para ingenieros y operadores.

**A los ingenieros ecuatorianos dentro y fuera del país:** revisen el diseño, intenten romper las pruebas y ayuden a construir una infraestructura que sus familias podrían usar todos los días.

---

*Proyecto Caudal es un proyecto independiente de código abierto bajo licencia Apache-2.0. No está afiliado al Banco Central del Ecuador. “RPI” y “SIP” se refieren a las infraestructuras y estándares públicos definidos en la normativa ecuatoriana vigente, que este proyecto busca implementar.*
