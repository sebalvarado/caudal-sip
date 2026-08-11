# Consultas técnicas sobre el Catálogo Operacional del SIP

> **Borrador, sin enviar.** Reemplazar los `[…]` antes de enviar. Registro en [`README.md`](README.md).

---

Quito, `[fecha]`

Señores
**Banco Central del Ecuador**
Gerencia de Medios de Pago y Servicios Nacionales
Presente

**Asunto:** Consultas técnicas sobre el *Catálogo Operacional para transferencias electrónicas de pago en tiempo real — Sistema Integrador de Pagos*, v1.0, marzo de 2026.

De mi consideración:

Mi nombre es `[nombre]`, soy ingeniero de software especializado en sistemas de pagos, con experiencia en `[breve, una línea]`. Junto con un grupo de colaboradores estamos desarrollando una implementación de software libre del estándar ecuatoriano de pagos en tiempo real, publicada bajo licencia Apache 2.0, con el propósito de que cualquier institución del sistema financiero —en particular las cooperativas, que son las que enfrentan la mayor carga de cumplimiento frente al plazo del 30 de noviembre de 2026— disponga de herramientas listas para conectarse al estándar.

El proyecto es independiente y no está afiliado al Banco Central del Ecuador. Trabajamos exclusivamente sobre documentación pública.

Al estudiar en detalle el Catálogo Operacional encontramos cinco puntos donde el documento admite más de una lectura. Los ponemos en su conocimiento porque cualquiera de ellos podría producir implementaciones incompatibles entre integrantes, y porque resolverlos temprano es mucho más barato que corregirlos después de la certificación.

## 1. Versión del mensaje de orden de pago

La sección 8, en las tablas de campos obligatorios, especifica **`pacs.008.001.08`**. La sección 11.3, en la descripción de los flujos, se refiere al mismo mensaje como **`pacs.008.004.08 BCE in/out`**.

Entendemos que se trata de una discrepancia editorial, pero las dos versiones tienen esquemas distintos y la elección determina el código de validación de cada integrante. ¿Cuál es la versión normativa?

## 2. Estado `PDNG`

En la Tabla 11, el estado `PDNG` se denomina **"Pago por Conciliar"**, mientras que su columna de explicación dice: *"Diferido (Próximo Día): El pago fue recibido después del Cut-off. Se moverá a la malla del siguiente día hábil."*

Son dos situaciones distintas. Nuestra duda concreta: la excepción 3 de la sección 11.4 describe un pago **irrevocable cuyo resultado el lado ordenante desconoce** porque la red beneficiaria no logró devolver la confirmación. Esa condición es, operativamente, la más delicada del esquema.

¿`PDNG` corresponde a esa condición, al diferimiento por cut-off, o a ambas? Y si corresponde solo al diferimiento, ¿con qué estado debe marcarse un pago irrevocable pendiente de conciliación?

## 3. Niveles de riesgo para los segmentos cooperativos 4 y 5

En la Tabla 19, las filas de los códigos de sector `0007` (Cooperativas de Ahorro y Crédito Segmento 4) y `0008` (Segmento 5) aparecen **sin porcentaje de requerimiento de encaje y sin nivel de riesgo aceptable**, a diferencia de todas las demás filas.

¿Es una omisión de edición, o esos parámetros están pendientes de definición? Lo consultamos porque, siendo las instituciones más pequeñas del sistema, son también las que tienen menos margen para absorber un techo de liquidez estrecho, y sin esos valores no podemos modelar su comportamiento.

## 4. Horarios de operación y de liquidación

La sección 5, regla 8, establece que el administrador del SIP parametriza el horario de operaciones, y la sección 9, regla 2, que el horario de liquidación de las mallas también es parametrizable. El catálogo no publica los valores.

¿Están definidos los horarios de operación y la cantidad y las horas de las liquidaciones diarias previstas? La pregunta tiene un efecto práctico directo: la sección 9, regla 4, establece que las órdenes cursadas fuera del horario de operaciones se liquidan el primer día laborable siguiente, y sin conocer la ventana no es posible dimensionar la exposición acumulada de un fin de semana o un feriado.

## 5. Valores de los *timeouts* por etapa

La sección 5, regla 13, indica que cada etapa del pago tiene un *timeout* parametrizable. Como el techo de 20 segundos entre inicio y confirmación es un requisito duro, los presupuestos por tramo determinan el diseño de cada implementación.

¿Están definidos esos valores, o se fijarán durante la etapa de certificación técnica y operativa?

---

## Sobre el catálogo técnico

Adicionalmente, agradeceríamos conocer si el **catálogo técnico** —esquemas y reglas de validación campo por campo— será publicado, y por qué medio. Es la pieza que hoy impide construir validadores conformes, y su disponibilidad temprana beneficiaría a todo el ecosistema, no solo a este proyecto.

## Lo que ofrecemos

Todo nuestro trabajo es público y está a disposición del Banco Central sin condiciones:

- Una lectura ordenada del Catálogo Operacional, con referencias a sus secciones, en `[URL]/estandar/`. Fue elaborada para que una institución pueda entender qué se le va a exigir; si contiene errores, agradeceremos la corrección y la aplicaremos públicamente.
- Un análisis del motor de riesgo de liquidez de la sección 10 y de su comportamiento fuera del horario de operaciones, incluida una propuesta técnica de liquidación continua con antecedentes de Brasil, Estados Unidos y el Reino Unido.
- La intención de publicar, cuando esté construido, un simulador que permita estimar con datos públicos el comportamiento del esquema bajo distintas configuraciones —entre otras, el efecto de un día de pago de nóminas sobre el cupo de liquidez de una cooperativa pequeña.

Quedamos a disposición para ampliar cualquiera de estos puntos por el medio que ustedes prefieran, y con mucho interés en conocer los criterios de participación en los foros de interoperabilidad previstos en la Resolución BCE-GG-008-2025.

Agradezco de antemano su atención.

Atentamente,

`[nombre]`
`[correo]` · `[teléfono]`
`[URL del proyecto]`
