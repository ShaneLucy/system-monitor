import { createGpuMemoryChart } from "./charts/gpu";

document.addEventListener("DOMContentLoaded", async () => {
  const canvas = document.getElementById("myChart") as HTMLCanvasElement;
  console.log(canvas);
  // if (canvas === null) {
  //   throw new TypeError("unable to get chart context");
  // }
  await createGpuMemoryChart(canvas);
});
