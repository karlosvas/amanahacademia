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

export function acceptCookies() {
  if (typeof window.gtag !== "undefined") {
    window.gtag("consent", "update", {
      analytics_storage: "granted",
    });
  }
  localStorage.setItem("cookieConsent", "accepted");
  hideBanner();
}

export function rejectCookies() {
  localStorage.setItem("cookieConsent", "rejected");
  hideBanner();
}

function hideBanner() {
  const banner = document.getElementById("cookie-banner");
  if (banner) {
    banner.style.animation = "slide-down 0.3s ease-out";
    setTimeout(() => {
      banner.classList.add("hidden");
    }, 300);
  }
}
