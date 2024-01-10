import { invoke } from "@tauri-apps/api/tauri";

import { ChartData } from "../../types";

import { Chart, registerables } from "chart.js";

export const createGpuMemoryChart = async (canvas: HTMLCanvasElement) => {
  Chart.register(...registerables);
  const gpuMemoryStats = (await invoke("gpu_memory_stats")) as ChartData;
  return new Chart(canvas, {
    type: "doughnut",
    data: {
      labels: gpuMemoryStats.labels,
      datasets: [
        {
          backgroundColor: gpuMemoryStats.datasets[0].background_color,
          data: gpuMemoryStats.datasets[0].data,
          hoverOffset: 4,
          label: "GPU Memory Utilisation",
        },
      ],
    },
  });
};

export const updateGpuMemoryChart = async (
  chart: Chart<"doughnut", number[], string>
) => {
  try {
    const gpuMemoryStats = (await invoke("gpu_memory_stats")) as ChartData;

    chart.data.datasets[0].data = [...gpuMemoryStats.datasets[0].data];
    chart.update();
    updateGpuMemoryChart(chart);
  } catch (error) {
    console.error("error calling back end for gpu stats");
    setTimeout(updateGpuMemoryChart, 1_000);
  }
};
