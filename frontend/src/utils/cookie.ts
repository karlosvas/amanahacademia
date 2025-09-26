export function getTheme(): string {
  const theme = document.cookie
    .split("; ")
    .find((row) => row.startsWith("theme="))
    ?.split("=")[1];

  return theme || "light";
}

export function getLangFromCookie(): string {
  const match = document.cookie.match(/(?:^|; )lang=([^;]*)/);
  return match ? match[1] : "es";
}
