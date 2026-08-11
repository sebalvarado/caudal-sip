import type { APIRoute } from 'astro';
import { SITE } from '../config';

const url = (path: string) => new URL(path, SITE.url).href;

export const GET: APIRoute = () =>
  new Response(
    `# ${SITE.fullName}

> Proyecto open source (Apache-2.0) que construye la implementación abierta del
> estándar ecuatoriano de pagos en tiempo real (RPI/SIP, ISO 20022, llaves de
> pago, QR interoperable). Independiente, no afiliado al Banco Central del Ecuador.
> "Caudal": el flujo de un río — y, en español, los fondos de una economía.

## Páginas principales

- [El estándar, explicado](${url('/estandar/')}): referencia abierta del Catálogo Operacional del SIP (BCE, v1.0, marzo 2026). Estados de una orden de pago (RCVD, ACTC, ACSP, ACSC, RJCT, PDNG), códigos de rechazo, tipos y ciclo de vida de las llaves de pago, tiempos máximos (60 s para confirmar, 20 s de extremo a extremo), mensajería ISO 20022 (head.001.001.04, pacs.008.001.08, pacs.002.001.10, acmt.998.*), compensación neta multilateral y el motor de riesgo de liquidez. Cada afirmación cita la sección del catálogo original.
- [Manifiesto](${url('/manifiesto/')}): por qué Ecuador necesita pagos instantáneos, universales y gratuitos, y por qué el software debe ser abierto.
- [Blog](${url('/blog/')}): bitácora pública de la construcción del proyecto.

## Contexto

Ecuador reguló los pagos instantáneos interoperables entre 2024 y 2026
(Resoluciones JPRM-2024-018-M, JPRM-2024-029-M, BCE-GG-008-2025 y BCE-GG-007-2026).
El estándar exige mensajería ISO 20022 (pacs.008, pacs.002, acmt), llaves de pago
y códigos QR, con fecha límite del 30 de noviembre de 2026 para pagos intra-redes.
Este proyecto construye las librerías, el conector de integrante, el simulador
de red y el kit de certificación para que cualquier institución pueda cumplir el
estándar. Código fuente: ${SITE.github}
`,
    { headers: { 'Content-Type': 'text/plain; charset=utf-8' } },
  );
