<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import DisplayCSV from "./components/DisplayCSV.vue";

const directoryContents = ref("");
const directoryPath = ref(
  "/home/adeeb/workspace/fidelity-transaction-rust-parser/transactions"
);
let options = ref<string[]>([]);

const selectedFile = ref("");
const showCSVViewer = ref(false);

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
  selectedFile.value = filename;
  showCSVViewer.value = true;
}

function closeCSVViewer() {
  showCSVViewer.value = false;
  selectedFile.value = "";
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
  <div class="app">
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

    <div v-if="showCSVViewer" class="modal-overlay" @click="closeCSVViewer">
      <div class="modal-content" @click.stop>
        <DisplayCSV
          :filename="selectedFile"
          :directory-path="directoryPath"
          @close="closeCSVViewer"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>
.app {
  min-height: 100vh;
  background: linear-gradient(135deg, #368727 0%, #3c8c73 100%);
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
  margin-bottom: 2rem;
}

.input-group {
  display: flex;
  gap: 0.75rem;
  flex-wrap: wrap;
}

.path-input {
  flex: 1;
  min-width: 300px;
  padding: 0.875rem 1rem;
  border: 2px solid #e2e8f0;
  border-radius: 12px;
  font-size: 1rem;
  transition: all 0.2s ease;
  background: #f8fafc;
}

.path-input:focus {
  outline: none;
  border-color: #10b981;
  background: white;
  box-shadow: 0 0 0 3px rgba(16, 185, 129, 0.1);
}

.search-btn {
  padding: 0.875rem 1.5rem;
  background: linear-gradient(135deg, #368727 0%, #3c8c73 100%);
  color: white;
  border: none;
  border-radius: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  min-width: 120px;
  justify-content: center;
}

.search-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 10px 20px rgba(16, 185, 129, 0.3);
}

.search-btn:active {
  transform: translateY(0);
}

.btn-icon {
  font-size: 1rem;
}

.status-message {
  padding: 1rem;
  background: #f0f9ff;
  border: 1px solid #bae6fd;
  border-radius: 8px;
  color: #0369a1;
  margin-bottom: 1.5rem;
  font-weight: 500;
}

.file-list-container {
  margin-top: 1.5rem;
}

.list-title {
  font-size: 1.2rem;
  font-weight: 600;
  margin: 0 0 1rem 0;
  color: #374151;
}

.file-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: grid;
  gap: 0.75rem;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem;
  background: #f8fafc;
  border: 1px solid #e2e8f0;
  border-radius: 12px;
  transition: all 0.2s ease;
}

.file-item-clickable {
  cursor: pointer;
}

.file-item-clickable:hover {
  background: #62c251;
  color: white;
  transform: translateY(-2px);
  box-shadow: 0 8px 16px rgba(16, 185, 129, 0.2);
}

.file-item-disabled {
  cursor: not-allowed;
  opacity: 0.7;
}

.file-item-disabled:hover {
  background: #f1f5f9;
  transform: none;
  box-shadow: none;
}

.file-icon {
  font-size: 1.5rem;
  flex-shrink: 0;
}

.file-name {
  flex: 1;
  font-weight: 500;
  word-break: break-all;
}

.file-index {
  font-size: 0.875rem;
  opacity: 0.7;
  font-weight: 600;
  background: rgba(0, 0, 0, 0.1);
  padding: 0.25rem 0.5rem;
  border-radius: 6px;
}

.file-badge {
  font-size: 0.75rem;
  font-weight: 600;
  padding: 0.25rem 0.5rem;
  border-radius: 6px;
  background: #10b981;
  color: white;
  margin-left: 0.5rem;
}

.file-badge.disabled {
  background: #6b7280;
  color: #d1d5db;
}

.file-item-clickable:hover .file-index,
.file-item-clickable:hover .file-badge {
  background: rgba(255, 255, 255, 0.2);
  opacity: 1;
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

/* Modal styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
  padding: 1rem;
}

.modal-content {
  width: 100%;
  max-width: 1200px;
  max-height: 90vh;
  overflow: hidden;
}

/* Responsive design */
@media (max-width: 640px) {
  .title {
    font-size: 2rem;
  }

  .input-group {
    flex-direction: column;
  }

  .path-input {
    min-width: unset;
  }

  .card {
    padding: 1.5rem;
    margin: 0 0.5rem;
  }
}
</style>
<style>
* {
  box-sizing: border-box;
}

body {
  margin: 0;
  padding: 0;
}

:root {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  line-height: 1.6;
  font-weight: 400;
  color: #2d3748;
  background-color: #f7fafc;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f7fafc;
    background-color: #1a202c;
  }
}
</style>
