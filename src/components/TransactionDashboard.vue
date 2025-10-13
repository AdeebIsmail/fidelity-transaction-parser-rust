<script setup lang="ts">
import { computed } from "vue";
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

import TransactionLineChart from "./TransactionLineChart.vue";
import TransactionPieChart from "./TransactionPieChart.vue";
import TransactionBarChart from "./TransactionBarChart.vue";
import TransactionDoughnutChart from "./charts/TransactionDoughnutChart.vue";

const income = ref(0.0);
const expenses = ref(0.0);
const size = ref(0);
const transactions = ref([]);

async function retrieveTransactionData() {
  income.value = parseFloat(await invoke<string>("get_income"));
  expenses.value = parseFloat(await invoke<string>("get_expenses"));
  size.value = parseInt(await invoke<string>("get_transaction_count"));
  transactions.value = await invoke<[]>("get_transactions");
  console.log("Income:", income.value);
  console.log("Expenses:", expenses.value);
  console.log("Transactions:", transactions);
}

onMounted(() => {
  retrieveTransactionData();
});
</script>

<template>
  <div class="charts-dashboard">
    <div class="dashboard-header">
      <h2 class="dashboard-title">
        <span class="dashboard-icon">📈</span>
        Transaction Analytics Dashboard
      </h2>
      <p class="dashboard-subtitle">Visualizing {{ size }} transactions</p>
    </div>
    <div class="charts-grid">
      <!-- <TransactionDoughnutChart :income="income" :expenses="expenses" /> -->
      <TransactionDoughnutChart :income="income" :expenses="expenses" />
      <TransactionDoughnutChart :income="income" :expenses="expenses" />

      <!-- Row 1: Overview charts -->
      <!-- <div class="chart-wrapper">
        <TransactionDoughnutChart :income="income" :expenses="expenses" />
      </div> -->
      <!-- <div class="chart-wrapper">
        <TransactionDoughnutChart :income="income" :expenses="expenses" />
      </div> -->

      <!-- <div class="chart-col">
          <TransactionPieChart :transactions="transactions" />
        </div> -->

      <!-- Row 2: Trends -->
      <!-- <div class="chart-row full-width">
        <TransactionLineChart :transactions="transactions" />
      </div> -->

      <!-- Row 3: Monthly summary -->
      <!-- <div class="chart-row full-width">
        <TransactionBarChart :transactions="transactions" />
      </div> -->
    </div>

    <!-- <div class="dashboard-footer">
      <div class="stats-summary">
        <div class="stat-card">
          <span class="stat-label">Total Transactions</span>
          <span class="stat-value">{{ size }}</span>
        </div>
        <div class="stat-card">
          <span class="stat-label">Date Range</span>
          <span class="stat-value">{{ -1 }}</span>
        </div>
        <div class="stat-card">
          <span class="stat-label">Total Profit</span>
          <span class="stat-value">${{ (income + expenses).toFixed(2) }}</span>
        </div>
      </div>
    </div> -->
  </div>
</template>

<style scoped>
.charts-dashboard {
  width: 100%;
  max-width: 100%;
  padding: 0;
  margin: 0;
  overflow-x: hidden;
  box-sizing: border-box;
}

.dashboard-header {
  text-align: center;
  margin-bottom: 2rem;
  padding: 1rem;
  border-radius: 16px;
  color: white;
}

.dashboard-title {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  font-size: 2rem;
  font-weight: 700;
  margin: 0 0 0.5rem 0;
}

.dashboard-icon {
  font-size: 2rem;
}

.dashboard-subtitle {
  font-size: 1.1rem;
  opacity: 0.9;
  margin: 0;
}

.charts-grid {
  display: flex;
  flex-direction: row;
  gap: 1.5rem;
  width: 100%;
  box-sizing: border-box;
}

.chart-wrapper {
  flex: 1;
  overflow: hidden;
  background: transparent;
  height: 100%;
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
}

.chart-wrapper {
  flex: 1;
  overflow: hidden;
  border-radius: 12px;
  background: white;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.chart-row {
  display: flex;
  gap: 1rem;
  width: 100%;
}

.chart-row.full-width {
  display: flex;
}

.chart-col {
  flex: 1;
  min-height: 400px;
  max-height: 400px;
  width: 0; /* This forces flex items to respect flex: 1 */
}

.dashboard-footer {
  margin-top: 2rem;
  padding: 1.5rem;
  background: white;
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.stats-summary {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
}

.stat-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 1rem;
  background: #f8fafc;
  border-radius: 8px;
  text-align: center;
}

.stat-label {
  font-size: 0.875rem;
  color: #6b7280;
  font-weight: 500;
  margin-bottom: 0.5rem;
}

.stat-value {
  font-size: 1.25rem;
  font-weight: 700;
  color: #2d3748;
}

/* Responsive design */
@media (max-width: 1024px) {
  .chart-row {
    flex-direction: column;
  }

  .dashboard-title {
    font-size: 1.5rem;
  }
}

@media (max-width: 768px) {
  .charts-dashboard {
    padding: 0;
  }

  .dashboard-header {
    padding: 1rem;
    margin-bottom: 1rem;
  }

  .dashboard-title {
    flex-direction: column;
    font-size: 1.25rem;
  }

  .charts-grid {
    gap: 1rem;
  }

  .stats-summary {
    grid-template-columns: 1fr;
  }
}
</style>
