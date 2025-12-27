import adze, { setup } from "adze";

// Configuración global
setup({
  activeLevel: import.meta.env.PROD ? "error" : "debug",
  withEmoji: true,
  showTimestamp: true,
});

// Creamos un logger base
const logger = adze.withEmoji.timestamp;

// Exportamos un wrapper para loggear y enviar errores automáticamente
export const log = {
  info: (...args: Parameters<typeof logger.info>) => logger.info(...args),
  debug: (...args: Parameters<typeof logger.debug>) => logger.debug(...args),
  warn: (...args: Parameters<typeof logger.warn>) => logger.warn(...args),
  error: (...args: Parameters<typeof logger.error>) => logger.error(...args),
};
