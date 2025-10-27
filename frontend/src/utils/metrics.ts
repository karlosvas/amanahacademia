import type { MetricData, ParsedArticleMetrics, ParsedClassMetrics, ParsedMetrics } from "@/types/types";

export function parseUsersMetricData(data: MetricData): ParsedMetrics {
  return {
    yearMonth: data.dimensionValues[0].value,
    activeUsers: Number.parseInt(data.metricValues[0].value),
    totalUsers: Number.parseInt(data.metricValues[1].value),
    newUsers: Number.parseInt(data.metricValues[2].value),
    sessions: Number.parseInt(data.metricValues[3].value),
    engagedSessions: Number.parseInt(data.metricValues[4].value),
    avgSessionDuration: Number.parseFloat(data.metricValues[5].value),
    bounceRate: Number.parseFloat(data.metricValues[6].value),
    sessionsPerUser: Number.parseFloat(data.metricValues[7].value),
  };
}

export function parseArticleMetricData(data: MetricData): ParsedArticleMetrics {
  return {
    pagePath: data.dimensionValues[0].value,
    activeUsers: Number.parseInt(data.metricValues[0].value),
    totalUsers: Number.parseInt(data.metricValues[1].value),
    newUsers: Number.parseInt(data.metricValues[2].value),
    sessions: Number.parseInt(data.metricValues[3].value),
    engagedSessions: Number.parseInt(data.metricValues[4].value),
    avgSessionDuration: Number.parseFloat(data.metricValues[5].value),
    bounceRate: Number.parseFloat(data.metricValues[6].value),
    sessionsPerUser: Number.parseFloat(data.metricValues[7].value),
  };
}

export function parseClassMetricData(data: MetricData): ParsedClassMetrics {
  return {
    yearMonth: data.dimensionValues[0].value,
    eventName: data.dimensionValues[1].value,
    bookings: Number.parseInt(data.metricValues[0].value),
  };
}
