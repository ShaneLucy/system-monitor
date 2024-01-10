export type SystemStats = {
  cpu_temp: number;
  memory_used: number;
  swap_used: number;
};

export type StaticSystemStats = {
  memory_total: number;
  swap_total: number;
};

export type GpuStats = {
  gpu_utilisation: number;
  memory_utilisation: number;
  free_memory: number;
  memory_used: number;
  fan_speed: number;
  temperature: number;
  power_usage: number;
};

export type StaticGpuStats = {
  name: string;
  id: number;
  total_memory: number;
};

export type ChartData = {
  labels: string[];
  datasets: Array<{
    label: string;
    data: number[];
    background_color: string[];
    hover_offset: number;
  }>;
};
