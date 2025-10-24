import { describe, it, expect } from "vitest";

// Función simple para testear
export const add = (a: number, b: number): number => a + b;

describe("Utils - add", () => {
  it("should add two positive numbers correctly", () => {
    expect(add(2, 3)).toBe(5);
  });

  it("should add negative numbers", () => {
    expect(add(-1, -2)).toBe(-3);
  });

  it("should return zero when both are zero", () => {
    expect(add(0, 0)).toBe(0);
  });
});
