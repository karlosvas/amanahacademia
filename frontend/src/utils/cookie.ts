export function getTheme(): string {
  const theme = document.cookie
    .split("; ")
    .find((row) => row.startsWith("theme="))
    ?.split("=")[1];

  return theme || "light";
}
