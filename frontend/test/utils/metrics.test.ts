import { describe, it, expect } from "vitest";
import { parseUsersMetricData, parseArticleMetricData, parseClassMetricData, mapToMonths } from "@/utils/metrics";
import type { MetricData } from "@/types/types";

describe("Metrics Utilities", () => {
  describe("parseUsersMetricData", () => {
    it("should parse user metrics correctly", () => {
      const mockData: MetricData = {
        dimensionValues: [{ value: "202401" }],
        metricValues: [
          { value: "100" }, // activeUsers
          { value: "500" }, // totalUsers
          { value: "50" }, // newUsers
          { value: "200" }, // sessions
          { value: "150" }, // engagedSessions
          { value: "120.5" }, // avgSessionDuration
          { value: "0.25" }, // bounceRate
          { value: "2.5" }, // sessionsPerUser
        ],
      };

      const result = parseUsersMetricData(mockData);

      expect(result).toEqual({
        yearMonth: "202401",
        activeUsers: 100,
        totalUsers: 500,
        newUsers: 50,
        sessions: 200,
        engagedSessions: 150,
        avgSessionDuration: 120.5,
        bounceRate: 0.25,
        sessionsPerUser: 2.5,
      });
    });

    it("should handle missing values with defaults", () => {
      const mockData: MetricData = {
        dimensionValues: [{ value: "202401" }],
        metricValues: [],
      };

      const result = parseUsersMetricData(mockData);

      expect(result).toEqual({
        yearMonth: "202401",
        activeUsers: 0,
        totalUsers: 0,
        newUsers: 0,
        sessions: 0,
        engagedSessions: 0,
        avgSessionDuration: 0,
        bounceRate: 0,
        sessionsPerUser: 0,
      });
    });
  });

  describe("parseArticleMetricData", () => {
    it("should parse article metrics correctly", () => {
      const mockData: MetricData = {
        dimensionValues: [{ value: "article_view" }, { value: "202401" }],
        metricValues: [
          { value: "250" }, // eventCount
          { value: "150" }, // totalUsers
        ],
      };

      const result = parseArticleMetricData(mockData);

      expect(result).toEqual({
        yearMonth: "202401",
        eventName: "article_view",
        eventCount: 250,
        totalUsers: 150,
      });
    });

    it("should handle undefined values", () => {
      const mockData: MetricData = {
        dimensionValues: [{ value: "test_event" }, { value: "202401" }],
        metricValues: [{ value: undefined }, { value: undefined }],
      };

      const result = parseArticleMetricData(mockData);

      expect(result).toEqual({
        yearMonth: "202401",
        eventName: "test_event",
        eventCount: 0,
        totalUsers: 0,
      });
    });
  });

  describe("parseClassMetricData", () => {
    it("should parse class metrics correctly", () => {
      const mockData: MetricData = {
        dimensionValues: [{ value: "202401" }, { value: "booking_created" }],
        metricValues: [
          { value: "42" }, // bookings
        ],
      };

      const result = parseClassMetricData(mockData);

      expect(result).toEqual({
        yearMonth: "202401",
        eventName: "booking_created",
        bookings: 42,
      });
    });
  });

  describe("mapToMonths", () => {
    it("should map data to months correctly", () => {
      const dataMap = new Map<string, number>([
        ["202401", 100],
        ["202402", 150],
        ["202403", 200],
      ]);

      const labels = ["enero 2024", "febrero 2024", "marzo 2024"];
      const monthLabels = [
        "enero",
        "febrero",
        "marzo",
        "abril",
        "mayo",
        "junio",
        "julio",
        "agosto",
        "septiembre",
        "octubre",
        "noviembre",
        "diciembre",
      ];

      const result = mapToMonths(dataMap, labels, monthLabels);

      expect(result).toEqual([100, 150, 200]);
    });

    it("should return 0 for months without data", () => {
      const dataMap = new Map<string, number>([
        ["202401", 100],
        ["202403", 200],
      ]);

      const labels = ["enero 2024", "febrero 2024", "marzo 2024"];
      const monthLabels = [
        "enero",
        "febrero",
        "marzo",
        "abril",
        "mayo",
        "junio",
        "julio",
        "agosto",
        "septiembre",
        "octubre",
        "noviembre",
        "diciembre",
      ];

      const result = mapToMonths(dataMap, labels, monthLabels);

      expect(result).toEqual([100, 0, 200]);
    });

    it("should handle empty data map", () => {
      const dataMap = new Map<string, number>();
      const labels = ["enero 2024", "febrero 2024"];
      const monthLabels = [
        "enero",
        "febrero",
        "marzo",
        "abril",
        "mayo",
        "junio",
        "julio",
        "agosto",
        "septiembre",
        "octubre",
        "noviembre",
        "diciembre",
      ];

      const result = mapToMonths(dataMap, labels, monthLabels);

      expect(result).toEqual([0, 0]);
    });
  });
});
