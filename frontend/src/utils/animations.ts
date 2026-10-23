import { gsap } from "gsap";
import { ScrollTrigger } from "gsap/ScrollTrigger";

gsap.registerPlugin(ScrollTrigger);

function findFadeIn(line: HTMLElement): HTMLElement | null {
  const ancestor = line.closest<HTMLElement>(".scroll-fade-in");
  if (ancestor) return ancestor;

  const previous = line.previousElementSibling;
  return previous instanceof HTMLElement &&
    previous.classList.contains("scroll-fade-in")
    ? previous
    : null;
}

function waitForFadeIn(element: HTMLElement): Promise<void> {
  return new Promise((resolve) => {
    const onTransitionEnd = (event: TransitionEvent) => {
      if (event.target !== element || event.propertyName !== "opacity") return;
      element.removeEventListener("transitionend", onTransitionEnd);
      resolve();
    };
    element.addEventListener("transitionend", onTransitionEnd);
  });
}

function expand(line: HTMLElement, scrollTrigger?: ScrollTrigger.Vars) {
  gsap.to(line, {
    scaleX: 1,
    transformOrigin: "center",
    duration: 0.8,
    ease: "power3.out",
    scrollTrigger,
  });
}

export function animateTitleLines() {
  gsap.matchMedia().add("(prefers-reduced-motion: no-preference)", () => {
    gsap.utils.toArray<HTMLElement>(".title-line").forEach((line) => {
      const fadeIn = findFadeIn(line);
      if (fadeIn) {
        waitForFadeIn(fadeIn).then(() => expand(line));
      } else {
        expand(line, { trigger: line, start: "top 90%", once: true });
      }
    });
  });
}
