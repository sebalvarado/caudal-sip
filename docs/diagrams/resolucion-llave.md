# Resolución de una llave

**Pregunta:** ¿Cómo se convierte una llave que una persona recuerda en una cuenta que puede confirmar?

**Canal representado:** mensajes de consulta y respuesta. No se mueve dinero durante este flujo.

**Estado:** borrador para revisión.

```mermaid
sequenceDiagram
    actor P as Quien paga
    participant IO as Integrante ordenante
    participant RO as Red ordenante
    participant SCL as Sistema Central de Llaves
    participant DB as Directorio de la red beneficiaria

    P->>IO: Ingresa celular, cédula, correo o alias
    IO->>RO: Solicita resolver la llave
    RO->>SCL: Consulta nacional
    SCL->>DB: Consulta la credencial en la red propietaria
    DB-->>SCL: Institución, cuenta, titular y estado
    SCL-->>RO: Credencial resuelta
    RO-->>IO: Credencial resuelta
    IO-->>P: Muestra el nombre para confirmar

    Note over SCL: Garantiza que la llave sea única y sabe qué red la administra
    Note over DB: Conserva la credencial asociada a la llave
```

## Qué debe validar la revisión

- Si el Sistema Central de Llaves consulta directamente el directorio distribuido o si debemos mostrar a la red beneficiaria como participante separado.
- Qué datos necesita ver el lector y cuáles pertenecen únicamente a la especificación técnica.
- Si registro y resolución de llaves deben explicarse en diagramas distintos.
