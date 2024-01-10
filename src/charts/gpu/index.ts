import { invoke } from "@tauri-apps/api/tauri";
import { ChartData } from "../../types";

import { Chart, registerables } from "chart.js";

export const createGpuMemoryChart = async (canvas: HTMLCanvasElement) => {
  Chart.register(...registerables);
  const gpuMemoryStats = (await invoke("gpu_memory_stats")) as ChartData;

  console.log(JSON.stringify(gpuMemoryStats));
  const chart = new Chart(canvas, {
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

  //   while (true) {
  //     const gpuMemoryUsage: GpuMemoryUsage = (await invoke(
  //       "gpu_memory_stats"
  //     )) as GpuMemoryUsage;

  //     chart.data.datasets[0].data = [
  //       gpuMemoryUsage.used_memory,
  //       gpuMemoryUsage.free_memory,
  //     ];
  //     chart.update();
  //   }
};
