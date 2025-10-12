<template>
  <div class="chart-container">
    <h3 class="chart-title">
      <span class="chart-icon">📈</span>
      Transaction Amount Over Time
    </h3>
    <Line :data="chartData" :options="chartOptions" class="chart" />
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler,
} from "chart.js";
import { Line } from "vue-chartjs";

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler
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
  // Sort transactions by date
  const sortedTransactions = [...props.transactions].sort(
    (a, b) => new Date(a.date).getTime() - new Date(b.date).getTime()
  );

  // Calculate running balance
  let runningBalance = 0;
  const balanceData = sortedTransactions.map((tx) => {
    runningBalance += tx.amount;
    return runningBalance;
  });

  return {
    labels: sortedTransactions.map((tx) =>
      new Date(tx.date).toLocaleDateString()
    ),
    datasets: [
      {
        label: "Transaction Amount",
        backgroundColor: "rgba(54, 135, 39, 0.2)",
        borderColor: "rgba(54, 135, 39, 1)",
        borderWidth: 3,
        pointBackgroundColor: "rgba(54, 135, 39, 1)",
        pointBorderColor: "#fff",
        pointBorderWidth: 2,
        pointRadius: 5,
        data: props.transactions.map((tx) => tx.amount),
        tension: 0.4,
      },
      {
        label: "Running Balance",
        backgroundColor: "rgba(60, 140, 115, 0.1)",
        borderColor: "rgba(60, 140, 115, 1)",
        borderWidth: 2,
        pointBackgroundColor: "rgba(60, 140, 115, 1)",
        pointBorderColor: "#fff",
        pointBorderWidth: 2,
        pointRadius: 4,
        data: balanceData,
        tension: 0.4,
        fill: true,
      },
    ],
  };
});

const chartOptions = computed(() => ({
  responsive: true,
  maintainAspectRatio: false,
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
    title: {
      display: false,
    },
    tooltip: {
      backgroundColor: "rgba(0, 0, 0, 0.8)",
      titleColor: "#fff",
      bodyColor: "#fff",
      borderColor: "rgba(54, 135, 39, 1)",
      borderWidth: 1,
      cornerRadius: 8,
      displayColors: true,
      callbacks: {
        label: function (context: any) {
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
        maxRotation: 45,
        color: "#6b7280",
        font: {
          size: 11,
        },
      },
    },
    y: {
      grid: {
        color: "rgba(0, 0, 0, 0.1)",
      },
      ticks: {
        color: "#6b7280",
        font: {
          size: 11,
        },
        callback: function (value: any) {
          return "$" + value.toFixed(2);
        },
      },
    },
  },
  interaction: {
    intersect: false,
    mode: "index" as const,
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
  height: 300px;
  width: 100%;
  max-width: 100%;
}
</style>
