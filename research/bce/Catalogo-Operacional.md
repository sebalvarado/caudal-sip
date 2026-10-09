# Catalogo-Operacional

> Transcripción mecánica del PDF oficial para búsqueda y lectura por agentes. La extracción de PDF puede alterar el orden de columnas o perder texto rotado; el PDF adjunto prevalece siempre, especialmente para tablas, firmas y contenido regulatorio. No es una versión oficial en Markdown.

- **Fuente oficial:** https://www.bce.fin.ec/storage/BCE/resoluciones/Catalogo-Operacional.pdf
- **PDF local:** [`Catalogo-Operacional.pdf`](Catalogo-Operacional.pdf)
- **Descargado:** 2026-08-18
- **SHA-256 del PDF:** `117e872c17fbe177fffac1393dcbab90f35ed69f247ce0db48ad43d545e40b9f`
- **Páginas:** 51


---

## Página 1

CATÁLOGO OPERACIONAL PARA TRANSFERENCIAS ELECTRÓNICAS DE
                               PAGO EN TIEMPO REAL


                        SISTEMA INTEGRADOR DE PAGOS







                                Catálogo Operacional               Marzo
                                                                   2026


  Gerencia   de   Medios   de   Pago   y   Servicios               Versión 1.0
  Nacionales.

---

## Página 2

Índice

         1.GLOSARIO
         2. SISTEMA INTEGRADOR DE PAGOS
                  2.1 Funcionalidades
                  2.2 Reglas de uso del Sistema Integrador de Pagos:
         3. SISTEMA CENTRAL DE LLAVES
                  3.1 Reglas de negocio
         4. PROCESO DE MIGRACIÓN MASIVA DE LLAVES DE UN INTEGRANTE HACIA OTRA RED
         DE PAGOS.
         5. REGLAS PARA EL PROCESO DE LAS ÓRDENES DE PAGO
         6. SERVICIOS DE LAS REDES DE PAGOS
         7. CÓDIGO ÚNICO DEL IDENTIFICADOR DE LA ORDEN DE PAGO.
         8. ESTÁNDAR DE MENSAJERÍA DE PAGOS
         9. COMPENSACIÓN
         10. RIESGOS DE LIQUIDEZ
         11. FLUJOS DEL SISTEMA INTEGRADOR DE PAGOS
                  11.1 Llaves
                  11.2 Credenciales de Pagos
                  11.3 Órdenes de Pago Inter Redes y sus Excepciones
                  11.4. Resumen de excepciones en el proceso de órdenes de pago
         12.REPORTES GENERALES PARA EL ADMINISTRADOR DEL SISTEMA DE LLAVES
                  12.1 Gestión de llaves del SIP
                  12.2 Reporte del proceso de conciliación de llaves
                  12.3 Reglas de conciliación de llaves
                  12.4 Resumen de reglas de conciliación de llaves
         13. REPORTES GENERALES PARA EL ADMINISTRADOR DEL SISTEMA (PAGOS)
                  13.1 Reporte del proceso de conciliación de pagos
                  13.2 Reglas de conciliación de pagos


         ANEXO 1.: CÓDIGOS PARA CONFORMACIÓN DE CAMPO ALFANUMÉRICO
         ANEXO 2.: LISTA DE CÓDIGOS BIC

---

## Página 3

1.   GLOSARIO

        BCE: Banco Central del Ecuador

        Compensación: Proceso realizado para determinar la posición neta a favor o en contra
         de  los  integrantes  de  las  redes  de  pagos,  que  deben  ser  liquidadas  mediante
         afectaciones de sus cuentas corrientes en el Banco Central del Ecuador;

        Credenciales de pago: Conjunto de datos propios del cliente que se utilizan para realizar
         o  recibir  recursos  en  una  transferencia  electrónica  para  efectuar  pagos.  Estas
         credenciales comprenden, al menos, el número de cuenta o tarjeta de crédito, el tipo
         de cuenta, el número de la identificación de personas naturales o jurídicas y el partícipe
         vinculado;

        Cliente  beneficiario:       Cliente  destinatario  que  recibe  los  recursos  objeto  de  una
         transferencia de pago;

        Cliente ordenante:        Cliente que inicia y autoriza una transferencia de pago para la
         acreditación de recursos a un cliente beneficiario;

        Código QR (Quick Response Code): Tipo de codificación de información bidimensional
         con campos específicos y estandarizados, definidos por el Banco Central del Ecuador,
         para facilitar las transacciones de pago. Los códigos QR podrán ser dinámicos o estáticos.
         Son dinámicos cuando contienen información que se puede cambiar después de ser
         generada; y, estáticos cuando contienen información fija que no puede ser editada ni
         cambiar después de ser generada;

        Cuentas: Se refiere a las cuentas de depósito a la vista, de pago electrónico;

        Directorio Distribuido: Estructura de datos y modelo operacional administrado por las
         redes  de  pagos,  destinado  a  almacenar  y  gestionar  las  llaves,  junto  con  toda  la
         información relevante asociada a los clientes;

        Integrantes de las redes de pagos (IR): Son los bancos privados; bancos públicos que
         realizan intermediación y captan recursos  del público en  general;  cooperativas de
         ahorro y crédito; asociaciones mutualistas de ahorro y crédito para la vivienda; cajas
         centrales;  neobancos;  sociedades  especializadas  de  depósitos  y  pagos  electrónicos
         (SEDPES); los partícipes del Sistema Auxiliar de Pagos que se encuentren debidamente
         autorizados en el “servicio de procesamiento de medios de pago electrónicos” en
         representación de una o varias entidades financieras; y, el Banco Central del Ecuador.
         Los integrantes actúan en una red de pagos con su infraestructura tecnológica de
         manera directa o la provista por un tercero, de forma indirecta;

        Integrante beneficiario:        Integrante de una red de pagos que permite a su cliente
         beneficiario recibir los recursos provenientes de un pago;

        Integrante ordenante: Integrante de una red de pagos que permite a su cliente ordenar
         un pago;

---

## Página 4

     Liquidación:    Función exclusiva del Banco Central del Ecuador, mediante la cual los
          resultados  de  las  posiciones  netas  de  los  procesos  de  compensación  se  cancelan
          mediante débitos o créditos en las cuentas que mantienen las entidades participantes
          en el Banco Central del Ecuador;

         Llave: Es el identificador único y público que permite al cliente vincular sus credenciales
          de pago con su información personal, como su número de identificación, número de
          teléfono celular o dirección de correo electrónico personal, para realizar o recibir pagos;

         Operaciones Inter-Redes:         Son aquellas operaciones realizadas entre integrantes de
          diferentes redes de pagos;

         Operaciones Intra-Redes: Son aquellas operaciones realizadas entre integrantes de una
          misma red de pago;

         Orden de Pago (OP): Es la instrucción que emite el Usuario Ordenante para que con
          cargo a su cuenta transfiera una cantidad de dinero a la cuenta del Usuario Beneficiario;

         Pago Instantáneo: Se refiere a una modalidad de transferencia de fondos en la que el
          dinero se transfiere de una cuenta a otra en cuestión de segundos o minutos, operando
          en tiempo real y disponible 24/7. Este tipo de pagos se realiza de manera inmediata,
          facilitando la disponibilidad inmediata de los recursos para el beneficiario;

         Pago   en tiempo real: Es una transacción electrónica mediante la cual se transfieren
          recursos de forma casi instantánea desde la cuenta del ordenante a la del beneficiario.
          El cliente ordenante inicia el pago mediante el uso de llaves, códigos QR o credenciales
          de pago, independientemente de los partícipes que provean los servicios de pago. Este
          proceso se realiza de forma continua, operando las 24 horas del día, los 365 días del
          año;

         Posición Bilateral Neta. - Es el resultado de la compensación recíproca de obligaciones
          de pago entre dos Integrantes. El resultado neto es igual a la suma del valor de todas las
          transacciones de pago a favor de un Integrante, durante un periodo de tiempo, menos
          la suma del valor de todas las transacciones de pago a su cargo.

         Posición  Multilateral  Neta.        -  Es  el  resultado  de  la  compensación  recíproca  de
          obligaciones de pago entre tres o más Integrantes. El resultado neto es igual a la suma
          del valor de todas las transacciones de pago a favor de un Integrante, durante un
          periodo de tiempo, menos la suma del valor de todas las transacciones de pago a su
          cargo.

         Redes de pagos (RP): Infraestructuras tecnológicas con procedimientos y servicios, para
          canalizar transferencias electrónicas de dinero para efectuar pagos en tiempo real;

         Red de Pagos Instantáneos (RPI):            Infraestructura tecnológica con procedimientos y
          servicios, para canalizar transferencias electrónicas de dinero para efectuar pagos en
          tiempo real, administrada por el Banco Central del Ecuador;

---

## Página 5

    Resolución de Llaves: Es el proceso mediante el cual se realiza la búsqueda de la llave
          en los directorios distribuidos, para obtener la información necesaria para verificar el
          cliente beneficiario y realizar el pago;

         SEDPES: Sociedades especializadas de depósitos y pagos electrónicos (SEDPES). - Son
          Participantes cuyo objeto social único es la recepción de recursos con fines exclusivos
          de facilitar pagos y traspasos de recursos mediante los medios de pago electrónicos
          autorizados; y, enviar y recibir giros financieros;

         Sistema Central de Llaves:         Modelo operativo administrado por el Banco Central del
          Ecuador, diseñado para gestionar la información necesaria de los clientes, facilitando la
          interoperabilidad de pagos mediante llaves;

         Sistema Integrador de Pagos:           Infraestructura tecnológica, administrada por el Banco
          Central del Ecuador, con procedimientos y servicios que gestiona las operaciones de
          llaves,  procesa  y  compensa  las  transferencias  de  pagos  inter-redes  y  remite  la
          información requerida para la liquidación;

         Tiempo Real: Se refiere a la capacidad que tiene una plataforma de pagos en procesar
          y compensar transacciones de manera casi instantánea. El tiempo máximo será definido
          por el Banco Central del Ecuador.

---

## Página 6

2.   SISTEMA INTEGRADOR DE PAGOS (SIP).

               2.1 Funcionalidades

El Sistema Integrador de Pagos tendrá las siguientes funcionalidades principales:

1.   Opera en tiempo real las 24 horas al día, los 7 días a la semana, todos los días del año;
     procesando las órdenes de pago de los clientes.
2.   Integra las RP;
3.   Mantiene información básica de las Llaves;
4.   Consulta a los Directorios Distribuidos de las RP, las credenciales asociadas a una Llave;
5.   Canaliza las credenciales de pago para la iniciación de un pago inter-redes;
6.   Canaliza las órdenes de pago inter-redes;
7.   Monitorea y controla el riesgo de liquidez en las órdenes de pago inter-redes;
8.   Monitorea que las RP se encuentren en estado activo para procesar pagos inter-redes;
9.   Permite determinar cuándo una RP se encuentra desconectado;
10.  Genera alertas tempranas por factores de riesgo de liquidez de las IR.
11.  Permite la deshabilitación temporal de un IR de los pagos inter-redes por factores de riesgo;
     así como de las IR de la RPI del BCE. La habilitación de la IR será mediante un proceso manual
     que lo realizará el Administrador del Sistema.
12.  Permite desconectar a una RP;
13.  Dispone de un Dashboard con información de las posiciones de los IR en el SIP y sus niveles
     de riesgo, entre otros aspectos, información a la que tiene acceso el Administrador del
     Sistema, las RP y sus IR de acuerdo con sus perfiles;
14.  Compensa en tiempo real las órdenes de pago inter-redes;
15.  Permite la trazabilidad de los pagos inter-redes (mediante un código único de identificación
     del pago, generado por la IR);
16.  Disponer  de  mecanismos  de  contingencia  que  garanticen  la  continuidad  operativa,
     documentado en un plan de contingencia, probado y comprobable;
17.  Genera reportes de operación en función de las necesidades del Administrador y de las RP,
     conforme a los perfiles de usuario detallados en este documento;
18.  Parametriza el cobro de comisiones;
19.  Brinda la información para el proceso de conciliación de Llaves y Pagos;
20.  Empadrona  información  relacionando:  IR,  RP,  y  producto  (Llave/Sin                    Llave),  la  que  se
     almacenará en una base de datos, para facilitar la operación en dos RP.


     2.2 Reglas de uso del Sistema Integrador de Pagos:

     1.   El  SPI  proporcionará  las  instrucciones  a  las         RP,  así  como  sus      IR  para  su  correcta
          participación.
     2.   El IR Ordenante confirmará el débito de los fondos y el IR Beneficiario acreditará los
          fondos una vez que el          SIP  haya  validada la OP y ésta haya sido remitida a la  RP
          Beneficiaria.
     3.   El IR  Beneficiario  puede  aceptar  o  rechazar  la  orden  de  pago  (dependerá  de  las
          restricciones del tipo de cuenta, por ejemplo, límite del valor a acreditarse en una cuenta
          básica). De su parte, el SIP remite estas respuestas a las RP, con los siguientes estados de
          la OP: a) aceptada, b) rechazada y c) por conciliar; esta última cuando la RP Beneficiaria
          no se logra comunicar con el SIP.

---

## Página 7

4.   Los  reintentos  no  alteran  la  condición  de  rechazos  de  pago  por  timeout  o  la
          irrevocabilidad del pago. Permite superar la condición de pago por conciliar en el caso
          de que la OP remitida por la IR Beneficiaria sea aceptada.
     5.   Si el SIP realizó la compensación de una OP y pierde conexión al momento de recibir la
          respuesta de la RP Beneficiaria; esta operación entrará en el proceso de conciliación de
          acuerdo al flujo presentado en el presente documento
     6.   En el aplicativo del IR Ordenante, se debe garantizar que en el momento de solicitar una
          OP por parte del Cliente Ordenante se realice la confirmación del valor a pagar y se debe
          visualizar el nombre        del Cliente Beneficiario, previo confirmar la             OP, sea con Llave o
          credencial.

---

## Página 8

3.   SISTEMA CENTRAL DE LLAVES

          3.1 Reglas de negocio

Respecto al manejo de Llaves, se definen las siguientes reglas:

1.   Se asignará una Llave a una única cuenta por cliente.
2.   El Sistema Central de Llaves deberá estar sincronizado y actualizado en tiempo real con los
     Directorios Distribuidos de cada Red.
3.   Para evitar duplicidad de las Llaves, al momento de la creación de las mismas, se verifica en
     el Sistema Central de Llaves la existencia de la Llave consultada por la RP.

     3.1  Si existe registro de la Llave, se niega y comunica a la RP con el mensaje “Llave ya existe”.
     3.2            Si no existe el registro de la        Llave, se procede con su creación en el Sistema
                    Central de Llaves y se comunica a la RP solicitante para que esta proceda con su
                    registro en tiempo real.
          3.2.1     La información que debe registrar en el Sistema Central de Llaves de acuerdo
                    con el tipo de Llaves es la siguiente:


                                                          Tabla No.1

            CÓDIGO               TIPO DE LLAVE                    TIPO DE CAMPO                FORMATO LÍMITE
               0001        número del celular                 Alfanumérico                    10 caracteres
               0002        número                         de  Numérico                        De  10   (número  de
                           identificación                                                     cédula)    hasta    13
                                                                                              caracteres (número
                                                                                              de RUC).
                                                                                              *No pasaporte.
               0003        correo electrónico                 Alfanumérico                    Hasta               250
                                                              -Acepta       caracteres        caracteres
                                                              especiales
                                                              -Letras en mayúsculas
                                                              y minúsculas.
                                                              -Debe             contener
                                                              siempre el carácter @
                                                              -Debe    contener    un
                                                              dominio              válido
                                                              después del @
               0004                alfanumérico               Alfanumérico                    Hasta 20  caracteres

          Nota: 1. El Código de la RP del Cliente será asignado por el BCE (hasta 11 caracteres alfanuméricos).

          2. Para la transformación y unicidad de la Llave número celular, las IR antepondrán el
          código  asignado       por  el  Banco  Central  del  Ecuador  para  la  generación  de  Llaves
          Alfanuméricas.
          3. El código del IR será asignado por el BCE (hasta 11 caracteres alfanuméricos), código
          BIC.

---

## Página 9

4. La consulta de Llaves podrá tener los siguientes estados: Activa, Suspendida, Anulada
         o Por conciliar.

         A continuación, se detallan los códigos de los estados de la Llave:

                                                        Tabla No.2

           CÓDIGO                       DESCRIPCIÓN                           FORMATO LÍMITE
             0001               Activa (empadronamiento)                          4 caracteres
             0002                          Anulada                                4 caracteres
             0003                       Por conciliar                             4 caracteres
             0004                        Suspendida                               4 caracteres


         5. Para el empadronamiento de las Llaves se utilizarán los siguientes códigos:

                                                        Tabla No.3

            CÓDIGO                  DESCRIPCIÓN                          FORMATO LÍMITE
               0001         Llave creada exitosamente                  4 caracteres numéricos
               0002         Rechazada: Llave ya existe                 4 caracteres numéricos
               0003            Rechazada: No cumple                      Hasta 4 caracteres
                                     parámetros
               0004          Rechazada: Error Técnico                  4 caracteres numéricos
               0005         Rechazada: Error por Time                  4 caracteres numéricos
                                          out

         6.  Para los procesos de solicitud de suspensión, anulación y habilitación, se deberá
         emplear los siguientes códigos:

                                                        Tabla No.4

            CÓDIGO                  DESCRIPCIÓN                          FORMATO LÍMITE
               0001                   Anulación                        4 caracteres numéricos
               0002                   Suspensión                       4 caracteres numéricos
               0003                  Habilitación                      4 caracteres numéricos

         7. Para la suspensión de las Llaves se utilizarán los siguientes códigos:

                                                        Tabla No.5

            CÓDIGO                  DESCRIPCIÓN                          FORMATO LÍMITE
               0001               Llave suspendida                     4 caracteres numéricos
                                    exitosamente
               0002            Rechazada: Llave ya se                  4 caracteres numéricos
                               encuentra suspendida

---

## Página 10

0003            Rechazada: No cumple                    4 caracteres numéricos
                                      parámetros
               0004          Rechazada: Error Técnico                  4 caracteres numéricos
               0005          Rechazada: Error por Time                 4 caracteres numéricos
                                           out
               0006          Rechazada: Llave no existe                4 caracteres numéricos

         8. Para la anulación de las Llaves se utilizarán los siguientes códigos:

                                                        Tabla No.6

             CÓDIGO                 DESCRIPCIÓN                           FORMATO LÍMITE
               0001         Llave anulada exitosamente                 4 caracteres numéricos
               0002            Rechazada: Llave ya se                  4 caracteres numéricos
                                 encuentra anulada
               0003            Rechazada: No cumple                      Hasta 4 caracteres
                                      parámetros
               0004          Rechazada: Error Técnico                  4 caracteres numéricos
               0005          Rechazada: Error por Time                 4 caracteres numéricos
                                           out
               0006          Rechazada: Llave no existe                4 caracteres numéricos

         9. Para la habilitación de las Llaves que se encontraban en estado Suspendidas, se
         utilizarán los siguientes códigos:

                                                        Tabla No.7

             CÓDIGO                 DESCRIPCIÓN                           FORMATO LÍMITE
               0001        Llave habilitada exitosamente               4 caracteres numéricos
               0002            Rechazada: Llave ya se                  4 caracteres numéricos
                                encuentra habilitada
               0003            Rechazada: No cumple                      Hasta 4 caracteres
                                      parámetros
               0004          Rechazada: Error Técnico                  4 caracteres numéricos
               0005          Rechazada: Error por Time                 4 caracteres numéricos
                                           out
               0006          Rechazada: Llave no existe                4 caracteres numéricos

         10. Previo a la anulación de Llaves, las Redes de Pago deben consultar al SIP el estado
         de la Llave.
         11. En la resolución de las Llaves; de la consulta recibida por parte de la RP Ordenante,
         se procede:
                   11.1 Verificar que la Llave solicitada se encuentre activa, previo a transferir la
                   consulta a la RP beneficiaria.
                   11.2  Si la  Llave   se encuentra anulada o no existe, el SIP             responde con esa
                   información a la RP Ordenante.
                   11.3 La RP Beneficiaria, remite la siguiente información para resolver una Llave:

---

## Página 11

Tabla No.8

                        DESCRIPCIÓN                       TIPO DE CAMPO             FORMATO LÍMITE

           Nombre del Cliente Beneficiario                       Texto                   Hasta 250
                                                                                         caracteres
           Número de identificación                           Numérico                    Hasta 13
                                                                                         caracteres
           Número    de    la    Cuenta    del                Numérico                    Hasta 35
           Beneficiario                                                                  caracteres
           Código del IR Beneficiario (BIC)                 Alfanumérico                  Hasta 11
                                                                                         caracteres
           Tipo de cuenta:                                    Numérico                  4 caracteres

           0010: Ahorros

           0011: Corriente

           0012: Básica

           0013: Electrónica

           0014: Tarjeta

     Nota: En caso de que la IR identifique que la cuenta beneficiaria del pago no esté activa, se deberá rechazar la
     resolución de esta Llave.

                   11.4 En el proceso de resolución de llaves, el SIP responderá con los siguientes
                   códigos:

                                                         Tabla 9

             CÓDIGO                  DESCRIPCIÓN                            FORMATO LÍMITE
               0001         Llave resuelta exitosamente                  4 caracteres numéricos
               0002          Rechazada: Llave no existe                  4 caracteres numéricos
               0003             Rechazada: No cumple                       Hasta 4 caracteres
                                       parámetros
               0004           Rechazada: Error Técnico                   4 caracteres numéricos
               0005          Rechazada: Error por Time                   4 caracteres numéricos
                                            out

---

## Página 12

4. PROCESO DE MIGRACIÓN               MASIVA      DE LLAVES DE UN INTEGRANTE HACIA OTRA RED DE
PAGOS.

Los  directorios  distribuidos  de las  diferentes  RP  deberán  mantenerse  sincronizados  con  el
Sistema Central de Llaves.

El proceso de migración masiva de llaves se realizará en base a los siguientes pasos:

     1.   El IR solicita al BCE la migración masiva de sus llaves, que se encuentran registradas en
          el Directorio Distribuido de una RP Origen.
     2.   El BCE debe notificar a la RP Origen y a la RP Destino, el “INICIO DE MIGRACIÓN”.
     3.   El administrador del SIP procede a reasignar todas las Llaves de la IR desde la RP origen
          a la RP Destino.
     4.   Las Llaves del IR quedan en estado “EN PROCESO DE MIGRACIÓN”. En esta etapa la IR
          no puede hacer pagos asociados a las Llaves, tampoco podrá gestionar las Llaves.
     5.   El SIP notifica la finalización de la migración a las RP Origen y Destino.
     6.   El SIP pone a disposición las Llaves a la RP Destino, con el propósito de que esta actualice
          su directorio distribuido.
     7.   El BCE debe notificar a la RP Origen y a la RP Destino, el “FIN DE MIGRACIÓN”.
     8.   La IR Origen y la RP Destino, previo a iniciar operación deben revisar                     el “Reporte de
          Conciliación”, para solucionar las discrepancias en caso de presentarse.
Ejemplo de inicio de migración masiva de llaves.

                                                     CUADRO 1.
                                      DIRECTORIO CENTRAL DE LLAVES


                DIRECTORIO                          DIRECTORIO                         DIRECTORIO
             DISTRIBUIDO RP1                    DISTRIBUIDO RP2                     DISTRIBUIDO RP3

                   BANCO 1                           BANCO 11                              COAC 1

                   BANCO 2                           BANCO 12                              COAC 2

                   BANCO 3                           BANCO 13                              COAC 3

                   BANCO 4                           BANCO 14                              COAC 4

                   BANCO 5                           BANCO 15

---

## Página 13

Ejemplo de migración masiva de llaves.

                                                       CUADRO 2.

                                        DIRECTORIO CENTRAL DE LLAVES


                 DIRECTORIO                           DIRECTORIO                           DIRECTORIO
             DISTRIBUIDO RP1                      DISTRIBUIDO RP2                      DISTRIBUIDO RP3

                                                       BANCO 11                               COAC 1

                   BANCO 2                             BANCO 12                               COAC 2

                   BANCO 3                             BANCO 13                               COAC 3

                   BANCO 4                             BANCO 14                               COAC 4

                   BANCO 5                             BANCO 15                              BANCO 1


La migración del Directorio de llaves de una IR en una RP origen, hacia otra RP Destino, deberá
realizarse fuera de horario de operaciones.

---

## Página 14

5. REGLAS PARA EL PROCESO DE LAS ÓRDENES DE PAGO

Respecto al proceso de pago, se definen las siguientes reglas:

    1.   Una vez resuelta la Llave, en los aplicativos (computador URL, dispositivo móvil APP) de
         los usuarios se deberá presentar el nombre del beneficiario. A continuación, se detallan
         los códigos de los canales para iniciar un pago:

                                                   Tabla No.10

                          CÓDIGO       DESCRIPCIÓN               FORMATO LÍMITE
                          0001         Web                       Hasta 140 caracteres
                          0002         APP                       Hasta 140 caracteres

    2.   Al usuario ordenante a través del App o web se le presentará el valor de la comisión a
         pagar incluidos impuestos.
    3.   Una vez resuelta la Llave, el cliente tiene hasta 60 segundos para confirmar el inicio del
         pago, superado ese tiempo, la IR requerirá al cliente reiniciar todo el proceso desde la
         resolución de la Llave.
    4.   El tiempo empleado desde el inicio hasta la confirmación del pago no puede superar 20
         segundos, superado este tiempo se rechaza el pago.
    5.   En el App o web del usuario ordenante se le presentará el estado del pago: Exitoso,
         Rechazado.
    6.   Los estados que una OP puede tener son:

                                                        Tabla No.11

 CÓDIGO             DESCRIPCIÓN                      FORMATO LÍMITE                    EXPLICACIÓN
      ACTC          Orden de Pago Validada                   4 caracteres              Recibido:     El  mensaje
                                                                                       pasó las validaciones de
                                                                                       esquema      y   firma.   El
                                                                                       pago está en cola.
      RJCT               Pago Rechazado                      4 caracteres              Rechazado: El     pago no
                                                                                       pudo     procesarse      (ej.
                                                                                       error  en   cuenta,   saldo
                                                                                       insuficiente en malla).
     PDNG               Pago por Conciliar                   4 caracteres              Diferido (Próximo Día):
                                                                                       El  pago    fue    recibido
                                                                                       después del Cut-off. Se
                                                                                       moverá    a  la  malla   del
                                                                                       siguiente día hábil.
      RCVD                Pago recibido                      4 caracteres              Aceptación Técnica: El
                                                                                       mensaje       ha       sido
                                                                                       recibido  por  el  nodo
                                                                                       destino (RP o SIP) y ha
                                                                                       superado                 las
                                                                                       validaciones básicas de
                                                                                       red   y   protocolo.   No

---

## Página 15

implica que el pago sea
                                                                                  válido funcionalmente,
                                                                                  solo que está en espera
                                                                                  de ser procesado por el
                                                                                  motor de pagos
     ACSP              Pago confirmado                    4 caracteres            En    Malla    (Hoy):    La
                                                                                  transacción       ya     es
                                                                                  irrevocable     y    forma
                                                                                  parte   de   la  malla   de
                                                                                  compensación actual.
     ACSC             Pago compensado                     4 caracteres            Liquidado: La     malla de
                                                                                  compensación se cerró
                                                                                  y   los   fondos    fueron
                                                                                  legalmente
                                                                                  transferidos.


    7.   Los conceptos por los que pueden ser rechazados las OP se describen a continuación:

                                                     Tabla No.12

   CÓDIGO                    DESCRIPCIÓN                   FORMATO LÍMITE               RESPONSABLE
                 0201 (PGO RECHAZADO, CUENTA                  4 caracteres              IR Beneficiario
     AC04        CERRADA)
                 0202 (PGO RECHAZADO, CUENTA                  4 caracteres              IR Beneficiario
     AC01        NO EXISTE)
                 0203 (PGO RECHAZADO, CUENTA                  4 caracteres              IR Beneficiario
     AC06        BLOQUEADA)
                 0204 (PGO RECHAZADO, ORDEN                   4 caracteres              IR Beneficiario
     AC05        BENEFICIARIO)
                 0205 (PGO RECHAZADO, POR NO                  4 caracteres                   Todos
     DT01        CONFIRMACIÓN TIME OUT)
                 0206 (PGO RECHAZADO, CUENTA                  4 caracteres              IR Beneficiario
     AC07        INACTIVA)
                 0207 (INSUFICIENCIA DE                       4 caracteres                     SIP
    AM04         FONDOS)
                 0208 (INFORMACIÓN                            4 caracteres                     SIP
    MS01         INCORRECTA)
    AM11         Tipo de moneda no valida                     4 caracteres                     SIP
     CH16        Contenido incorrecto                         4 caracteres                     SIP
     CH21        Elemento obligatorio ausente.                4 caracteres                     SIP
     DT02        Fecha no válida                              4 caracteres                     SIP
                 Número de transacciones no                   4 caracteres                     SIP
     DS0K        permitido
     CH20        Formato de monto incorrecto                  4 caracteres                     SIP
    AM01         Monto cero                                   4 caracteres                     SIP

---

## Página 16

La identificación del mensaje no                4 caracteres                      SIP
     DU01         es única
    AM02          Monto mayor al permitido                        4 caracteres                      SIP
     AC02         Número de cuenta inválido                       4 caracteres                      SIP
     TA01         Error de comunicación                           4 caracteres                      SIP
     FF10         Problemas técnicos                              4 caracteres                      SIP
     FF02         Sintaxis de mensaje incorrecto                  4 caracteres                      SIP

    8.   El Administrador del SIP deberá parametrizar: Fecha de inicio, Fecha de fin, Horario
         inicio, Horario fin; de operaciones, montos mínimos, montos máximos, por forma y tipo
         de inicio de pago.
    9.   El monto máximo para pagos realizados en horario no laborable se parametrizará por
         franjas de horario.
    10. La trazabilidad de los pagos se efectúa mediante el código de identificación único del
         pago; el código es generado por la IR Ordenante.
    11. Se deberán identificar los pagos iniciados con                 Llaves, para tal efecto el IR deberá
         informar en la mensajería del pago con los siguientes códigos:

                                                   Tabla No. 13

      CÓDIGO            DESCRIPCIÓN                                  FORMATO LÍMITE
      Key               Pago iniciado con Llave                      Hasta 140 caracteres
      Cred              Pago iniciado con credencial                 Hasta 140 caracteres
      QR                Pago iniciado con QR                         Hasta 140 caracteres

    12. Se parametrizará el cobro de comisiones.
    13. Cada etapa del pago tendrá un time out (parametrizables).
    14. Cada cambio/etapa en el SIP, se registrará en el “Registro de eventos”, para auditoría y
         trazabilidad.

---

## Página 17

6. SERVICIOS DE LAS REDES DE PAGOS

Las RP podrán prestar los siguientes servicios a los IR:

     a)   Inicio de pago mediante el uso de Llaves
     b)   Inicio de pago sin el uso de Llaves (credenciales)

Un IR podrá optar por la prestación de estos servicios de iniciación de pagos a sus clientes a
través de distintas redes; sin embargo, el mismo servicio no lo puede prestar en redes distintas.
Ejemplo:  Un IR podrá optar por dar el servicio con               Llaves, a través del servicio de la Red 1, y
prestar  el  servicio  sin    Llaves  a  través  de  la    Red  2.   Se  requiere  que  todos  los  integrantes
comuniquen al BCE la RP que utilizarían sin el uso de Llaves, esta información se debe almacenar
en el SIP con el propósito del ruteo de las órdenes de pagos. Para este efecto el BCE debe
implementar un proceso de empadronamiento obligatorio a fin de identificar la                         RP con la que
opera sin el uso de Llaves.
A través del proceso de resolución de Llaves descrito en este documento, cuando la RP ordenante
remite la mensajería de la OP al SIP, este rutea la instrucción hacia la RP beneficiaria que resolvió
la Llave.
Tratándose de pagos iniciados sin utilización de Llaves, una vez que el SIP recibe la OP de la RP
Ordenante, identifica la RP Beneficiaria a la que pertenece el IR Beneficiario conforme a la
información   registrada   en   el   empadronamiento,   y   rutea   la               OP    a   la  RP    Beneficiaria
correspondiente.


7. CÓDIGO ÚNICO DEL IDENTIFICADOR DE LA ORDEN DE PAGO.

El código único de identificado de las órdenes de pago es generado por la IR ordenante. Este
código es generado automáticamente por el sistema de las IR para cada operación y sirve como
referencia oficial para rastrear, conciliar y validar el pago.
Su función principal es la de garantizar que cada operación pueda ser distinguida de cualquier
otra, incluso si se repite el mismo monto, fecha o destinatario.

Este dato debe:

         Se incluye en los comprobantes que recibe el usuario.
         Permite a los IR y al SIP hacer conciliaciones.
         Sirve como referencia para el seguimiento de la orden de pago, así como para las
          auditorías y trazabilidad.

Su generación es automática, en el momento en que el sistema procesa la orden de pago y debe
cumplir con la estructura definida por el BCE.

---

## Página 18

Tabla No. 14

                BIC ORDENANTE                     FECHA Y HORA                NÚMERO SECUENCIAL
                                                  TRANSACCIÓN                          O HASH
                                                                                  CRIPTOGRÁFICO

                     PACIECEG                   27012026094735                        XYZ12345

            Código   BIC   asignado          Fecha: 27 de enero de           secuencial      o      hash
            por   el   BCE   a   la   IR     2026                            criptográfico
            Ordenante.                       Hora: 09:47:35
Nota: El código único de identificador de la orden de pago puede tener una extensión de 20 a 35 caracteres.

Consideraciones:

         No es editable: El usuario no puede modificar este código; se genera automáticamente.
         Obligatorio: Cada pago debe tenerlo, sin excepción.
         Trazabilidad y auditoría: Es el dato que se debe usar en seguimiento o conciliaciones de
          las órdenes de pago.
         Confidencialidad: Aunque aparece en comprobantes, no expone información sensible
          del usuario.

---

## Página 19

8. ESTÁNDAR DE MENSAJERÍA DE PAGOS

El estándar de mensajería en formato ISO 20022 contendrá los siguientes campos obligatorios:

                                                       Solicitud de Pago

                                                      Tabla No.15
                                                  Campos head.001.001.04

                   TAG                       MANDATORIO                          DESCRIPCIÓN
  AppHdr/Fr/FIId/FinInstnId/BICFI Obligatorio                      Código BIC del emisor del mensaje
                                         ISO20022
  AppHdr/To/FIId/FinInstnId/BICFI Obligatorio                      Código BIC del receptor del mensaje
                                         ISO20022
  AppHdr/BizMsgIdr                       Obligatorio               Identificador único del mensaje
                                         ISO20022
  AppHdr/MsgDefIdr                       Obligatorio               Identificador del tipo de mensaje
                                         ISO20022
  AppHdr/CreDt                           Obligatorio               Fecha y hora de creación del mensaje
                                         ISO20022
                                                                   Identificador de reintento
                                                                            True
  AppHdr/PssblDplct                      Obligatorio                        False
                                         ISO20022                       Ó
                                                                            0
                                                                            1


                                                      Tabla No.16
                                               Campos pacs.008.001.08

                           TAG                              MANDATORIO                  DESCRIPCIÓN
                    GrpHdr/MsgId                              Obligatorio          Identificador único del
                                                               ISO20022                    mensaje
                   GrpHdr/CreDtTm                             Obligatorio         Fecha y hora de la orden
                                                               ISO20022                     de pago
                   GrpHdr/NbOfTxs                             Obligatorio        Número de transacciones
                                                               ISO20022
             GrpHdr/SttlmInf/SttlmMtd                         Obligatorio          Método de liquidación.
                                                               ISO20022
              CdtTrfTxInf/PmtId/InstrId                       Obligatorio        Código identificador de la
                                                                   BCE                  orden de pago
          CdtTrfTxInf/PmtId/EndToEndId                        Obligatorio         Identificador de punto a
                                                               ISO20022                      punto
            CdtTrfTxInf/IntrBkSttlmAmt                        Obligatorio          Valor de transacción y
                                                               ISO20022              código de moneda

---

## Página 20

Responsabilidad de
                  CdtTrfTxInf/ChrgBr                            Obligatorio          comisión. Por defecto
                                                                 ISO20022           DEBT (Ordenante asume
                                                                                    costos de la transacción)
       CdtTrfTxInf/InstgAgt/FinInstnId/BICFI                    Obligatorio            Código BIC de la RP
                                                                    BCE                     Ordenante
       CdtTrfTxInf/InstdAgt/FinInstnId/BICFI                    Obligatorio            Código BIC de la RP
                                                                    BCE                     Beneficiaria
                 CdtTrfTxInf/Dbtr/Nm                            Obligatorio            Nombre del cliente
                                                                    BCE                     Ordenante
        CdtTrfTxInf/Dbtr/Id/PrvtId/Othr/Id                      Obligatorio         Identificación del cliente
                                                                    BCE                     Ordenante
 CdtTrfTxInf/Dbtr/Id/PrvtId/Othr/SchmeNm/Cd                     Obligatorio        Tipo de identificación del
                                                                    BCE                 cliente Ordenante
          CdtTrfTxInf/DbtrAcct/Id/Othr/Id                       Obligatorio          Número de cuenta del
                                                                    BCE                 cliente Ordenante
       CdtTrfTxInf/DbtrAgt/FinInstnId/BICFI                     Obligatorio             Código BIC del IR
                                                                 ISO20022                   Ordenante
       CdtTrfTxInf/CdtrAgt/FinInstnId/BICFI                     Obligatorio             Código BIC del IR
                                                                 ISO20022                   Beneficiario
                 CdtTrfTxInf/Cdtr/Nm                            Obligatorio            Nombre del cliente
                                                                    BCE                     Beneficiario
        CdtTrfTxInf/Cdtr/Id/PrvtId/Othr/Id                      Obligatorio         Identificación del cliente
                                                                    BCE                     Beneficiario
 CdtTrfTxInf/Cdtr/Id/PrvtId/Othr/SchmeNm/Cd                     Obligatorio        Tipo de identificación del
                                                                    BCE                cliente beneficiario
          CdtTrfTxInf/CdtrAcct/Id/Othr/Id                       Obligatorio          Número de cuenta del
                                                                    BCE                cliente Beneficiario
        CdtTrfTxInf/InstrForCdtrAgt/InstrInf                    Obligatorio               Canal de pago
                                                                    BCE
        CdtTrfTxInf/InstrForNxtAgt/InstrInf                     Obligatorio          Forma de iniciación del
                                                                    BCE                         pago
                CdtTrfTxInf/Purp/Prtry                          Obligatorio              Motivo del Pago
                                                                    BCE
             SplmtryData/Envlp/QrCode                         Opcional BCE           Información Código QR
                                                                                         (personalizado)
           SplmtryData/Envlp/TimeStamp                        Opcional BCE              Marca de tiempo
                                                                                         (personalizado)

---

## Página 21

Tabla No.17
                                                     Confirmación de Pago
                                                      Campos head.001.001.04

                     TAG                         MANDATORIO                        DESCRIPCIÓN
  AppHdr/Fr/FIId/FinInstnId/BICFI            Obligatorio              Código BIC del emisor del mensaje
                                             ISO20022
  AppHdr/To/FIId/FinInstnId/BICFI            Obligatorio              Código BIC del receptor del
                                             ISO20022                 mensaje
  AppHdr/BizMsgIdr                           Obligatorio              Identificador único del mensaje
                                             ISO20022
  AppHdr/MsgDefIdr                           Obligatorio              Identificador del tipo de mensaje
                                             ISO20022
  AppHdr/CreDt                               Obligatorio              Fecha y hora de creación del
                                             ISO20022                 mensaje


                                                       Tabla No.18
                                               Campos pacs.002.001.10

                        TAG                        MANDATORIO                     DESCRIPCIÓN
      AppHdr/Fr/FIId/FinInstnId/BICFI           Obligatorio            Bic Ordenante
                                                ISO20022
      AppHdr/To/FIId/FinInstnId/BICFI           Obligatorio            Bic de la institución respuesta
                                                ISO20022
      AppHdr/BizMsgIdr                          Obligatorio            Identificador único del
                                                ISO20022               mensaje
      AppHdr/MsgDefIdr                          Obligatorio            Identificador del tipo de
                                                ISO20022               mensaje
      AppHdr/CreDt                              Obligatorio            Fecha y hora de creación del
                                                ISO20022               mensaje
      GrpHdr/MsgId                              Obligatorio            Identificador único del
                                                ISO20022               mensaje
      GrpHdr/CreDtTm                            Obligatorio            Fecha y hora de la
                                                ISO20022               confirmación o notificación
      GrpHdr/InstgAgt/FinInstnId/BICFI          Obligatorio BCE        Código BIC del IR Beneficiario
      OrgnlGrpInfAndSts/OrgnlMsgId              Obligatorio BCE        Identificador único del
                                                                       mensaje de la orden de pago
      OrgnlGrpInfAndSts/OrgnlMsgNmId Obligatorio BCE                   Definición del mensaje de la
                                                                       OP
      TxInfAndSts/OrgnlInstrId                  Obligatorio BCE        Código identificador de la OP
      TxInfAndSts/OrgnlEndToEndId               Obligatorio BCE        Identificador de punto a punto
                                                                       de la OP
      TxInfAndSts/TxSts                         Obligatorio BCE        Respuesta de la OP
      TxInfAndSts/StsRsnInf/Rsn/Cd              Condicional            Código de rechazo. Obligatorio
                                                                       si TxSts es RJCT.

---

## Página 22

TxInfAndSts/StsRsnInf/AddtlInf              Condicional             Información de rechazo.
                                                                          Obligatorio si TxSts es RJCT.
      SplmtryData/Envlp/TimeStamp                 Opcional                Marca de tiempo
                                                                          (personalizado)



9. COMPENSACIÓN

Respecto a la compensación se definen las siguientes reglas:

     1.   El SIP mantendrá por cada IR, los compromisos generados de las OP inter-redes exitosas
          así como las irrevocables, que servirán para estructurar en tiempo real  las mallas de
          compensación bilaterales y multilaterales.
     2.   La  liquidación  de  las  mallas  de  compensación  se  procesa  de  manera  automática,
          parametrizable el horario. Adicionalmente el Administrador, de forma manual, podrá
          gestionar la liquidación de una malla de compensación                   por situaciones de riesgo de
          liquidez.
     3.   Permitir durante el día efectuar varias liquidaciones de los resultados de compensación
          neta multilateral.
     4.   Las OP exitosas e irrevocables ordenadas fuera de horario de operaciones, se liquidarán
          el primer día laborable siguiente.
     5.   Con base a la configuración en la parametrización en el cobro de comisión, el SIP deberá
          realizar los cálculos correspondientes.
     6.   En los horarios de liquidación de las cámaras de compensación especializadas, SPI, SCI,
          CCE y CCC, se priorizará la liquidación del SIP.
     7.   La liquidación se efectúa en las cuentas de los IR en el BCE.
     8.   Genera reportes de operación en función de las necesidades del Administrador del RPI.

---

## Página 23

10. RIESGOS DE LIQUIDEZ

Respecto al riesgo de liquidez se definen las siguientes reglas:

    1.   En el SIP operan Integrantes que mantengan saldos en cuentas corrientes en el Banco
         Central del Ecuador, a continuación, se presenta la estructura para el cálculo del riesgo
         de liquidez:

                                                  Tabla No.19

         CÓDIGO DEL             CÓDIGO DEL             PORCENTAJE              NIVEL DE           SALDO
      SECTOR DEL IR(1)               IR (2)          REQUERIMIENTO              RIESGO          MÍNIMO
                                                         ENCAJE (3)           ACEPTABLE        ACEPTABLE
                                                                                   (4)          (USD$) (5)
      Sector Financiero Privado y Público
                               Activos:
              0001             Superiores     a              5%                   25%             Cálculo
                               USD         1.000
                               millones
              0002             Menor o igual                 4%                   30%             Cálculo
                               a   USD   1.000
                               millones
      Sector Financiero Popular y Solidario
              0003             Cooperativas                  4%                   40%             Cálculo
                               de   Ahorro   y
                               Crédito
                               Segmento  1  y
                               Cajas
                               Centrales
              0004             Mutualistas de                4%                   40%             Cálculo
                               Ahorro            y
                               Crédito para la
                               Vivienda
              0005             Cooperativas                 2.5%                 100%             Cálculo
                               de   Ahorro   y
                               Crédito
                               Segmento 2
              0006             Cooperativas                  1%                  100%             Cálculo
                               de   Ahorro   y
                               Crédito
                               Segmento 3
              0007             Cooperativas
                               de   Ahorro   y
                               Crédito
                               Segmento 4
              0008             Cooperativas
                               de   Ahorro   y

---

## Página 24

Crédito
                               Segmento 5
      FINTECH (6)
              0009             SEDPES                        0,5%                 100%             Cálculo

         CAMPO 1.  Agrupa a entidades financieras de las mismas características y es asignado
         por el Administrador del SIP
         CAMPO 2. Corresponde al código BIC de la IR
         CAMPO 3. Porcentajes de encaje, se ajustarán conforme a la normativa de la JPRFM.
         CAMPO 4. Se definirán de conformidad con los análisis de riesgo de liquidez.
         CAMPO 5. Corresponde a la multiplicación del valor de encaje requerido, por el factor
         de riesgo.
         CAMPO 6. Para el caso de las Fintech, adicional al encaje establecido por la JPRFM, se
         añadirá un porcentaje en función de sus niveles de riesgo.

    A continuación, se presenta un ejemplo del cálculo del nivel de riesgo aceptable:


                                                  Tabla No. 20

  CÓDIGO DEL                                                                              NIVEL DE        SALDO
  SECTOR DEL          CÓDIGO DEL         CAPTACIONES           REQUERIMIENTO               RIESGO           CTA
         IR                 IR                                   ENCAJE (5%)            ACEPTABLE           BCE
                                                                                            (25%)
       0001            BIC XXXXX            11141.71                 557.09                139.27         576.72
Nota: El valor del riesgo aceptable se mantendrá durante 7 días y variará de acuerdo con el
cálculo de encaje en el sistema SBA (de jueves a miércoles).

    2.   El nivel de riesgo aceptable se calcula semanalmente y aplica para la siguiente semana.
    3.   Ningún IR puede enviar OP si ha comprometido su nivel de riesgo (semáforo rojo), para
         tal efecto el sistema no acepta nuevas OP del IR excluido del ecosistema. El SIP bloquea
         las  OP  enviadas  por  el  IR  Ordenante  cuando  éste  ha  superado  el  nivel  de  riesgo
         aceptable.
    4.   La deshabilitación se notifica al IR Ordenante mediante un PACS002 en respuesta al
         PACS008 de intención de pago. Si el pago se origina con Llave, la notificación se realizará
         mediante un mensaje ACK pacs002. (notificación de recibido)
    5.   Cuando una IR supere el nivel de riesgo aceptable y/o el saldo de la cuenta corriente el
         administrador del Sistema SIP bloqueará la cuenta corriente en el BCE contra débitos de
         manera manual.
    6.   La  deshabilitación  del  IR  en  el      SIP  por  factores  de  riesgo  se  realizará  de  manera
         automática, mientras que la habilitación de la IR, la realizará el Administrador del
         Sistema; posterior al análisis y autorizaciones que correspondan y será de                        manera
         manual.
    7.   El sistema deberá controlar los umbrales de riesgo de cada IR por OP considerando: el
         nivel de riesgo     aceptable, el saldo de la cuenta corriente (este se comparará en el
         Sistema SLA cada 30 minutos; dato parametrizable) así como el valor comprometido
         (sumatoria de las OPs ordenadas)

---

## Página 25

8.   El SIP presenta a la RP y a la IR mediante un dashboard, cuando la IR                 se aproxima a sus
         umbrales de riesgo, con base a una semaforización parametrizable, ejemplo: 100%
         (Color rojo), 90% al 99% (color naranja), 75% al 89% (color amarillo), y menos del 75%
         (color verde).

                                                        Tabla No.21

                                                            VALOR DE LAS ÓRDENES DE PAGOS ENVIADAS
                                                                     PENDIENTES DE LIQUIDAR USD
   CÓDIGO                           SALDO CTA
     DEL           CÓDIGO               BCE                                NARANJA         AMARILLO            VERDE
   SECTOR           DEL IR              USD                ROJO
    DEL IR

     0001         BIC XXXXX           576.72              139.27             125.34           104.45           103.06


    9.   El dashboard para el monitoreo de riesgos presentará la siguiente información:
         9.1       Órdenes de pago enviadas: Valor y número;
         9.2       Posición neta: Valor;
         9.3       Saldo en cuenta corriente, especificando la fecha y hora de la información:
                   Ejemplo: Saldo al 16/06/2025, 14h30 (captura información del SLA);
         9.4       Requerimiento de encaje (captura información del SBA);
         9.5       Nivel de riesgo aceptable;
         9.6       Saldo mínimo aceptable;
    10. El dashboard presentará la información en tabla, semaforización, y gráficos.
    11. Solo los IR y el Administrador del SIP pueden visualizar su saldo en cuenta corriente,
         requerimiento de encaje, posición neta de obligaciones respecto del requerimiento de
         encaje (semaforizado).

    Reglas Generales de Riesgo de Liquidez

    1.   El SIP debe segregar la información que se otorga a los diferentes usuarios, dependiendo
         del  tipo  de  perfil:  i)  Administrador,  ii)  Operador  iii)  Consulta  BCE;  además  de  su
         modificación o eliminación de perfiles; manteniendo el historial de los perfiles.
    2.   El SIP emitirá una alerta cuándo una RP o una IR se encuentren desconectadas.

---

## Página 26

11. FLUJOS DEL SISTEMA INTEGRADOR DE PAGOS

11.1 Llaves

Creación de las Llaves: acmt.998.001.bce01.in (Empadronamiento de Llave)
                                 acmt.998.002.bce01.out (Respuesta a empadronamiento)

Para la creación de Llaves, seguirá la siguiente secuencia:

     1.   El Cliente solicita el registro o creación de la Llave; esto a través de los canales que defina
          el IR, a su vez el IR valida la identidad del Cliente y obtiene su consentimiento para poder
          registrar la información en su propio Directorio;
     2.   La IR envía solicitud de crear la Llave a la RP;
     3.   La RP reserva la Llave y envía la solicitud de crear la Llave al SIP;
     4.   El SIP verifica la existencia de la Llave. Si la Llave “existe” notifica a la RP “Llave ya existe”,
          éste a su vez le notifica a la IR; si la Llave “no existe”                  el SIP notifica a la RP        “Llave
          disponible”; y a la vez el SIP reserva y registra la información que le corresponde para la
          creación de la Llave;
     5.   La RP registra la Llave; y autoriza el registro de la Llave a la IR;
     6.   La IR registra la Llave e informa al Cliente el registro exitoso de la Llave.

Nota: Si la RP tuvo un error de conexión o timeout al enviar la solicitud de crear llave al SIP, se
     realizarán los reintentos parametrizados. En caso de que no exista conexión entre las partes,
     se rechaza la creación de la llave y no se reserva la misma.

                           Gráfico 1. Esquema de registro o creación de la Llave






















Fuente: SASP
Elaboración: SASP

---

## Página 27

Anulación de la Llave: acmt.998.003.bce01.in (solicitud)
                               acmt.998.004.bce01.out (respuesta)

Para la anulación de Llaves, seguirá la siguiente secuencia:


     1.   El Cliente solicita la anulación de la Llave al IR
     2.   El IR verifica la existencia de la Llave y envía solicitud de anular Llave al RP;
     3.   La RP verifica la existencia de la Llave y anula la Llave; y a su vez comunica al SIP la
          anulación de la misma;
     4.   El SIP verifica la existencia de la Llave y anula, y a la vez notifica a la RP la anulación de
          la Llave;
     5.   La RP envía notificación de “Llave anulada” a la IR;
     6.   El IR informa al Cliente respecto de la anulación exitosa de la Llave.

Notas:
     1.   En el caso de portabilidad, el cliente debe anular la llave en su IR actual, para posterior
          registro de la llave en la nueva IR.
     2.   Si la RP tuvo un error de conexión o timeout al enviar solicitud de anulación de llave al
          SIP, se realizará los reintentos parametrizados. En caso de que no exista conexión entre
          las partes, se rechaza la anulación de la llave.

                                 Gráfico 2.  Esquema de anulación de la Llave



























Fuente: SASP
Elaboración: SASP

---

## Página 28

Suspensión de la Llave: acmt.998.003.bce01.in (solicitud)
                              acmt.998.004.bce01.out (respuesta)

     El SIP para la suspensión de Llaves, seguirá la siguiente secuencia

     1.   El IR suspende la Llave y comunica la solicitud de suspensión de la Llave a su RP;
     2.   La RP registra la intención de suspensión de la Llave y notifica al SIP la solicitud de
          suspensión de la misma;
     3.   El SIP suspende la Llave e informa a la RP;
     4.   La RP suspende la Llave e informa al IR;
     5.   El IR informa al Cliente respecto de la suspensión de la Llave por algún evento inusual.

Nota: Una llave a la que se le haya registrado con intención de suspensión, no podrá realizar
ninguna operación.

La suspensión de una llave es responsabilidad de la IR.


                               Gráfico 3. Esquema de Suspensión de la Llave































Fuente: SASP

Elaboración: SASP

---

## Página 29

Habilitación de Llave Suspendida: acmt.998.003.bce01.in (solicitud)
                                               acmt.998.004.bce01.out (respuesta)

Para la habilitación de Llaves, seguirá la siguiente secuencia:

1. El IR solicita habilitar la Llave suspendida a la RP;
2. La RP solicita habilitar la Llave suspendida al SIP;
3. El SIP habilita la Llave e informa a la RP;
4. La RP habilita la Llave e informa al IR;
5. El IR informa al Cliente respecto de la habilitación de la Llave.

Nota: Solo se puede habilitar una llave que previamente tenía el estado de “Suspendida”
La habilitación de una llave suspendida es responsabilidad de la IR.


                                Gráfico 4. Esquema de Habilitación de la Llave


























Fuente: SASP
Elaboración: SASP

---

## Página 30

Resolución de Llaves para pagos Inter Redes: acmt.998.005.bce01.in(consulta)
                                                                     acmt.998.006.bce01.out (respuesta)


Para la resolución de Llaves de pagos inter - redes, seguirá la siguiente secuencia:


     1.   El cliente ordenante envía solicitud para consultar llave a la IR Ordenante
     2.   El IR ordenante consulta a su RP la existencia y estado de la llave del Cliente
          beneficiario
     3.   La RP ordenante consulta la Llave al SIP a fin de identificar su existencia;
     4.   El SIP revisa el estado de la Llave y si ésta se encuentra activa canaliza la consulta de
          credenciales de pago a la RP beneficiaria a la que pertenece la Llave consultada. En el
          caso que la llave esté anulada, suspendida o no exista, devuelve la consulta a la RP
          ordenante y éste a su vez al IR, como rechazada.
     5.   El RP beneficiaria canaliza la consulta de la llave a la IR beneficiaria
     6.   El IR beneficiaria, resuelve la Llave, y remite los siguientes datos a la RP Beneficiaria:

               a) Número de documento de identificación del Cliente beneficiario
               b) Nombre del Cliente beneficiario
              c) Número de la Cuenta del beneficiario
               d) Estado de la cuenta
               e) Tipo de cuenta
                f) BIC de la IR Beneficiaria

      7.      La RP beneficiaria remite los datos del Cliente Beneficiario al SIP
      8.      El SIP remite los datos del Cliente Beneficiario a la RP ordenante.
      9.      La RP ordenante remite los datos del Cliente Beneficiario a la IR ordenante.
      10.   La IR ordenante presenta al Cliente ordenante los datos del Cliente Beneficiario en su
      aplicativo, para que este confirme el pago.

      Nota: Los pagos iniciados con códigos QR, se resolverá la credencial siguiendo los pasos
      del presente flujo.

---

## Página 31

Gráfico 5. Esquema de Resolución de Llaves para pagos Inter Redes
























Fuente: SASP
Elaboración: SASP

11.2 Credenciales de Pagos

Validación de credenciales para pagos Inter Redes acmt.998.005.bce.o1.in (Consulta)
                                                                       acmt.998.006.bce.o1.out (Respuesta)

Para la validación de credenciales para pagos inter - redes, seguirá la siguiente secuencia:

     1)   El  Cliente Ordenante requiere iniciar el pago con credencial, por lo que procede a
          registrar las credenciales del Cliente beneficiario en el aplicativo de su IR;
     2)   El IR ordenante solicita a su RP la consulta de la validación del cliente beneficiario
     3)   La RP ordenante canaliza la consulta al SIP sobre la validación del cliente beneficiario.
     4)   El  SIP  remite    a  la   RP   correspondiente,  la  validación  del  cliente  beneficiario,  de
          conformidad con la información suministrada por el IR en la creación. En el caso que la
          credencial no exista se devuelve un mensaje “credencial no existe” a la RP ordenante.
     5)   La RP beneficiaria canaliza la consulta al IR Beneficiario sobre la validación del cliente
          beneficiario.
     6)   El IR Beneficiario valida la credencial y esta informa a la RP Beneficiario sobre la
          información de la credencial consultada, informando que la misma, está habilitada para
          recibir recursos, en caso de no estar habilitada la IR beneficiaria deberá rechazar la
          validación.
     7)   La RP Beneficiario canaliza la respuesta al SIP, sobre la información de la credencial
          consultada
     8)   El SIP remite la respuesta a la RP ordenante.
     9)   La RP ordenante remite la respuesta de la consulta a la IR ordenante.

---

## Página 32

10) La IR ordenante muestra el resultado de la validación al Cliente Ordenante, dándole la
          opción de guardar este contacto para futuras transacciones; si la validación resulta
          errónea, podría ingresar nuevamente los datos corregidos del Cliente Beneficiario.


                     Gráfico 6. Validación de credenciales para pagos Inter Redes
























Fuente: SASP
Elaboración: SASP


11.3 Órdenes de Pago Inter Redes y sus Excepciones

Orden de Pago: pacs.008.004.08 BCE in (IR Ordenante – RP Ordenante/ RP Ordenante – SIP)
Orden de Pago: pacs.008.004.08 BCE out (SIP – RP Beneficiaria/ RP Beneficiaria – IR
Beneficiaria)

Confirmación de la Orden de Pago/Notificación: pacs.002.001.10 BCE Conf
Rechazo de la Orden de Pago/Notificación:          pacs.002.001.10 BCE Rechazo


El SIP para canalizar las órdenes de pago inte-redes, seguirá la siguiente secuencia:

     1.   El Cliente Ordenante confirma el pago mediante el aplicativo del IR Ordenante
     2.   El IR ordenante recibe la OP, debita y reserva los fondos; a su vez el IR ordenante remite
          la OP a su RP.
     3.   La RP valida la estructura de la OP y remite la misma al SIP.
     4.   El SIP valida la estructura de la OP y registra el pago. En caso de no cumplir la validación
          de la OP, o supere los límites de riesgo se rechaza la transacción.
     5.   El SIP remite la OP a la RP beneficiaria;

---

## Página 33

6.   La RP beneficiaria recibe y remite la OP al IR beneficiario.
     7.   El IR beneficiaria valida la OP. El IR beneficiaria acredita los fondos en la cuenta del cliente
          beneficiario, y a su vez notifica al Cliente beneficiario la OP exitosa; en caso de no cumplir
          la validación o reglas de la OP o se rechaza la transacción
     8.   El IR beneficiario envía la confirmación exitosa de la OP a la RP beneficiaria
     9.   La RP beneficiaria envía la confirmación exitosa de la OP al SIP.
     10. El SIP recibe y valida la confirmación de la OP exitosa, y genera las mallas multilaterales
          con la OP confirmada, y a su vez envía la confirmación exitosa de la OP a la RP ordenante.
     11. La RP ordenante envía la confirmación de la OP exitosa al IR ordenante
     12. El IR ordenante libera los fondos de la cuenta del cliente ordenante y notifica al Cliente
          ordenante de su OP exitosa.

Nota: Las OP son consideradas irrevocables una vez que el SIP haya notificado y la RP beneficiaria
haya recibido dicha OP; en el caso de no tener respuesta por la RP beneficiaria, las OP entran en
un proceso de conciliación; si la OP es rechazada, ésta será resuelta por las partes fuera del
sistema.
La liquidación es ejecutada en base a la malla multilateral; sin embargo, el sistema mantendrá a
nivel de reporte la malla bilateral.


                            Gráfico 7. Esquema de Orden de Pago– Inter Redes




























Fuente: SASP

Elaboración: SASP

---

## Página 34

Excepción 1. Error de conexión de la red de pagos (ordenante).
pacs.002.001.10 BCE Rechazo (Respuesta de la RP Ordenante al IR Ordenante)

La RP Ordenante recibe el pacs0008 desde el IR Ordenante, pero no puede comunicar esta
situación al SIP:


     1.   El Cliente Ordenante confirma el pago mediante el aplicativo del IR Ordenante.
     2.   El IR ordenante recibe la OP y debita los fondos, a su vez el IR ordenante remite la OP a
          su RP.
     3.   La RP ordenante presenta un error de conexión al enviar la solicitud de pago al SIP, y
          realiza varios reintentos de conexión al SIP dentro del time out establecido; sin conexión
          exitosa.
     4.   El IR Ordenante, una vez superado el time out, comunica al cliente Ordenante que la OP
          está “en proceso”
     5.   Una vez restablecida la RP Ordenante, comunica a la IR que la OP fue rechazada.
     6.   El IR ordenante devuelve los fondos al Cliente Ordenante y le notifica que la OP fue
          rechazada.

Nota: el número de reintentos dependerá de la RP ordenante, sin superar el time out.


             Gráfico 8: Excepción 1. Error de conexión de la Red de pagos (ordenante).























Fuente: SASP
Elaboración: SASP
Nota: El número de reintentos se realizarán dentro del time out definido para cada tramo.

---

## Página 35

Excepción 2. Error de conexión del SIP al envío de la OP a la Red de Pagos (beneficiario)
pacs.002.001.10 BCE Rechazo. (del SIP a la RP Ordenante)

El SIP recibe el pacs0008 desde el RP Ordenante, pero no puede comunicar esta situación a la RP
Beneficiaria:

     5.   El SIP presenta un error de conexión al enviar la OP a la RP beneficiaria, y realiza varios
          reintentos de conexión con la RP beneficiaria dentro del time out establecido; sin
          conexión exitosa.
     6.   La RP Ordenante al no recibir respuesta del SIP y una vez superado el time out, marca la
          OP como “OP en proceso”; y notifica al IR ordenante que la OP se encuentra en proceso.
     7.   El IR Ordenante notifica al cliente que la OP se encuentra en proceso a fin de que no
          realice reintentos de la OP.
     8.   Una vez que se restablezca la conexión, el SIP envía el estado de la OP rechazada a la RP
          ordenante.
     9.   La RP ordenante envía el estado de OP rechazada a la IR ordenante.
     10. El IR ordenante devuelve los fondos al Cliente Ordenante y le notifica que la OP fue
          rechazada.


      Gráfico 9. Excepción 2: Error de conexión del SIP al envío de la OP a la Red de Pagos
                                                   (beneficiario)






















Fuente: SASP

Elaboración: SASP
Nota: El número de reintentos se realizarán dentro del timeout definido para cada tramo

---

## Página 36

Excepción 3. Pago no confirmado por la Red de Pagos (beneficiario)
pacs.002.001.10 BCE Rechazo

La RP Beneficiaria recibe el pacs0002 desde la IR beneficiaria, pero no puede comunicar esta
situación al SIP:

     9.   La RP beneficiaria recibe el estado de la OP pero presenta un error de conexión al enviar
          el estado de     la OP al SIP y realiza varios reintentos de conexión, dentro del time out
          establecido; sin conexión exitosa.
     10. El SIP al no recibir respuesta de la RP Beneficiaria y una vez superado el time out, genera
          la malla multilateral y marca la OP “por conciliar”.
     11. El SIP notifica a la RP Ordenante la OP “por conciliar”.
     12. La RP Ordenante notifica al IR Ordenante la OP “por conciliar”.
     13. El IR Ordenante notifica al Cliente Ordenante como “pago en proceso” a fin de que no
          realice reintentos de la OP.
     14. Al momento que la RP Beneficiaria logra reconectarse, notifica al SIP la confirmación
          exitosa o rechazada de la OP.
     15. El SIP envía la confirmación de la OP exitosa o rechazada a la RP ordenante.
     16. La RP ordenante envía la confirmación de la OP exitosa o rechazada al IR ordenante
     17. En el caso que la OP sea exitosa el IR ordenante libera los fondos de la cuenta del cliente
          ordenante y notifica al Cliente ordenante de su OP exitosa.


         Gráfico 10. Excepción 3. Pago no confirmado por la Red de Pagos (beneficiario)























Fuente: SASP
Elaboración: SASP
Nota: El número de reintentos se realizarán dentro del time out definido para cada tramo

---

## Página 37

Excepción 4. Error técnico del SIP a la Red de pagos (ordenante)
pacs.002.001.10 BCE Rechazo (Flujo)

El SIP recibe el pacs0002 desde la RP beneficiaria o la OP fue declarada irrevocable, pero no
puede comunicar esta situación a la RP Ordenante:

       10. El SIP recibe y valida la confirmación de la OP exitosa o rechazada, y genera la malla
             multilateral con la OP confirmada, al momento de enviar la confirmación exitosa o
             rechazada de la OP a la RP ordenante; presenta un error de conexión.
       11.  El SIP realiza varios reintentos dentro del time out establecido, para comunicar a la
             RP Ordenante
       12. La RP ordenante superado el time out comunica al IR ordenante “pago en proceso”
       13. El IR ordenante comunica al cliente ordenante “pago en proceso”
       14.  Reestablecidas las comunicaciones entre el SIP con la RP ordenante, informa el estado
             final del pago (aceptada o rechazada).
       15. La RP ordenante comunica al IR ordenante el estado final del pago (aceptada o
             rechazada)
       16. El IR ordenante comunica al cliente el estado del pago; en caso de ser un pago
             rechazado libera los fondos.


           Gráfico 11. Excepción 4. Error técnico del SIP a la Red de pagos (ordenante)




























Fuente: SASP

Elaboración: SASP
Nota: El número de reintentos se realizarán dentro del time out definido para cada tramo.

---

## Página 38

11.4. Resumen de excepciones en el proceso de órdenes de pago

 Exc        Origen         Descripción           Estado         Descripción         Irrevocabilidad          Estado
   .         Error            Técnica              ISO                                                        Malla
                                              (pacs.002)
   1     RP                Error al            RJCT (tras      Rechazado: El                No                  No
         Ordenante         enviar             reintentos       pago no pudo                                 generada
         a SIP             pacs.008 al              )          procesarse.
                           SIP.
   2     SIP a RP          SIP no                 RJCT         Rechazado: El                No                  No
         Beneficiari       puede                               pago no pudo                                 generada
         a                 entregar la                         procesarse.
                           OP a la RP
                           beneficiaria.
   3     RP Benef. a       Error al               ACSP         En Malla                      Sí                Por
         SIP (vuelta)      devolver                            (Hoy): La                                    conciliar
                           confirmació                         transacción
                           n de                                ya es
                           acreditación                        irrevocable y
                           al SIP.                             forma parte
                                                               de la malla de
                                                               compensació
                                                               n actual.
   4     SIP a RP          Error al               ACSC         En Malla                      Sí            Generada
         Ordenante         enviar                              (Hoy): La
         (vuelta)          confirmació                         transacción
                           n final al                          ya es
                           origen.                             irrevocable y
                                                               forma parte
                                                               de la malla de
                                                               compensació
                                                               n actual.

---

## Página 39

12.REPORTES GENERALES PARA EL ADMINISTRADOR DEL SISTEMA DE LLAVES

12.1 Gestión de llaves del SIP

Para una correcta gestión del administrador del sistema, como para los operadores de este, se
deberá contar con los siguientes reportes:


                                                                                Fecha: 29/12/2025 Hora: 12:02
                                                                                       Generado por: Luis Pérez
                                                                                    Identificación: 1720841414
                                        REPORTE RESUMEN DE LLAVES

         TOTAL                    LLAVES                 LLAVES                 LLAVES              LLAVES POR
                                 ACTIVAS            SUSPENDIDAS               ANULADAS               CONCILIAR
 TOTAL                              206                     50                     87                      8
Nota1: Esta información deberá ser descargada en formato Excel y PDF.
Nota2: Se podrá realizar búsqueda de información mediante uno o varios filtros.
Nota3: Los reportes deberán presentar la opción de filtro por cada variable.
Nota4: Los reportes podrán ser filtrados por una fecha específica o por un periodo.

                                                                                Fecha: 29/12/2025 Hora: 12:02
                                                                                       Generado por: Luis Pérez
                                                                                    Identificación: 1720841414
                                         REPORTE DETALLE DE LLAVES

  REDES DE PAGO              LLAVES              LLAVES              LLAVES            LLAVES POR          LLAVES
          BIC               ACTIVAS         SUSPENDIDAS           ANULADAS             CONCILIAR          TOTALES
 RPI (RPIEQ)                    52                  5                   10                   1                68
 BANRED                         30                 10                    5                   1                46
 (BANREDCEQ)
 CHAS (CHASCUE)                 24                 15                    8                   1                48
 DEUNA                         100                 20                   64                   5               189
 (DUNAEQ)
 TOTAL                         206                 50                   87                   8               351
Nota1: Esta información únicamente estará disponible para el administrador del SIP, para cada RP, se
deberá presentar los datos que le corresponden.
Nota 2: Esta información deberá ser descargada en formato Excel y PDF
Nota 3: Esta información podrá desplegarse a detalle, según la necesidad del Administrador.
Nota 4: Se podrá realizar búsqueda de información mediante uno o varios filtros.
Nota 5: Los reportes deberán presentar la opción de filtro por cada variable.
Nota 6: Los reportes podrán ser filtrados por una fecha específica o por un periodo.

---

## Página 40

Fecha: 29/12/2025 Hora: 12:02
                                                                                       Generado por: Luis Pérez
                                                                                    Identificación: 1720841414

                                           REPORTE TIPO DE LLAVES

 NRO                       TIPO DE                VALOR                BIC DEL            BIC DE LA         ESTAD
    .      CÓDIG            LLAVE                                    INTEGRANT             RED DE              O
             O DE                                                          E               PAGOS
            LLAVE
    1        0001           Celular            096577658             PRODECEQ            RPI (RPIEQ)         0001
    2        0002       Identificació          172044366               PACICEG             BANRED            0002
                               n                                                       (BANREDCEQ
                                                                                               )
    3        0003           Correo          jjparedes@bce.e            PACICEG              CHAS             0001
                         electrónico                  c                                  (CHASCUE)
    4        0001           Celular           0987547600             PRODECEQ              DEUNA             0003
                                                                                         (DUNAEQ)
Nota1: Esta información únicamente estará disponible para el administrador del SIP, para cada Red de
Pago, se deberá presentar los datos que le corresponden.
Nota 2: Esta información deberá ser descargada en formato Excel y PDF
Nota 3: Esta información podrá desplegarse a detalle, según la necesidad del Administrador.
Nota 4: Se podrá realizar búsqueda de información mediante uno o varios filtros.
Nota 5: Los reportes deberán presentar la opción de filtro por cada variable.
Nota 6: Los reportes podrán ser filtrados por una fecha específica o por un periodo.

Esta información le permitirá a cada Red de Pago ir monitoreando la Gestión de Llaves, por su
parte el cuadro presentado previamente podrá ser desplegado para verificación de información
a detalle.

                                                                               Fecha: 29/12/2025 Hora: 12:02
                                                                                       Generado por: Luis Pérez
                                                                                    Identificación: 1720841414
                                             CATÁLOGO DE LLAVES

    NRO.                        TIPO DE                 VALOR              ESTADO              ACCIONES
              CÓDIGO             LLAVE                                                   EDITAR / ELIMINAR
                  DE
                LLAVE
       1        0001             Celular             096577658               0001
       2        0002        Identificación           172044366               0002
       3        0003             Correo          jjparedes@bce.ec            0001
                              electrónico
       4        0001             Celular            0987547600               0003
Nota 1: Esta información únicamente estará disponible para el administrador del SIP, para cada Red de
Pago, se deberá presentar los datos que le corresponden.
Nota 2: Esta información deberá ser descargada en formato Excel y PDF.
Nota 3: Esta información podrá desplegarse a detalle, según la necesidad del Administrador.
Nota 4: Se podrá realizar búsqueda de información mediante uno o varios filtros.

---

## Página 41

Nota 5: Los reportes deberán presentar la opción de filtro por cada variable.
              Nota 6: Los reportes podrán ser filtrados por una fecha específica o por un periodo.


                                                                                               Fecha: 29/12/2025 Hora: 12:02
                                                                                                       Generado por: Luis Pérez
                                                                                                    Identificación: 1720841414

                                                                TIPO DE LLAVES

                                 NRO.           CÓDIGO DE             TIPO DE LLAVE                  ESTADO
                                                   LLAVE
                                    1              0001                    Celular                 Habilitado
                                    2              0002                Identificación              Habilitado
                                    3              0003             Correo electrónico             Habilitado

                                    4              0004                Alfanumérico                Habilitado
                                                          Total:                                         4


                                                                                               Fecha: 29/12/2025 Hora: 12:02
                                                                                                       Generado por: Luis Pérez
                                                                                                    Identificación: 1720841414
                                                             REPORTE DE LLAVES

NR       CÓDIGO        TIPO DE LLAVE        VALOR            BIC DEL          BIC DE LA RED       ESTADO          FECHA               FECHA DE
O.      DE LLAVE                                           INTEGRANTE           DE PAGOS                        REGISTRO Y       ACTUALIZACIÓN DE
                                                           ORDENANTE                                               HORA           ESTADO Y HORA

1        0001             Celular         0965776         PRODECEQ            RPI (RPIEQ)          0001        29/12/202           30/12/2025
                                              58                                                                     5                 12:15
                                                                                                                  12:15
2        0002         Identificación      1720443           PACICEG             BANRED             0002        29/12/202           30/12/2025
                                              66                             (BANREDCEQ                              5                 12:18
                                                                                     )                            12:18
3        0003             Correo          jjparedes         PACICEG               CHAS             0001        29/12/202           31/12/2025
                       electrónico        @bce.ec                             (CHASCUE)                              5                 12:20
                                                                                                                  12:20
4        0001             Celular         0987547         PRODECEQ               DEUNA             0003        29/12/202           31/12/2025
                                              600                              (DUNAEQ)                              5                 12:21
                                                                                                                  12:21
              Nota1: Esta información únicamente estará disponible para el administrador del SIP, para cada Red de
              Pago, se deberá presentar los datos que le corresponden.
              Nota 2: Esta información deberá ser descargada en formato Excel y PDF
              Nota 3: Esta información podrá desplegarse a detalle, según la necesidad del Administrador.
              Nota 4: La fecha de registro hace referencia a la primera vez.
              Nota 5: Se podrá realizar búsqueda de información mediante uno o varios filtros.
              Nota 6: Los reportes deberán presentar la opción de filtro por cada variable.
              Nota 7: Los reportes podrán ser filtrados por una fecha específica o por un periodo.

---

## Página 42

Fecha: 29/12/2025 Hora: 12:02
                                                                                      Generado por: Luis Pérez
                                                                                   Identificación: 1720841414
                                     REPORTE DE HISTORIAL DE LLAVES
                         0996122125
Valor llave:

    NR       CÓDIGO          BIC DEL        BIC DE LA RED       ESTADO          FECHA             FECHA DE
     O.     DE LLAVE      INTEGRANTE          DE PAGOS                       REGISTRO Y      ACTUALIZACIÓN DE
                          ORDENANTE                                             HORA           ESTADO Y HORA


     1        0001        PRODECEQ          RPI (RPIEQ)          0001        29/12/20           30/12/2025
                                                                                  25               12:15
                                                                                12:15
     2        0001          PACICEG           BANRED             0002        29/12/20           30/12/2025
                                            (BANREDCE                             25               12:18
                                                  Q)                            12:18
     3        0001          PACICEG             CHAS             0001        29/12/20           31/12/2025
                                            (CHASCUE)                             25               12:20
                                                                                12:20
     4        0001        PRODECEQ             DEUNA             0003        29/12/20           31/12/2025
                                             (DUNAEQ)                             25               12:21
                                                                                12:21
                                           TOTAL                                                      4

Nota 1: Esta información únicamente estará disponible para el administrador del SIP, para cada Red de
Pago, se deberá presentar los datos que le corresponden.
Nota 2: Esta información deberá ser descargada en formato Excel y PDF
Nota 3: Esta información podrá desplegarse a detalle, según la necesidad del Administrador.
Nota 4: La fecha de registro hace referencia a la primera vez.
Nota 5: Se podrá realizar búsqueda de información mediante uno o varios filtros.
Nota 6: Los reportes deberán presentar la opción de filtro por cada variable.
Nota 7: Los reportes podrán ser filtrados por una fecha específica o por un periodo.

12.2 Reporte del proceso de conciliación de llaves
                                                                               Fecha: 29/12/2025 Hora: 12:02
                                                                                      Generado por: Luis Pérez
                                                                                   Identificación: 1720841414
                                     GESTIÓN DE LLAVES POR DETALLE
RED DE PAGOS: DeUna (DUNAEQ)
Fecha: 15 de septiembre de 2025
  NRO.        TIPO DE LLAVE                      LLAVE                  INTEGRANTE                  ESTADO
    1       Número de cédula                 1720841414                   BASAECEG             POR CONCILIAR
    2       Número de celular                0996122125                   BBOLECEG             POR CONCILIAR
    3       Correo electrónico        jjparedes@hotmail.com                LOJBECEL            POR CONCILIAR
    4       Número de cédula                 1720853796                   PRCBECEQ             POR CONCILIAR
    5       Número de celular                0998123567                   PRCBECEQ             POR CONCILIAR
Nota 1: Las conciliaciones de las Llaves deberán realizarse de manera diaria, por lo cual hasta las 6 pm
(horario parametrizado), la RP deberá asignar el estado final al total de llaves en estado por conciliar.

---

## Página 43

Fecha: 29/12/2025 Hora: 12:02
                                                                                Generado por: Luis Pérez
                                                                              Identificación: 1720841414


                              GESTIÓN DE LLAVES POR DETALLE DE RED

RED DE PAGOS: DeUna (DUNAEQ)
Fecha: 15 de septiembre de 2025
  NRO.          TIPO DE LLAVE                       LLAVE                 INTEGRANTE           ESTADO
                                                                                                 FINAL
    1         Número de cédula                  1720841414                  BASAECEG            Creada
    2         Número de celular                 0996122125                  BBOLECEG            Creada
    3         Correo electrónico          jjparedes@hotmail.com             LOJBECEL           Anulada
    4         Número de cédula                  1720853796                  PRCBECEQ            Creada
    5         Número de celular                 0998123567                  PRCBECEQ           Anulada

Esta información, actualizará adicionalmente el reporte de Gestión de Llaves.


                                                                         Fecha: 29/12/2025 Hora: 12:02
                                                                                Generado por: Luis Pérez
                                                                              Identificación: 1720841414

                                           GESTIÓN DE LLAVES

        REDES DE PAGO                LLAVES CREADAS           LLAVES ANULADAS             LLAVES POR
                                                                                           CONCILIAR
 RPI (RPIEQ)                                  52                       11                        0
 BANRED (BANREDCEQ)                           31                        5                        0
 CHAS (CHASCUE)                               25                        8                        0
 DEUNA (DUNAEQ)                              103                       66                        0
 TOTAL                                       211                       90                        0

---

## Página 44

Fecha: 29/12/2025 Hora: 12:02
                                                                                                            Generado por: Luis Pérez
                                                                                                         Identificación: 1720841414

                                               CONCILIACIÓN DE LLAVES ENTRE DIRECTORIOS


            DIRECTORIO CENTRAL DE LLAVES                                                  RED DE PAGOS XXX
       VALOR LLAVE                INTEGRANTE          ESTADO               VALOR LLAVE                INTEGRANTE            ESTADO         OBSERVACIÓN

1720841414                        BASAECEG           Creada        1720841414                         BASAECEG           Suspendida        Estado       no
                                                                                                                                           coincide
0996122125                        BBOLECEG           Creada        0996122125                         BASAECEG           Creada            IR               no
                                                                                                                                           coincide
jjparedes@hotmail.com             LOJBECEL           Anulada       jjparedes@hotmail.com              LOJBECEL           Suspendida        Estado       no
                                                                                                                                           coincide
                Nota 1: Los Directorios Distribuidos se sincronizarán de manera automática con el Directorio Central de
                Llaves; por lo que este reporte reflejará la información registrada en el Directorio Central de Llaves versus
                la Información registrada en los Directorios Distribuidos de las RP; al existir discrepancias; el Directorio
                Central de Llaves deberá actualizar su información conforme lo presentado por los Directorios Distribuidos
                de manera automática.
                Nota 2: Si la RP contiene una Llave y el Directorio Central de Llaves no lo tiene; el DCLL tiene que actualizar
                esta información;
                Nota 3: El proceso de conciliación de llaves lo debe iniciar cada Red de Pagos.
                Nota 4: Solo se puede solucionar discrepancias en los estados de las Llaves entre el DCLL y las RP (no en
                llaves creadas previamente)
                Nota 5: Para solucionar discrepancias en creación de llaves; el DCLL deberá revisar el flujo de creación de
                Llaves.
                Nota 6: El reporte presentará solo información de las llaves por conciliar.


                12.3 Reglas de conciliación de llaves

                1.   El SIP no tiene registrada la llave, pero la Red de Pagos si tiene registrada la llave: El SIP debe
                     actualizar la llave y el estado.
                2.   La RP tiene registrada la llave, y el SIP también, pero con estados diferentes de la llave: El
                     SIP actualiza el estado.
                3.   La RP1 no tiene registrado la llave, pero el SIP tiene registrada la llave en la RP2: La RP1 no
                     puede gestionar dicha llave.
                4.   La RP1 tiene registrada la llave, el SIP tiene registrada esa llave en la RP2: La RP1 debe
                     ajustarse a la información del SIP, eliminando la llave del directorio distribuido1.

---

## Página 45

12.4 Resumen de reglas de conciliación de llaves

 Nro.          Escenario de                Origen del Conflicto             Acción Definida          Estado Final del
              Inconsistencia                                                                            Directorio
   1      Llave nueva en RP              Desfase de registro               El SIP actualiza            Sincronizado
          no en SIP                      inicial                           su base.                      (SIP = RP)
   2      Estado diferente               Falla en flujo de                 El SIP actualiza el         Sincronizado
          (SIP vs RP)                    suspensión/habilitación           estado.                       (SIP = RP)
   3      Conflicto de                   Intento de portabilidad           La RP debe                    Única (SIP
          Pertenencia (RP1 vs            fallida o error de carga          borrar la llave                manda)
          RP2)                                                             localmente
                                                                           según el SIP.
   4      Anulación fallida en           Error de comunicación             Conciliación               Pendiente de
          SIP                            (Excepción de                     dentro del                    cierre día
                                         anulación)                        mismo día.

---

## Página 46

13. REPORTES GENERALES PARA EL ADMINISTRADOR DEL SISTEMA (PAGOS)

  El SIP presentará reportes con la frecuencia establecida, con el propósito de                    administrar la
  información de los pagos.


                                                                              Fecha: 29/12/2025 Hora: 12:02
                                                                                     Generado por: Luis Pérez
                                                                                   Identificación: 1720841414
                                         GESTIÓN DE PAGOS TOTALES

  Fecha: 15 de septiembre de 2025
        REDES DE PAGO                 OP EXITOSAS              OP RECHAZADAS              OP POR CONCILIAR
   RPI (RPIEQ)                             1000                        10                           1
   BANRED (BANREDCEQ)                       200                        25                           1
   CHAS (CHASCUE)                           100                        12                           1
   DEUNA (DUNAEQ)                          5000                        100                          5
   TOTAL                                   6300                        147                          8
  Nota 1: Esta información únicamente estará disponible para el administrador del SIP, para cada Red de
  Pago, se deberá presentar los datos que le corresponden.
  Esta información le permitirá a cada Red de Pago ir monitoreando la Gestión de           Pagos, por su parte el
  cuadro presentado previamente podrá ser desplegado para verificación de información a detalla.

  13.1 Reporte del proceso de conciliación de pagos

                                                                              Fecha: 29/12/2025 Hora: 12:02
                                                                                     Generado por: Luis Pérez
                                                                                   Identificación: 1720841414

                                      GESTIÓN DE PAGOS POR DETALLE
                                               OP POR CONCILIAR

  RED DE PAGOS ORDENANTE: DeUna (DUNAEQ)
  Fecha: 15 de septiembre de 2025
NRO.      METODO DE            ID PAGO        INTEGRANTE             RP            INTEGRANTE            ESTADO
           INICIACIÓN                         ORDENANTE        BENEFICIARIA      BENEFICIARIOS
 1          LLAVES          PICHECEQ00         PICHECEQ        BANREDCEQ           BASAECEG               POR
                                00001                                                                 CONCILIAR
 2       CREDENCIAL         GUAYECE000         GUAYECE          CHASCUE            BBOLECEG               POR
                                00023                                                                 CONCILIAR
 3            QR            AUSTECEQ0         AUSTECEQ           COONEQ            LOJBECEL               POR
                              0000048                                                                 CONCILIAR
 4          LLAVES           AUSTECEQ         AUSTECEQ             RPIEQ           PRCBECEQ               POR
                              00000022                                                                CONCILIAR
 5       CREDENCIAL         PICHECEQ00         PICHECEQ        BANREDCEQ           PRCBECEQ               POR
                                00003                                                                 CONCILIAR
  Nota 1: Las conciliaciones de los pagos deberán realizarse de manera diaria, por lo cual hasta las 6 pm
  (horario parametrizado), la RP deberá asignar el estado final al total de las OP en estado por conciliar.

---

## Página 47

Fecha: 29/12/2025 Hora: 12:02
                                                                                Generado por: Luis Pérez
                                                                              Identificación: 1720841414

                                     GESTIÓN DE PAGOS POR DETALLE
                                           PAGOS CONCILIADOS

   RED DE PAGOS ORDENANTE: DeUna (DUNAEQ)
   Fecha: 15 de septiembre de 2025
NR     METODO DE           ID PAGO         INTEGRANTE              RP            INTEGRANTE           ESTADO
O.      INICIACIÓN                         ORDENANTE         BENEFICIARIA       BENEFICIARIOS
1         LLAVES         PICHECEQ00         PICHECEQ          BANREDCEQ            BASAECEG              OP
                             00001                                                                    EXITOSO
2      CREDENCIAL        GUAYECE000          GUAYECE           CHASCUE             BBOLECEG              OP
                             00023                                                                   RECHAZA
                                                                                                         DO
3            QR           AUSTECEQ0         AUSTECEQ            COONEQ              LOJBECEL             OP
                           0000048                                                                   RECHAZA
                                                                                                         DO
4         LLAVES          AUSTECEQ          AUSTECEQ             RPIEQ             PRCBECEQ              OP
                           00000022                                                                   EXITOSO
5      CREDENCIAL        PICHECEQ00         PICHECEQ          BANREDCEQ            PRCBECEQ              OP
                             00003                                                                    EXITOSO

   Esta información, actualizará adicionalmente el reporte de Gestión de Pagos.

                                                                         Fecha: 29/12/2025 Hora: 12:02
                                                                                Generado por: Luis Pérez
                                                                              Identificación: 1720841414


                                            GESTIÓN DE PAGOS
   Fecha: 15 de septiembre de 2025
         REDES DE PAGO               OP EXITOSAS           OP RECHAZADAS            OP POR CONCILIAR
    RPI (RPIEQ)                          1000                      11                         0
    BANRED (BANREDCEQ)                    200                      26                         0
    CHAS (CHASCUE)                        101                      12                         0
    DEUNA (DUNAEQ)                       5003                     102                         0
    TOTAL                                6304                     151                         0

---

## Página 48

13.2 Reglas de conciliación de pagos

     1)   Los  pagos  que  por alguna  razón presenten alguna novedad  en su procesamiento,
          contaran con la etiqueta “POR CONCILIAR”, esto está descrito en los flujos funcionales.
     2)   El proceso de conciliación será gestionado por las Redes de Pagos, para esto el SIP
          proporcionará la información necesaria.
     3)   Es preciso que el Sistema Integrador de Pagos y las Redes de Pagos se encuentren
          sincronizados, el objetivo es de mantener la información actualizada.

---

## Página 49

ANEXO 1.

CÓDIGOS PARA CONFORMACIÓN DE CAMPO ALFANUMÉRICO

Códigos para anteponer al número celular y convertir a estos en una Llave Alfanumérica:


                Entidades con activos mayores a USD 1.500 millones de dólares.

                     NRO.          CÓDIGO                   INTEGRANTE (IR)
                        1              10                 BANCO  PICHINCHA
                        2              30                BANCO DEL PACIFICO
                        3              17                 BANCO  GUAYAQUIL
                        4              36               BANCO PRODUBANCO
                        5              37                BANCO BOLIVARIANO
                        6              32              BANCO INTERNACIONAL
                        7              35                 BANCO DEL AUSTRO
                        8             126                    BANCO DINERS
                                                  COOP. JUVENTUD ECUATORIANA
                        9             107                    PROGRESISTA
                                                 COOP. AHORRO Y CREDITO JARDIN
                       10             100                       AZUAYO
                                                    COOP DE AHORRO Y CREDITO
                       11             124              ALIANZA DEL VALLE LTDA
Nota: Con la incorporación de nuevos integrantes, se actualizará la presente información.

---

## Página 50

ANEXO 2.

LISTA DE CÓDIGOS BIC

    1.  Redes de Pago

  ITEM     #CTA CTE              ENTIDAD                    RUC             CÓDIGO BIC
                                                                              NUEVO

                               BANRED (RED
    1       1770025         INTERBANCARIA DE          991325026001          BANDECEQ
                         CAJEROS AUTOMÁTICOS)
    2       1770015           COONECTA RED            1792063574001         COONECEQ
                             TRANSACCIONAL
    3       1770028             ETIKO´s S.A.          195137684001           ETIKECEC
    4                         DEUNA (BANCO                                  DEUNECEQ
                                PICHINCHA)

    2.  Integrantes obligados



   ITEM      #CTA CTE          ENTIDAD                 RUC           CÓDIGO BIC        CÓDIGO BIC
                                                                    ACTUAL SWIFT          NUEVO

1
             1600022     BANCO PICHINCHA        1790010937001       PICHECEQ500       PICHECEQ500
2            2600187     BANCO DEL              990005737001        PACIECEG          PACIECEG
                         PACIFICO
3            2600021     BANCO                  990049459001        GUAYECEGNOS       GUAYECEGNOS
                         GUAYAQUIL
4            1600303     BANCO                  1790368718001       PRODECEQ          PRODECEQ
                         PRODUBANCO
5            2600344     BANCO                  990379017001        BBOLECEG          BBOLECEG
                         BOLIVARIANO
6            1600212     BANCO                  1790098354001       BINTECEQ          BINTECEQ
                         INTERNACIONAL
7            3600186     BANCO AUSTRO           190055965001        AUSTECEQ          AUSTECEQ
8            1700319     BANCO DINERS           1790283380001       DICDECEQ          DICDECEQ
                         CLUB
                         COOP. JUVENTUD
9            3700113     ECUATORIANA            190115798001        CJEPECEQ          CJEPECEQ
                         PROGRESISTA

---

## Página 51

COOP. AHORRO Y
10           3700114      CREDITO JARDIN        190155722001         CCJAECE1          CAJAECEC
                          AZUAYO
                          COOP DE AHORRO
11           1700435      Y CREDITO             1790501469001        CAEZECE1          CADVECEQ
                          ALIANZA DEL
                          VALLE LTDA.
