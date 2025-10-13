<template>
  <div class="chart-container">
    <h3 class="chart-title">
      <span class="chart-icon">🍩</span>
      Income vs Expenses
    </h3>
    <div class="chart-content">
      <div class="chart-section">
        <Doughnut :data="chartData" :options="chartOptions" class="chart" />
      </div>
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

const netAmount = computed(() => props.income + props.expenses);

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
  padding: 1rem;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  height: 100vh;
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
  margin-bottom: 1rem;
  flex-shrink: 0;
}

.chart-icon {
  font-size: 1.5rem;
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

.chart {
  height: 200px;
  width: 200px;
  max-width: 100%;
  max-height: 100%;
  box-sizing: border-box;
}

/* Ensure Chart.js canvas has no default margins/padding */
.chart :deep(canvas) {
  margin: 0 !important;
  padding: 0 !important;
  box-sizing: border-box;
}

.chart-summary {
  flex-shrink: 0;
  max-width: 50%;
  overflow: hidden;
  box-sizing: border-box;
}

.summary-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 0;
  border-bottom: 1px solid #f1f5f9;
  width: 100%;
  box-sizing: border-box;
  min-width: 0; /* Allow flex children to shrink */
}

.summary-item:last-child {
  border-bottom: none;
  font-weight: 600;
  font-size: 1.1rem;
}

.summary-label {
  color: #6b7280;
  font-weight: 500;
  flex-shrink: 0;
  margin-right: 1rem;
  font-size: 0.9rem;
}

.summary-value {
  font-weight: 600;
  font-size: 1rem;
  text-align: right;
  flex-shrink: 0;
  white-space: nowrap;
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
