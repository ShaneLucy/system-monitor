import { createGpuMemoryChart, updateGpuMemoryChart } from "./charts/gpu";

document.addEventListener("DOMContentLoaded", async () => {
  const canvas = document.getElementById("myChart") as HTMLCanvasElement;

  if (canvas === null) {
    throw new TypeError("unable to get chart context");
  }
  const gpuChart = await createGpuMemoryChart(canvas);
  updateGpuMemoryChart(gpuChart);
});
