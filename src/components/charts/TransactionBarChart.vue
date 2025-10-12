<template>
  <div class="chart-container">
    <h3 class="chart-title">
      <span class="chart-icon">📊</span>
      Monthly Transaction Summary
    </h3>
    <Bar :data="chartData" :options="chartOptions" class="chart" />
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend,
} from "chart.js";
import { Bar } from "vue-chartjs";

ChartJS.register(
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend
);

interface TransactionData {
  date: string;
  amount: number;
  description: string;
  [key: string]: any;
}

const props = defineProps<{
  transactions: TransactionData[];
}>();

const chartData = computed(() => {
  // Group transactions by month
  const monthlyData = new Map<
    string,
    { income: number; expenses: number; count: number }
  >();

  props.transactions.forEach((tx) => {
    const date = new Date(tx.date);
    const monthKey = `${date.getFullYear()}-${String(
      date.getMonth() + 1
    ).padStart(2, "0")}`;

    if (!monthlyData.has(monthKey)) {
      monthlyData.set(monthKey, { income: 0, expenses: 0, count: 0 });
    }

    const monthData = monthlyData.get(monthKey)!;
    if (tx.amount > 0) {
      monthData.income += tx.amount;
    } else {
      monthData.expenses += Math.abs(tx.amount);
    }
    monthData.count++;
  });

  const sortedMonths = Array.from(monthlyData.keys()).sort();

  return {
    labels: sortedMonths.map((month) => {
      const [year, monthNum] = month.split("-");
      const date = new Date(parseInt(year), parseInt(monthNum) - 1);
      return date.toLocaleDateString("en-US", {
        month: "short",
        year: "numeric",
      });
    }),
    datasets: [
      {
        label: "Income",
        backgroundColor: "rgba(16, 185, 129, 0.8)",
        borderColor: "rgba(16, 185, 129, 1)",
        borderWidth: 1,
        borderRadius: 6,
        borderSkipped: false,
        data: sortedMonths.map((month) => monthlyData.get(month)!.income),
      },
      {
        label: "Expenses",
        backgroundColor: "rgba(239, 68, 68, 0.8)",
        borderColor: "rgba(239, 68, 68, 1)",
        borderWidth: 1,
        borderRadius: 6,
        borderSkipped: false,
        data: sortedMonths.map((month) => monthlyData.get(month)!.expenses),
      },
      {
        label: "Transaction Count",
        backgroundColor: "rgba(59, 130, 246, 0.6)",
        borderColor: "rgba(59, 130, 246, 1)",
        borderWidth: 1,
        borderRadius: 6,
        borderSkipped: false,
        yAxisID: "y1",
        data: sortedMonths.map((month) => monthlyData.get(month)!.count),
      },
    ],
  };
});

const chartOptions = computed(() => ({
  responsive: true,
  maintainAspectRatio: false,
  interaction: {
    mode: "index" as const,
    intersect: false,
  },
  plugins: {
    legend: {
      position: "top" as const,
      labels: {
        usePointStyle: true,
        padding: 20,
        font: {
          size: 12,
          weight: "bold" as const,
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
          if (context.datasetIndex === 2) {
            return `${context.dataset.label}: ${context.parsed.y} transactions`;
          }
          return `${context.dataset.label}: $${context.parsed.y.toFixed(2)}`;
        },
      },
    },
  },
  scales: {
    x: {
      grid: {
        display: false,
      },
      ticks: {
        color: "#6b7280",
        font: {
          size: 11,
        },
      },
    },
    y: {
      type: "linear" as const,
      display: true,
      position: "left" as const,
      grid: {
        color: "rgba(0, 0, 0, 0.1)",
      },
      ticks: {
        color: "#6b7280",
        font: {
          size: 11,
        },
        callback: function (value: any) {
          return "$" + value.toFixed(0);
        },
      },
    },
    y1: {
      type: "linear" as const,
      display: true,
      position: "right" as const,
      grid: {
        drawOnChartArea: false,
      },
      ticks: {
        color: "#6b7280",
        font: {
          size: 11,
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
  width: 100%;
  max-width: 100%;
  overflow: hidden;
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
  height: 350px;
  width: 100%;
  max-width: 100%;
}
</style>
