<script setup lang="ts">
import { computed, ref } from "vue";
import { useRoute, useRouter } from "vue-router";

import DisplayCSV from "../components/DisplayCSV.vue";
import TransactionDashboard from "../components/TransactionDashboard.vue";

const route = useRoute();
const router = useRouter();

// Get props from route parameters and query
const filename = computed(() => route.params.filename as string);
const directoryPath = computed(() => (route.query.path as string) || "");

const viewMode = ref<"table" | "charts">("table");

function goBack() {
  router.push("/");
}

function toggleView(mode: "table" | "charts") {
  viewMode.value = mode;
}

// Mock transaction data for demo - replace with actual CSV parsing
</script>

<template>
  <div class="csv-viewer-page">
    <div class="nav-bar">
      <button @click="goBack" class="back-btn">
        <span class="back-icon">←</span>
        Back to Files
      </button>
      <div class="breadcrumb">
        <span class="breadcrumb-path">{{ directoryPath }}</span>
        <span class="breadcrumb-separator">/</span>
        <span class="breadcrumb-file">{{ filename }}</span>
      </div>
    </div>

    <!-- View Toggle Buttons -->
    <div class="view-controls">
      <button
        @click="toggleView('table')"
        :class="['view-btn', { active: viewMode === 'table' }]"
      >
        <span class="btn-icon">📋</span>
        Table View
      </button>
      <button
        @click="toggleView('charts')"
        :class="['view-btn', { active: viewMode === 'charts' }]"
      >
        <span class="btn-icon">📊</span>
        Charts View
      </button>
    </div>

    <!-- Content Based on View Mode -->
    <div class="viewer-container">
      <DisplayCSV
        v-if="viewMode === 'table'"
        :filename="filename"
        :directory-path="directoryPath"
        @close="goBack"
      />

      <TransactionDashboard v-if="viewMode === 'charts'" />
    </div>
  </div>
</template>

<style scoped>
.csv-viewer-page {
  min-height: 100vh;
  background: linear-gradient(135deg, #368727 0%, #5c9951 100%);
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  padding: 1rem;
}

.nav-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1rem;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  border-radius: 12px;
  padding: 1rem;
  color: white;
}

.view-controls {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1.5rem;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  border-radius: 12px;
  padding: 0.5rem;
}

.view-btn {
  background: rgba(255, 255, 255, 0.2);
  color: white;
  border: none;
  border-radius: 8px;
  padding: 0.75rem 1.5rem;
  cursor: pointer;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  transition: all 0.2s ease;
  flex: 1;
  justify-content: center;
}

.view-btn:hover {
  background: rgba(255, 255, 255, 0.3);
  transform: translateY(-1px);
}

.view-btn.active {
  background: rgba(255, 255, 255, 0.9);
  color: #2d3748;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

.btn-icon {
  font-size: 1.1rem;
}

.back-btn {
  background: rgba(255, 255, 255, 0.2);
  color: white;
  border: none;
  border-radius: 8px;
  padding: 0.75rem 1.5rem;
  cursor: pointer;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  transition: all 0.2s ease;
  transform: scale(0.8);
}

.back-btn:hover {
  background: rgba(255, 255, 255, 0.3);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

.back-icon {
  font-size: 1.2rem;
}

.breadcrumb {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.9rem;
  opacity: 0.9;
}

.breadcrumb-path {
  opacity: 0.7;
}

.breadcrumb-separator {
  opacity: 0.5;
}

.breadcrumb-file {
  font-weight: 600;
}

.viewer-container {
  width: 100%;
  max-width: 100%;
  overflow-x: hidden;
  box-sizing: border-box;
}

@media (max-width: 768px) {
  .nav-bar {
    flex-direction: column;
    gap: 1rem;
    align-items: flex-start;
  }

  .breadcrumb {
    font-size: 0.8rem;
    word-break: break-all;
  }
}
</style>
