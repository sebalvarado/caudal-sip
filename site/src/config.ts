// Single place to change the project's name and identity.
export const SITE = {
  // Codename of the movement/project. Caudal = a river's flow — and, in
  // Spanish, wealth/funds ("caudales"). Water that moves shapes the world.
  name: 'Caudal',
  fullName: 'Proyecto Caudal',
  // The regulated standard this project implements
  standard: 'RPI EC',
  tagline: 'La implementación abierta de los pagos instantáneos del Ecuador',
  description:
    'Proyecto open source que construye la infraestructura de pagos en tiempo real del Ecuador: interoperable, universal, segura y gratuita para las personas.',
  // Comes from `site` in astro.config.mjs — change it there, not here.
  url: import.meta.env.SITE,
  github: 'https://github.com/sebalvarado/caudal-sip',
  locale: 'es',
  // The live demo network (infra/azure). Empty until it is deployed; the demo
  // page then explains how to run the same network locally.
  demoApi: (import.meta.env.PUBLIC_DEMO_API ?? '').replace(/\/$/, ''),
} as const;
