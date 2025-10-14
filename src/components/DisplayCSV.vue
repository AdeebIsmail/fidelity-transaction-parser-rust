<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const props = defineProps<{
  filename: string;
  directoryPath: string;
}>();

const csvData = ref<string[][]>([]);
const loading = ref(false);
const error = ref("");

async function parseCSV() {
  loading.value = true;
  error.value = "";

  try {
    const fullPath = `${props.directoryPath}/${props.filename}`;
    const data = await invoke<string[][]>("parse_csv", { path: fullPath });
    invoke("parse_transactions", { path: fullPath });
    csvData.value = data;
  } catch (err) {
    error.value = `Error parsing CSV: ${err}`;
    console.error("CSV parsing error:", err);
  } finally {
    loading.value = false;
  }
}

onMounted(() => {
  parseCSV();
});
</script>

<template>
  <div class="csv-viewer">
    <div class="csv-header">
      <h2 class="csv-title">
        <span class="csv-icon">📊</span>
        {{ filename }}
      </h2>
      <!-- <button @click="closeView" class="close-btn">
        <span>✕</span>
      </button> -->
    </div>

    <div v-if="loading" class="loading-state">
      <span class="loading-icon">⏳</span>
      <p>Parsing CSV file...</p>
    </div>

    <div v-else-if="error" class="error-state">
      <span class="error-icon">❌</span>
      <p>{{ error }}</p>
      <button @click="parseCSV" class="retry-btn">Try Again</button>
    </div>

    <div v-else-if="csvData.length > 0" class="csv-content">
      <div class="csv-stats">
        <span class="stat">Rows: {{ csvData.length }}</span>
        <span class="stat">Columns: {{ csvData[0]?.length || 0 }}</span>
      </div>

      <div class="table-container">
        <table class="csv-table">
          <thead>
            <tr>
              <th v-for="(header, index) in csvData[0]" :key="index">
                {{ header || `Column ${index + 1}` }}
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(row, rowIndex) in csvData.slice(1)" :key="rowIndex">
              <td v-for="(cell, cellIndex) in row" :key="cellIndex">
                {{ cell }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <div v-else class="empty-state">
      <span class="empty-icon">📄</span>
      <p>No data found in CSV file</p>
    </div>
  </div>
</template>

<style scoped>
.csv-viewer {
  background: white;
  border-radius: 16px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
  padding: 2rem;
  /* max-height: 80vh; */
  /* width: 100vw; */
  /* overflow: hidden;
  display: flex;
  flex-direction: column; */
}

.csv-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
  padding-bottom: 1rem;
  border-bottom: 2px solid #e2e8f0;
}

.csv-title {
  font-size: 1.5rem;
  font-weight: 600;
  margin: 0;
  color: #2d3748;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.csv-icon {
  font-size: 1.5rem;
}

.close-btn {
  background: #ef4444;
  color: white;
  border: none;
  border-radius: 8px;
  padding: 0.5rem;
  cursor: pointer;
  font-size: 1.2rem;
  transition: all 0.2s ease;
}

.close-btn:hover {
  background: #dc2626;
  transform: scale(1.1);
}

.loading-state,
.error-state,
.empty-state {
  text-align: center;
  padding: 3rem 1rem;
  color: #6b7280;
}

.loading-icon,
.error-icon,
.empty-icon {
  font-size: 3rem;
  display: block;
  margin-bottom: 1rem;
}

.retry-btn {
  background: #10b981;
  color: white;
  border: none;
  border-radius: 8px;
  padding: 0.75rem 1.5rem;
  cursor: pointer;
  font-weight: 600;
  margin-top: 1rem;
}

.retry-btn:hover {
  background: #059669;
}

.csv-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.csv-stats {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
}

.stat {
  background: #f0f9ff;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  font-weight: 600;
  color: #0369a1;
  font-size: 0.875rem;
}

.table-container {
  flex: 1;
  overflow: auto;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
}

.csv-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.875rem;
}

.csv-table th {
  background: #f8fafc;
  padding: 0.75rem;
  text-align: left;
  font-weight: 600;
  border-bottom: 2px solid #e2e8f0;
  position: sticky;
  top: 0;
  z-index: 1;
}

.csv-table td {
  padding: 0.75rem;
  border-bottom: 1px solid #f1f5f9;
}

.csv-table tbody tr:hover {
  background: #f8fafc;
}

.csv-table tbody tr:nth-child(even) {
  background: #fafbfc;
}

.csv-table tbody tr:nth-child(even):hover {
  background: #f1f5f9;
}
</style>
