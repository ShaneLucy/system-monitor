import { Chart, registerables } from "chart.js";

import { createGpuMemoryChart, updateGpuMemoryChart } from "./charts/gpu";
import {
  createCpuUtilisationChart,
  updateCpuUtilisationChart,
} from "./charts/cpu";
import { invoke } from "@tauri-apps/api/tauri";

document.addEventListener("DOMContentLoaded", async () => {
  Chart.register(...registerables);
  const canvas = document.getElementById("myChart") as HTMLCanvasElement;

  if (canvas === null) {
    throw new TypeError("unable to get chart context");
  }

  // const gpuChart = await createGpuMemoryChart(canvas);
  // updateGpuMemoryChart(gpuChart);

  const chart = await createCpuUtilisationChart(canvas);

  updateCpuUtilisationChart(chart);
});
