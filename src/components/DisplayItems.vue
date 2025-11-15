<template>
  <transition name="fade">
    <div
      v-if="open"
      class="modal-overlay"
      @click.self="close"
      role="dialog"
      aria-modal="true"
    >
      <div
        class="modal"
        ref="dialog"
        @keydown.esc.prevent="close"
        tabindex="-1"
      >
        <header class="modal-header">
          <h3 class="modal-title">{{ title }}</h3>
          <button class="close-btn" @click="close" aria-label="Close">✕</button>
        </header>

        <section class="modal-body">
          <div class="row-container" v-if="props.transactions.length">
            <table class="csv-table">
              <thead>
                <tr>
                  <th v-for="(header, index) in header" :key="index">
                    {{ header || `Column ${index + 1}` }}
                  </th>
                </tr>
              </thead>
              <tbody>
                <tr
                  v-for="(row, rowIndex) in props.transactions"
                  :key="rowIndex"
                  class="row"
                >
                  <td v-for="(cell, cellIndex) in row" :key="cellIndex">
                    {{ cell }}
                  </td>
                </tr>
              </tbody>
            </table>
          </div>

          <div v-else class="empty">No data</div>
        </section>

        <footer class="modal-footer">
          <slot name="actions">
            <button @click="close" class="btn">Close</button>
          </slot>
        </footer>
      </div>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { computed, ref, watch, nextTick } from "vue";

interface Transaction {
  date: string;
  description: string;
  amount: number;
  account_name: string;
  category: string;
  transaction_type: string;
  sub_category: string;
  hidden: boolean;
}

const props = defineProps<{
  title: string;
  transactions: Transaction[];
  compact?: boolean;
  modelValue: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: boolean): void;
}>();

const dialog = ref<HTMLDivElement | null>(null);

const open = computed(() => props.modelValue);

const close = () => {
  emit("update:modelValue", false);
};

const header = [
  "Date",
  "Description",
  "Amount (in $)",
  "Account Name",
  "Transaction Type",
  "Category",
  "Subcategory",
  "Hidden Transaction",
];

watch(open, (val) => {
  if (val) {
    // focus the dialog for keyboard handling
    nextTick(() => {
      const el = dialog.value;
      if (el && typeof el.focus === "function") el.focus();
    });
  }
});
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.45);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 50;
  padding: 2rem 1rem;
  overflow: hidden;
}

.modal {
  background: #fff;
  max-width: 1200px;
  width: 100%;
  max-height: 90vh;
  border-radius: 8px;
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.25);
  outline: none;
  display: flex;
  flex-direction: column;
  margin: auto;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1rem;
  border-bottom: 1px solid #eee;
}

.modal-title {
  margin: 0;
  font-size: 1.1rem;
}

.close-btn {
  background: transparent;
  border: none;
  font-size: 1.1rem;
  cursor: pointer;
  padding: 0.25rem;
}

.modal-body {
  padding: 1rem;
  overflow: auto;
  flex: 1;
  min-height: 0;
}

.row-container {
  display: flex;
  gap: 0.5rem;
  flex-wrap: nowrap;
  padding-bottom: 0.25rem;
}

.row-item {
  min-width: 160px;
  background: #f9fafb;
  border: 1px solid #eee;
  border-radius: 6px;
  padding: 0.5rem;
  display: flex;
  flex-direction: column;
}

.row-key {
  font-size: 0.75rem;
  color: #6b7280;
  margin-bottom: 0.35rem;
}

.row-value {
  font-size: 0.95rem;
  color: #111827;
  word-break: break-word;
}

.empty {
  color: #6b7280;
}

.modal-footer {
  padding: 0.5rem 1rem;
  border-top: 1px solid #eee;
  display: flex;
  justify-content: flex-end;
}

.btn {
  padding: 0.4rem 0.8rem;
  border-radius: 6px;
  border: 1px solid #d1d5db;
  background: #fff;
  cursor: pointer;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
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
  top: 0;
  z-index: 1;
}

.csv-table td {
  padding: 0.75rem;
  border-bottom: 1px solid #f1f5f9;
}

.csv-table tbody tr {
  background: #fafbfc;
}

.csv-table tbody tr:hover {
  background: #f1f5f9;
}
</style>
