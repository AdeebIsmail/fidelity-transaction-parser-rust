<template>
  <div class="chart-container">
    <h3 class="chart-title">
      <span class="chart-icon">🍩</span>
      Income vs Expenses
    </h3>
    <div class="chart-content">
      <Doughnut :data="chartData" :options="chartOptions" class="chart" />
      <div class="chart-summary">
        <div class="summary-item">
          <span class="summary-label">Total Income:</span>
          <span class="summary-value income">${{ props.income }}</span>
        </div>
        <div class="summary-item">
          <span class="summary-label">Total Expenses:</span>
          <span class="summary-value expense">${{ props.expenses }}</span>
        </div>
        <div class="summary-item">
          <span class="summary-label">Net:</span>
          <span
            class="summary-value"
            :class="netAmount >= 0 ? 'income' : 'expense'"
          >
            ${{ netAmount.toFixed(2) }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { Chart as ChartJS, ArcElement, Tooltip, Legend } from "chart.js";
import { Doughnut } from "vue-chartjs";

ChartJS.register(ArcElement, Tooltip, Legend);

const props = defineProps<{
  income: number;
  expenses: number;
}>();

const netAmount = computed(() => props.income - props.expenses);

const chartData = computed(() => ({
  labels: ["Income", "Expenses"],
  datasets: [
    {
      data: [props.income, props.expenses],
      backgroundColor: ["rgba(16, 185, 129, 0.8)", "rgba(239, 68, 68, 0.8)"],
      borderColor: ["rgba(16, 185, 129, 1)", "rgba(239, 68, 68, 1)"],
      borderWidth: 3,
      hoverBorderWidth: 4,
      cutout: "60%",
    },
  ],
}));

const chartOptions = computed(() => ({
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      position: "bottom" as const,
      labels: {
        usePointStyle: true,
        padding: 20,
        font: {
          size: 14,
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
          const total = props.income + props.expenses;
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

.chart-content {
  display: flex;
  align-items: center;
  gap: 2rem;
}

.chart {
  height: 250px;
  flex: 1;
  width: 100%;
  max-width: 100%;
}

.chart-summary {
  flex: 1;
  min-width: 200px;
}

.summary-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 0;
  border-bottom: 1px solid #f1f5f9;
}

.summary-item:last-child {
  border-bottom: none;
  font-weight: 600;
  font-size: 1.1rem;
}

.summary-label {
  color: #6b7280;
  font-weight: 500;
}

.summary-value {
  font-weight: 600;
  font-size: 1.1rem;
}

.summary-value.income {
  color: #10b981;
}

.summary-value.expense {
  color: #ef4444;
}

@media (max-width: 768px) {
  .chart-content {
    flex-direction: column;
    gap: 1rem;
  }
}
</style>
