# Participantes de un pago inter-redes

**Pregunta:** ¿Quién se comunica con quién en un pago entre dos redes?

**Canal representado:** mensajes. Las flechas no representan movimiento de dinero.

**Estado:** aprobado el 18-08-2026 para el resumen ejecutivo institucional.

```mermaid
sequenceDiagram
    actor P as Quien paga
    participant IO as Integrante ordenante
    participant RO as Red ordenante
    participant SIP as Sistema central (SIP)
    participant RB as Red beneficiaria
    participant IB as Integrante beneficiario
    actor B as Quien cobra

    P->>IO: Confirma el pago
    IO->>RO: Orden de pago
    RO->>SIP: Orden inter-redes
    SIP->>RB: Orden para su integrante
    RB->>IB: Orden de abono
    IB->>B: Acredita la cuenta

    IB-->>RB: Resultado del abono
    RB-->>SIP: Confirmación
    SIP-->>RO: Estado final
    RO-->>IO: Estado final
    IO-->>P: Muestra el resultado

    Note over IO,IB: Los integrantes no se conectan directamente con el SIP
```

## Qué debe validar la revisión

- Si este nivel de detalle explica la topología sin entrar todavía en irrevocabilidad.
- Si conviene mostrar a las personas o comenzar y terminar en los integrantes.
- Si el pago dentro de una sola red merece un diagrama separado.
