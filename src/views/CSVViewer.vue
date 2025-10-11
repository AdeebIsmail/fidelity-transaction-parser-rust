<script setup lang="ts">
import { computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import { ref } from "vue";
import DisplayCSV from "../components/DisplayCSV.vue";
// import { Bar } from "vue-chartjs";

const route = useRoute();
const router = useRouter();

// Get props from route parameters and query
const filename = computed(() => route.params.filename as string);
const directoryPath = computed(() => (route.query.path as string) || "");

const displayCSV = ref(false);
function goBack() {
  router.push("/");
}
function showModal() {
  displayCSV.value = !displayCSV.value;
}
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
    <button @click="showModal" class="back-btn">📊 Display CSV</button>
    <div class="viewer-container" v-if="displayCSV">
      <DisplayCSV
        :filename="filename"
        :directory-path="directoryPath"
        @close="showModal"
      />
    </div>
  </div>
</template>

<style scoped>
.csv-viewer-page {
  min-height: 100vh;
  background: linear-gradient(135deg, #368727 0%, #3c8c73 100%);
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
  max-width: 1200px;
  margin: 0 auto;
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
