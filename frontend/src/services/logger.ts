import adze, { setup } from "adze";

// Configuración global
setup({
  activeLevel: import.meta.env.PROD ? "error" : "debug",
  withEmoji: true,
  showTimestamp: true,
});

// Creamos un logger base
const logger = adze.withEmoji.timestamp;
const isDevelopment = import.meta.env.DEV;

/**
 * Sanitiza un error para logging seguro en producción
 */
function sanitizeError(error: unknown): string {
  if (!(error instanceof Error)) {
    return String(error);
  }

  if (isDevelopment) {
    // En desarrollo: retornar el error completo para que adze lo formatee
    return error.stack || error.message;
  }

  // En producción: solo tipo y mensaje, SIN stack trace
  return `${error.name}: ${error.message}`;
}

/**
 * Procesa argumentos para sanitizar errores en producción
 */
function processLogArgs(args: unknown[]): any[] {
  if (isDevelopment) {
    return args; // En desarrollo: pasar sin cambios
  }

  // En producción: sanitizar errores
  return args.map((arg) => {
    if (arg instanceof Error) {
      return sanitizeError(arg);
    }
    return arg;
  });
}

// Exportamos un wrapper para loggear de forma segura
export const log = {
  info: (...args: any[]) => {
    const processed = processLogArgs(args);
    return (logger.info as any)(...processed);
  },

  debug: (...args: any[]) => {
    const processed = processLogArgs(args);
    return (logger.debug as any)(...processed);
  },

  warn: (...args: any[]) => {
    const processed = processLogArgs(args);
    return (logger.warn as any)(...processed);
  },

  error: (...args: any[]) => {
    const processed = processLogArgs(args);
    return (logger.error as any)(...processed);
  },
};
