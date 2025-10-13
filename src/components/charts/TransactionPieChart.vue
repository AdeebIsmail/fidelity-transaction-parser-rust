<template>
  <div class="chart-container">
    <h3 class="chart-title">
      <span class="chart-icon">🥧</span>
      Transaction Categories
    </h3>
    <div class="chart-content">
      <div class="chart-section">
        <Pie :data="chartData" :options="chartOptions" class="chart" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { Chart as ChartJS, ArcElement, Tooltip, Legend } from "chart.js";
import { Pie } from "vue-chartjs";

ChartJS.register(ArcElement, Tooltip, Legend);

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
  const categoryMap = new Map<string, number>();

  props.transactions
    .filter((tx) => !tx.hidden && tx.transaction_type == "Expenses")
    .forEach((tx) => {
      const category = tx.category;
      const currentAmount = categoryMap.get(category) || 0;
      categoryMap.set(category, currentAmount + Math.abs(tx.amount / 100));
    });

  const categories = Array.from(categoryMap.keys());
  const amounts = Array.from(categoryMap.values());

  return {
    labels: categories,
    datasets: [
      {
        data: amounts,
        backgroundColor: generateColors(categories.length), // Dynamic color generation
        borderColor: "#ffffff",
        borderWidth: 2,
        hoverBorderWidth: 3,
        hoverBorderColor: "#ffffff",
      },
    ],
  };
});

// Function to generate colors dynamically based on number of categories
function generateColors(count: number): string[] {
  const baseColors = [
    "#10b981",
    "#3b82f6",
    "#f59e0b",
    "#ef4444",
    "#8b5cf6",
    "#06b6d4",
    "#f97316",
    "#84cc16",
    "#ec4899",
    "#6b7280",
    "#14b8a6",
    "#f43f5e",
    "#a78bfa",
    "#34d399",
    "#fbbf24",
    "#fb7185",
    "#60a5fa",
    "#4ade80",
    "#fa8c4a",
    "#c084fc",
    "#2dd4bf",
    "#94a3b8",
    "#38bdf8",
    "#22c55e",
  ];

  // If we need more colors than in our base array, generate additional ones
  if (count <= baseColors.length) {
    return baseColors.slice(0, count);
  }

  // Generate additional colors using HSL
  const colors = [...baseColors];
  for (let i = baseColors.length; i < count; i++) {
    const hue = (i * 137.508) % 360; // Golden angle for good distribution
    const saturation = 65 + (i % 3) * 10; // Vary saturation slightly
    const lightness = 50 + (i % 4) * 5; // Vary lightness slightly
    colors.push(`hsl(${hue}, ${saturation}%, ${lightness}%)`);
  }

  return colors;
}

const chartOptions = computed(() => ({
  responsive: true,
  maintainAspectRatio: false,
  layout: {
    padding: {
      top: 10,
      bottom: 10,
      left: 10,
      right: 10,
    },
  },
  plugins: {
    legend: {
      position: "bottom" as const,
      maxHeight: 300, // Increased further to ensure all labels fit
      onClick: (_e: any, legendItem: any, legend: any) => {
        // Toggle the dataset visibility
        const index = legendItem.index;
        const chart = legend.chart;
        const meta = chart.getDatasetMeta(0);

        meta.data[index].hidden = !meta.data[index].hidden;
        chart.update();
      },
      labels: {
        usePointStyle: true,
        padding: 12, // Reduced padding slightly to fit more items
        boxWidth: 12,
        font: {
          size: 11, // Slightly smaller font to fit more labels
        },
        textAlign: "left" as const,
        generateLabels: function (chart: any) {
          const data = chart.data;
          if (data.labels.length && data.datasets.length) {
            return data.labels.map((label: string, i: number) => {
              const value = data.datasets[0].data[i];
              const total = data.datasets[0].data.reduce(
                (sum: number, val: number) => sum + val,
                0
              );
              const percentage = ((value / total) * 100).toFixed(1);
              // Truncate very long labels to prevent overflow
              const displayLabel =
                label.length > 20 ? label.substring(0, 17) + "..." : label;

              // Check if this data point is hidden
              const meta = chart.getDatasetMeta(0);
              const isHidden = meta && meta.data[i] && meta.data[i].hidden;

              return {
                text: `${displayLabel}: ${percentage}%`,
                fillStyle: isHidden
                  ? "transparent"
                  : data.datasets[0].backgroundColor[i],
                strokeStyle: isHidden
                  ? data.datasets[0].backgroundColor[i]
                  : data.datasets[0].borderColor,
                lineWidth: isHidden ? 2 : data.datasets[0].borderWidth,
                pointStyle: isHidden ? "dash" : "circle", // Use dash when hidden
                hidden: false, // Keep the legend item visible
                index: i,
              };
            });
          }
          return [];
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
          const total = context.dataset.data.reduce(
            (sum: number, val: number) => sum + val,
            0
          );
          const percentage = ((context.parsed / total) * 100).toFixed(1);
          // Format as currency since amounts are already converted from cents
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
