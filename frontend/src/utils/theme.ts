import { writeThemeCookie } from "./cookie";

export function applyThemeClass(value: string) {
  // aseguramos que la clase está en html (documentElement) y no exista la opuesta
  document.documentElement.classList.remove("dark", "light");
  document.documentElement.classList.add(value);
}

export function applyTheme(newTheme: string) {
  const html = document.documentElement;
  if (newTheme !== "dark" && newTheme !== "light") return;
  html.classList.remove("dark", "light");
  html.classList.add(newTheme);

  // Actualizamos la cookie (centralizado)
  writeThemeCookie(newTheme);

  // Actualizar logo si procede
  const logo = document.getElementById("logo_amanah");
  if (logo && logo instanceof HTMLImageElement) {
    logo.src = newTheme === "dark" ? "/img/logo_amanah_dark.webp" : "/img/logo_amanah.webp";
  }
}
