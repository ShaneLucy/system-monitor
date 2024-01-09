<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type {
    SystemStats,
    StaticSystemStats,
    GpuStats,
    StaticGpuStats,
  } from "../types";
  import { onMount } from "svelte";

  let systemStats: SystemStats;
  let staticSystemStats: StaticSystemStats;

  let gpuStats: GpuStats;
  let staticGpuStats: StaticGpuStats;

  onMount(async () => {
    staticSystemStats = await invoke("static_system_stats");
    staticGpuStats = await invoke("static_gpu_stats");
    gpuStats = await invoke("gpu_stats");
    console.log("GPU:", gpuStats);
    // while (true) {
    //   systemStats = await invoke("system_stats");
    //   gpuStats = await invoke("gpu_stats");
    //   console.log("System: ", systemStats);
    //   console.log("GPU:", gpuStats);
    // }
  });
</script>

<p>CPU Temp:</p>
<p>{systemStats?.cpu_temp}</p>
<br />
<p>Memory Total:</p>
<p>{staticSystemStats?.memory_total}</p>
<br />
<p>Memory Used:</p>
<p>{systemStats?.memory_used}</p>
<br />
<p>Swap Total:</p>
<p>{staticSystemStats?.swap_total}</p>
<br />
<p>Swap Used:</p>
<p>{systemStats?.swap_used}</p>
<br />
