use super::chart::{ChartData, ChartDataSets};
use super::colours::COLOUR_HEX_CODES;

use nvml_wrapper::enums::device::UsedGpuMemory;
use nvml_wrapper::struct_wrappers::device::*;
use sysinfo::{Pid, System};

struct GpuMemoryUsage {
    pub free_memory: u64,
    pub used_memory: u64,
}


struct ProcessMemoryUtilisation {
    pub name: String,
    pub memory_used: u64,
}

pub fn get_gpu_memory_usage(device: &nvml_wrapper::Device) -> ChartData {
    let graphics_processes = device
        .running_graphics_processes()
        .expect("error retrieving running graphics processes");
    let unused_memory = ProcessMemoryUtilisation {
        name: String::from("Unused Memory"),
        memory_used: device
            .memory_info()
            .expect("error retrieving memory info")
            .free  / 1_048_576,
    };

    let mut process_memory_utilisation = get_running_graphics_processes(graphics_processes);

    process_memory_utilisation.push(unused_memory);

    process_memory_utilisation.sort_by(|a, b| a.memory_used.cmp(&b.memory_used));

    let mut chart_data_sets = Vec::new();
    chart_data_sets.push(ChartDataSets {
            data: process_memory_utilisation
                .iter()
                .map(|process| process.memory_used)
                .collect(),
            background_color: COLOUR_HEX_CODES[..process_memory_utilisation.len()].to_vec(),
        });

    return ChartData {
        labels: process_memory_utilisation
            .iter()
            .map(|process| format!("{}: {} MB", process.name, process.memory_used))
            .collect(),
        datasets: chart_data_sets
       
    };
}

fn get_running_graphics_processes(
    graphics_processes: Vec<ProcessInfo>,
) -> Vec<ProcessMemoryUtilisation> {
    let mut system = System::new();
    system.refresh_all();

    let mut all_running_graphics_process: Vec<ProcessMemoryUtilisation> = Vec::new();
    for ele in graphics_processes {
        let used_mem = match ele.used_gpu_memory {
            UsedGpuMemory::Used(used) => used,
            _ => 0,
        };
        let running_graphics_process = ProcessMemoryUtilisation {
            name: system
                .process(Pid::from_u32(ele.pid))
                .expect("error retrieving process id")
                .name()
                .to_string(),
            memory_used: used_mem / 1_048_576,
        };

        all_running_graphics_process.push(running_graphics_process);
    }

    return all_running_graphics_process;
}
