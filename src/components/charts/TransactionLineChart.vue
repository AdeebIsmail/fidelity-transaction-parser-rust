<template>
  <div class="chart-container">
    <h3 class="chart-title">
      <span class="chart-icon">📈</span>
      Transaction Amount Over Time
    </h3>
    <div class="chart-content">
      <div class="chart-section">
        <Line :data="chartData" :options="chartOptions" class="chart" />
      </div>
    </div>
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

interface Transaction {
  date: string;
  description: string;
  amount: number; // Amount in cents
  account_name: string;
  category: string;
  transaction_type: string;
  sub_category: string;
  hidden: boolean;
}

const props = defineProps<{
  transactions: Transaction[];
}>();

const chartData = computed(() => {
  // Filter out hidden transactions and sort by date
  const visibleTransactions = props.transactions
    .filter((tx) => !tx.hidden && tx.transaction_type != "Transfers")
    .sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime());

  // Calculate running balance (convert from cents to dollars)
  let runningBalance = 0;
  const balanceData = visibleTransactions.map((tx) => {
    runningBalance += tx.amount / 100; // Convert cents to dollars
    return runningBalance;
  });

  return {
    labels: visibleTransactions.map((tx) =>
      new Date(tx.date).toLocaleDateString("en-US", {
        month: "short",
        day: "numeric",
        year: "2-digit",
      })
    ),
    datasets: [
      {
        label: "Running Balance",
        backgroundColor: "rgba(16, 185, 129, 0.1)",
        borderColor: "rgba(16, 185, 129, 1)",
        borderWidth: 3,
        pointBackgroundColor: "rgba(16, 185, 129, 1)",
        pointBorderColor: "#fff",
        pointBorderWidth: 2,
        pointRadius: 4,
        data: balanceData,
        tension: 0.4,
        fill: true,
      },
      {
        label: "Individual Transactions",
        backgroundColor: "rgba(59, 130, 246, 0.2)",
        borderColor: "rgba(59, 130, 246, 1)",
        borderWidth: 2,
        pointBackgroundColor: (context: any) => {
          const transaction = visibleTransactions[context.dataIndex];
          return transaction?.transaction_type === "Income"
            ? "rgba(34, 197, 94, 1)"
            : "rgba(239, 68, 68, 1)";
        },
        pointBorderColor: "#fff",
        pointBorderWidth: 2,
        pointRadius: 5,
        data: visibleTransactions.map((tx) => tx.amount / 100), // Convert cents to dollars
        tension: 0.2,
        showLine: false, // Show only points for individual transactions
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
      borderColor: "rgba(16, 185, 129, 1)",
      borderWidth: 1,
      cornerRadius: 8,
      displayColors: true,
      callbacks: {
        label: function (context: any) {
          const datasetLabel = context.dataset.label;
          const value = context.parsed.y.toFixed(2);

          if (datasetLabel === "Individual Transactions") {
            // Get the transaction details for individual transaction points
            // Use the SAME filtering logic as chart data
            const visibleTransactions = props.transactions
              .filter((tx) => !tx.hidden && tx.transaction_type != "Transfers")
              .sort(
                (a, b) =>
                  new Date(a.date).getTime() - new Date(b.date).getTime()
              );
            const transaction = visibleTransactions[context.dataIndex];

            if (transaction) {
              return [
                `${transaction.transaction_type}: $${value}`,
                `Category: ${transaction.category}`,
                `Description: ${transaction.description.substring(0, 30)}${
                  transaction.description.length > 30 ? "..." : ""
                }`,
              ];
            }
          }

          return `${datasetLabel}: $${value}`;
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
  padding: 1rem;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  overflow: hidden; /* Changed from hidden to allow legend to show completely */
  height: 100%; /* Increased height to accommodate larger legend */
  width: 100%;
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
}

.chart-title {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 1.2rem;
  font-weight: 600;
  color: #2d3748;
  flex-shrink: 0;
}

.chart-icon {
  font-size: 1.5rem;
}

.chart {
  /* height: 350px; */
  width: 100%;
  /* flex-shrink: 0;  */
}

.chart-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  flex: 1;
  min-height: 0;
  width: 100%;
  overflow: hidden;
  box-sizing: border-box;
}

.chart-section {
  flex: 1;
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 0;
}
</style>
