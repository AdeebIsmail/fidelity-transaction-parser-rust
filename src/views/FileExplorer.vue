<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from "vue-router";

const router = useRouter();

const directoryContents = ref("");
const directoryPath = ref(
  "/home/adeeb/workspace/fidelity-transaction-rust-parser/transactions"
);
let options = ref<string[]>([]);

async function readDirectory() {
  try {
    const files = await invoke<string[]>("list_dir", {
      path: directoryPath.value,
    });
    options.value = files;
    const csvCount = files.filter((file) => getFileType(file) === "csv").length;
    directoryContents.value = `Found ${files.length} files (${csvCount} CSV files)`;
  } catch (error) {
    directoryContents.value = `Error: ${error}`;
    options.value = [];
  }
}

function selectFile(filename: string) {
  console.log("Selected CSV file:", filename);
  // Navigate to CSV viewer route with filename and directory path
  router.push({
    name: "csv-viewer",
    params: { filename },
    query: { path: directoryPath.value },
  });
}

function getFileIcon(filename: string): string {
  const ext = filename.split(".").pop()?.toLowerCase();
  switch (ext) {
    case "csv":
      return "📊";
    case "txt":
      return "📄";
    case "json":
      return "📋";
    case "xml":
      return "📰";
    case "pdf":
      return "📕";
    case "xls":
    case "xlsx":
      return "📗";
    default:
      return "📄";
  }
}

function getFileType(filename: string): string {
  const ext = filename.split(".").pop()?.toLowerCase();
  if (ext != undefined) return ext;
  return "";
}
</script>

<template>
  <div class="file-explorer">
    <header class="header">
      <h1 class="title">
        <span class="icon">📁</span>
        Fidelity Transaction Parser
      </h1>
      <p class="subtitle">Browse and parse your transaction files</p>
    </header>

    <main class="main-content">
      <div class="card">
        <h2 class="card-title">Directory Browser</h2>

        <form class="search-form" @submit.prevent="readDirectory">
          <div class="input-group">
            <input
              id="path-input"
              v-model="directoryPath"
              placeholder="Enter directory path..."
              class="path-input"
            />
            <button type="submit" class="search-btn">
              <span class="btn-icon">🔍</span>
              Browse
            </button>
          </div>
        </form>

        <div v-if="directoryContents" class="status-message">
          {{ directoryContents }}
        </div>

        <div v-if="options.length > 0" class="file-list-container">
          <h3 class="list-title">Files Found:</h3>
          <ul class="file-list">
            <li
              v-for="(item, index) in options"
              :key="index"
              :class="[
                'file-item',
                getFileType(item) === 'csv'
                  ? 'file-item-clickable'
                  : 'file-item-disabled',
              ]"
              @click="getFileType(item) === 'csv' ? selectFile(item) : null"
            >
              <span class="file-icon">
                {{ getFileIcon(item) }}
              </span>
              <span class="file-name">{{ item }}</span>
              <span class="file-index">#{{ index + 1 }}</span>
            </li>
          </ul>
        </div>

        <div
          v-else-if="directoryContents && !directoryContents.includes('Error')"
          class="empty-state"
        >
          <span class="empty-icon">📂</span>
          <p>No files found in this directory</p>
        </div>
      </div>
    </main>
  </div>
</template>

<style scoped>
.file-explorer {
  min-height: 100vh;
  background: linear-gradient(135deg, #368727 0%, #5c9951 100%);
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
}

.header {
  text-align: center;
  padding: 2rem 1rem;
  color: white;
}

.title {
  font-size: 2.5rem;
  font-weight: 700;
  margin: 0 0 0.5rem 0;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
}

.icon {
  font-size: 2rem;
}

.subtitle {
  font-size: 1.1rem;
  opacity: 0.9;
  margin: 0;
  font-weight: 300;
}

.main-content {
  max-width: 800px;
  margin: 0 auto;
  padding: 0 1rem 2rem 1rem;
}

.card {
  background: white;
  border-radius: 16px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
  padding: 2rem;
  backdrop-filter: blur(10px);
}

.card-title {
  font-size: 1.5rem;
  font-weight: 600;
  margin: 0 0 1.5rem 0;
  color: #2d3748;
}

.search-form {
  margin-bottom: 1.5rem;
}

.input-group {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.path-input {
  flex: 1;
  min-width: 300px;
  padding: 0.75rem 1rem;
  border: 2px solid #e2e8f0;
  border-radius: 8px;
  font-size: 1rem;
  transition: all 0.2s ease;
}

.path-input:focus {
  outline: none;
  border-color: #368727;
  box-shadow: 0 0 0 3px rgba(54, 135, 39, 0.1);
}

.search-btn {
  background: linear-gradient(135deg, #368727 0%, #5c9951 100%);
  color: white;
  border: none;
  border-radius: 8px;
  padding: 0.75rem 1.5rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.search-btn:hover {
  background: linear-gradient(135deg, #327b25 0%, #5c9951 100%);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(54, 135, 39, 0.3);
}

.btn-icon {
  font-size: 1rem;
}

.status-message {
  background: #f0f7f0;
  border: 1px solid #a7d3a7;
  border-radius: 8px;
  padding: 1rem;
  margin-bottom: 1.5rem;
  color: #2d6b20;
  font-weight: 500;
}

.file-list-container {
  margin-top: 1.5rem;
}

.list-title {
  font-size: 1.1rem;
  font-weight: 600;
  margin: 0 0 1rem 0;
  color: #374151;
}

.file-list {
  list-style: none;
  padding: 0;
  margin: 0;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  overflow: hidden;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  border-bottom: 1px solid #f3f4f6;
  transition: all 0.2s ease;
}

.file-item:last-child {
  border-bottom: none;
}

.file-item-clickable {
  cursor: pointer;
  background: white;
}

.file-item-clickable:hover {
  background: #f7fcf7;
  transform: translateX(4px);
  box-shadow: 0 2px 8px rgba(54, 135, 39, 0.15);
}

.file-item-disabled {
  background: #f9fafb;
  color: #9ca3af;
  cursor: not-allowed;
}

.file-icon {
  font-size: 1.5rem;
  flex-shrink: 0;
}

.file-name {
  flex: 1;
  font-weight: 500;
  color: #374151;
}

.file-item-disabled .file-name {
  color: #9ca3af;
}

.file-index {
  background: #d1d5db;
  color: #6b7280;
  font-size: 0.75rem;
  font-weight: 600;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  flex-shrink: 0;
}

.file-item-clickable .file-index {
  background: #368727;
  color: white;
}

.empty-state {
  text-align: center;
  padding: 3rem 1rem;
  color: #6b7280;
}

.empty-icon {
  font-size: 3rem;
  display: block;
  margin-bottom: 1rem;
}

.small-text {
  font-size: 0.875rem;
  opacity: 0.7;
  margin-top: 0.5rem;
}
</style>
