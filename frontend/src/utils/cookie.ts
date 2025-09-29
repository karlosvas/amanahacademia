export function getThemeFromCookie() {
  if (typeof document === "undefined" || !document.cookie) return "light";
  const cookies = document.cookie.split(";").map((c) => c.trim());
  for (const c of cookies) {
    if (c.startsWith("theme=")) {
      return c.substring("theme=".length);
    }
  }
  return "light";
}

export function getLangFromCookie(): string {
  const match = document.cookie.match(/(?:^|; )lang=([^;]*)/);
  return match ? match[1] : "es";
}

function getCarryFromCookies() {
  const carryCookie = document.cookie.split(";").find((cookie) => cookie.trim().startsWith("carry="));
  if (carryCookie) {
    const carryValue = carryCookie.split("=")[1];
    try {
      return JSON.parse(decodeURIComponent(carryValue));
    } catch (error) {
      console.error("Error parsing carry cookie:", error);
      return null;
    }
  }
  return null;
}
