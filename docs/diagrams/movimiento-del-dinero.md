# Qué cambia cuando una persona paga

**Pregunta:** ¿Qué cambia cuando una persona paga y cuándo ocurre cada cambio?

**Canales representados:** cuentas de clientes, obligaciones entre instituciones y liquidación en dinero del Banco Central. Cada canal ocupa una franja separada.

**Estado:** borrador para revisión.

```mermaid
flowchart TB
    subgraph CLIENTES["1. Cuentas de clientes — cada integrante lleva su propio libro"]
        direction LR
        RESERVA["Reserva en la cuenta de quien paga"]
        DEBITO["Débito definitivo al ordenante"]
        CREDITO["Crédito a quien cobra"]
        RESERVA -->|"confirmación final"| DEBITO
    end

    subgraph OBLIGACIONES["2. Obligaciones entre instituciones — libro del SIP"]
        direction LR
        CUPO["Reserva de cupo del integrante"]
        ACSP["Obligación irrevocable · ACSP"]
        MALLA["Posición incluida en la malla"]
        CUPO -->|"la red beneficiaria recibe la orden"| ACSP
        ACSP -->|"se acumula"| MALLA
    end

    subgraph BCE["3. Dinero del Banco Central — cuentas de los integrantes en el BCE"]
        direction LR
        INSTRUCCION["Instrucción neta de liquidación"]
        ACSC["Débitos y créditos en el BCE · ACSC"]
        INSTRUCCION -->|"se ejecuta"| ACSC
    end

    RESERVA -. "la orden entra al SIP" .-> CUPO
    ACSP -. "permite acreditar" .-> CREDITO
    CREDITO -. "respuesta del beneficiario" .-> DEBITO
    MALLA -->|"cierra el ciclo"| INSTRUCCION
    ACSC -. "libera el cupo comprometido" .-> CUPO
```

## Qué debe validar la revisión

- Si “débito definitivo” describe correctamente lo que el integrante hace con los fondos reservados al recibir la confirmación.
- Si la acreditación al beneficiario debe aparecer antes o después de `ACSP` en la explicación editorial.
- Si mostrar el retorno de cupo desde `ACSC` hace el diagrama más claro o introduce un ciclo difícil de leer.
