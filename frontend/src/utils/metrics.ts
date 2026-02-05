import type {
  MetricData,
  ParsedArticleMetrics,
  ParsedClassMetrics,
  ParsedMetrics,
} from "@/types/types";

// Helper: parse values safely with defaults
const safeInt = (v?: string) => Number.parseInt(v ?? "0", 10);
const safeFloat = (v?: string) => Number.parseFloat(v ?? "0");

// Extrae metricas de parseo usados por usuarios y articulos
function parseCommonMetrics(data: MetricData) {
  return {
    activeUsers: safeInt(data.metricValues[0]?.value),
    totalUsers: safeInt(data.metricValues[1]?.value),
    newUsers: safeInt(data.metricValues[2]?.value),
    sessions: safeInt(data.metricValues[3]?.value),
    engagedSessions: safeInt(data.metricValues[4]?.value),
    avgSessionDuration: safeFloat(data.metricValues[5]?.value),
    bounceRate: safeFloat(data.metricValues[6]?.value),
    sessionsPerUser: safeFloat(data.metricValues[7]?.value),
  };
}

export function parseUsersMetricData(data: MetricData): ParsedMetrics {
  return {
    yearMonth: data.dimensionValues[0].value,
    ...parseCommonMetrics(data),
  };
}

export function parseArticleMetricData(data: MetricData): ParsedArticleMetrics {
  return {
    yearMonth: data.dimensionValues[1].value,
    eventName: data.dimensionValues[0].value,
    eventCount: safeInt(data.metricValues[0]?.value),
    totalUsers: safeInt(data.metricValues[1]?.value),
  };
}

export function parseClassMetricData(data: MetricData): ParsedClassMetrics {
  return {
    yearMonth: data.dimensionValues[0].value,
    eventName: data.dimensionValues[1].value,
    bookings: safeInt(data.metricValues[0]?.value),
  };
}

export const mapToMonths = (
  dataMap: Map<string, number>,
  labels: string[],
  monthLabels: string[],
) => {
  return labels.map((label) => {
    const monthIndex = monthLabels.indexOf(label.split(" ")[0]);
    const year = label.split(" ")[1];
    const monthKey = `${year}${String(monthIndex + 1).padStart(2, "0")}`;
    return dataMap.get(monthKey) ?? 0;
  });
};
