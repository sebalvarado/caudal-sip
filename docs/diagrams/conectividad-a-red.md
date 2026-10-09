# Red propia y oportunidad de conectividad

**Pregunta:** ¿Cuál es el camino principal y cuándo considerar conectividad hacia una red existente?

**Canal representado:** mensajes. Las flechas no representan movimiento ni custodia de dinero.

**Estado:** aprobado el 18-08-2026 para el resumen ejecutivo institucional.

```mermaid
flowchart TB
    subgraph P["Camino principal · Red de pagos propia"]
        direction LR
        I["Integrantes<br/>de nuestra red"]
        R["Nuestra red de pagos<br/>gateway · directorio · certificación · enrutamiento"]
        S["SIP · BCE"]

        I -->|"API sencilla"| R
        R <-->|"mensajes inter-redes"| S
    end

    subgraph O["Oportunidad opcional · Solo si aparece una contraparte"]
        direction LR
        D{"Autorización clara<br/>certificación · reutilización<br/>sin exclusividad · sin retraso"}
        E["Institución afiliada<br/>a otra red"]
        C["Conector Caudal<br/>sin exclusividad"]
        X["Red existente"]

        D -.-> E
        E -.-> C
        C -.-> X
    end
```

## Qué debe validar la revisión

- Si la red propia aparece inequívocamente como el camino principal.
- Si las responsabilidades resumidas dentro de la red son suficientes para un lector no técnico.
- Si la conectividad hacia otra red se entiende como oportunidad opcional, sin sugerir una transición de clientes ni una dependencia.
