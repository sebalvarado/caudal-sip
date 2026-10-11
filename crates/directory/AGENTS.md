# caudal-directory

Owns llaves (format per Tabla 1), credenciales, the central key system (llave to red) and each red's distributed directory (llave to credencial), and the masked name.

Never depends on anything but `caudal-types` and `caudal-registry`, or on anything that does I/O.

- A resolution outside the owning red returns only a `MaskedName`; the masking rule is the [proposal] default of ADR-0014 until the directory privacy RFC.
- Registration is single-phase for the seeded sandbox. The two-phase reservation with the central system is not built yet.
