<template>
  <div class="chart-container">
    <h3 class="chart-title">
      <span class="chart-icon">🥧</span>
      Transaction Categories
    </h3>
    <Pie :data="chartData" :options="chartOptions" class="chart" />
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { Chart as ChartJS, ArcElement, Tooltip, Legend } from "chart.js";
import { Pie } from "vue-chartjs";

ChartJS.register(ArcElement, Tooltip, Legend);

interface TransactionData {
  amount: number;
  description: string;
  category?: string;
  [key: string]: any;
}

const props = defineProps<{
  transactions: TransactionData[];
}>();

const chartData = computed(() => {
  // Group transactions by category or description
  const categoryMap = new Map<string, number>();

  props.transactions.forEach((tx) => {
    const category = tx.category || categorizeTransaction(tx.description);
    const currentAmount = categoryMap.get(category) || 0;
    categoryMap.set(category, currentAmount + Math.abs(tx.amount));
  });

  const categories = Array.from(categoryMap.keys());
  const amounts = Array.from(categoryMap.values());

  return {
    labels: categories,
    datasets: [
      {
        data: amounts,
        backgroundColor: [
          "#10b981", // Green
          "#3b82f6", // Blue
          "#f59e0b", // Amber
          "#ef4444", // Red
          "#8b5cf6", // Purple
          "#06b6d4", // Cyan
          "#f97316", // Orange
          "#84cc16", // Lime
          "#ec4899", // Pink
          "#6b7280", // Gray
        ],
        borderColor: "#ffffff",
        borderWidth: 2,
        hoverBorderWidth: 3,
        hoverBorderColor: "#ffffff",
      },
    ],
  };
});

// Simple transaction categorization
function categorizeTransaction(description: string): string {
  const desc = description.toLowerCase();

  if (
    desc.includes("grocery") ||
    desc.includes("food") ||
    desc.includes("restaurant")
  ) {
    return "Food & Dining";
  } else if (
    desc.includes("gas") ||
    desc.includes("fuel") ||
    desc.includes("transportation")
  ) {
    return "Transportation";
  } else if (
    desc.includes("shopping") ||
    desc.includes("store") ||
    desc.includes("retail")
  ) {
    return "Shopping";
  } else if (
    desc.includes("bill") ||
    desc.includes("utility") ||
    desc.includes("electric")
  ) {
    return "Bills & Utilities";
  } else if (
    desc.includes("entertainment") ||
    desc.includes("movie") ||
    desc.includes("game")
  ) {
    return "Entertainment";
  } else if (
    desc.includes("health") ||
    desc.includes("medical") ||
    desc.includes("pharmacy")
  ) {
    return "Healthcare";
  } else if (
    desc.includes("transfer") ||
    desc.includes("deposit") ||
    desc.includes("withdrawal")
  ) {
    return "Transfers";
  } else {
    return "Other";
  }
}

const chartOptions = computed(() => ({
  responsive: true,
  maintainAspectRatio: false,
  layout: {
    padding: {
      top: 10,
      bottom: 10,
      left: 10,
      right: 10,
    },
  },
  plugins: {
    legend: {
      position: "bottom" as const,
      maxHeight: 120,
      labels: {
        usePointStyle: true,
        padding: 10,
        boxWidth: 12,
        font: {
          size: 11,
        },
        generateLabels: function (chart: any) {
          const data = chart.data;
          if (data.labels.length && data.datasets.length) {
            return data.labels.map((label: string, i: number) => {
              const value = data.datasets[0].data[i];
              const total = data.datasets[0].data.reduce(
                (sum: number, val: number) => sum + val,
                0
              );
              const percentage = ((value / total) * 100).toFixed(1);
              return {
                text: `${label}: ${percentage}%`,
                fillStyle: data.datasets[0].backgroundColor[i],
                strokeStyle: data.datasets[0].borderColor,
                lineWidth: data.datasets[0].borderWidth,
                pointStyle: "circle",
                hidden: false,
                index: i,
              };
            });
          }
          return [];
        },
      },
    },
    tooltip: {
      backgroundColor: "rgba(0, 0, 0, 0.8)",
      titleColor: "#fff",
      bodyColor: "#fff",
      borderColor: "rgba(16, 185, 129, 1)",
      borderWidth: 1,
      cornerRadius: 8,
      callbacks: {
        label: function (context: any) {
          const total = context.dataset.data.reduce(
            (sum: number, val: number) => sum + val,
            0
          );
          const percentage = ((context.parsed / total) * 100).toFixed(1);
          return `${context.label}: $${context.parsed.toFixed(
            2
          )} (${percentage}%)`;
        },
      },
    },
  },
}));
</script>

<style scoped>
.chart-container {
  background: white;
  border-radius: 12px;
  padding: 1.5rem;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  margin-bottom: 1.5rem;
  /* width: 45vw;
  height: 40vh; */
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.chart-title {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 1.2rem;
  font-weight: 600;
  color: #2d3748;
  margin-bottom: 1rem;
}

.chart-icon {
  font-size: 1.5rem;
}

.chart {
  height: 280px;
  width: 100%;
  max-width: 100%;
  flex: 1;
  min-height: 0;
}
</style>
