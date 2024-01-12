import { appWindow, WebviewWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";
import { Chart } from "chart.js";
import type { ChartData } from "../../types";

type InitialCpuUtilisationChartData = {
  data: Array<number>;
  background_color: string;
  label: string;
  index: number;
};

type CpuUtilisationChartData = {
  data: Array<number>;
  index: number;
};

type UpdatedCpuUtilisationResponse = {
  labels: Array<string>;
  dataset: Array<CpuUtilisationChartData>;
};

export const createCpuUtilisationChart = async (canvas: HTMLCanvasElement) => {
  const cpuUtilisation = (await invoke(
    "get_initial_cpu_usage"
  )) as Array<InitialCpuUtilisationChartData>;

  const datasets = cpuUtilisation.map((dataset) => {
    return {
      label: dataset.label,
      data: dataset.data,
      fill: false,
      borderColor: dataset.background_color,
      tension: 0.1,
    };
  });

  return new Chart(canvas, {
    type: "line",
    data: {
      labels: [""],
      datasets,
    },
    options: {
      scales: {
        y: {
          suggestedMin: 0,
          suggestedMax: 100,
        },
      },
    },
  });
};

export const updateCpuUtilisationChart = async (
  chart: Chart<"line", number[], string>
) => {
  await invoke("get_cpu_usage");

  await listen<string>("update-cpu-usage", (event) => {
    const updatedCpuUtilisation = JSON.parse(
      event.payload
    ) as UpdatedCpuUtilisationResponse;

    chart.data.labels = updatedCpuUtilisation.labels;
    chart.data.datasets.forEach((dataset, index) => {
      dataset.data = updatedCpuUtilisation.dataset[index].data;
    });

    chart.update();
  });
};
