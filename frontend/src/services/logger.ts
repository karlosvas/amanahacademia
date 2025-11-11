import adze, { setup } from "adze";

// Configuración global
setup({
  activeLevel: import.meta.env.PROD ? "error" : "debug",
  withEmoji: true,
  showTimestamp: true,
});

// Creamos un logger base
const logger = adze.withEmoji.timestamp;

// Función para enviar logs al endpoint (solo errores)
function sendErrorToEndpoint(entry: any) {
  // Validar que entry existe y tiene render
  // if (!entry?.render) {
  //   console.error("Entry inválida para logging:", entry);
  //   return;
  // }
  // const logData = entry.render?.json();
  //console.log(logData);
  // Cambia "level" por "levelName"
  // if (logData?.levelName === "error") {
  //   fetch("/api/log", {
  //     method: "POST",
  //     headers: { "Content-Type": "application/json" },
  //     body: JSON.stringify(entry.render.json()),
  //   }).catch(() => {
  //     console.error("Error enviando log al servidor");
  //   });
  // }
}

// Exportamos un wrapper para loggear y enviar errores automáticamente
export const log = {
  info: (...args: Parameters<typeof logger.info>) => logger.info(...args),
  debug: (...args: Parameters<typeof logger.debug>) => logger.debug(...args),
  warn: (...args: Parameters<typeof logger.warn>) => logger.warn(...args),
  error: (...args: Parameters<typeof logger.error>) => {
    const entry = logger.error(...args);
    sendErrorToEndpoint(entry);
  },
};
