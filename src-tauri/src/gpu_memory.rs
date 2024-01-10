use super::chart::{ChartData, ChartDataSets};
use super::colours::COLOUR_HEX_CODES;

use nvml_wrapper::enums::device::UsedGpuMemory;
use nvml_wrapper::Device;
use sysinfo::{Pid, System};

struct ProcessMemoryUtilisation {
    pub name: String,
    pub memory_used: u64,
}

pub fn get_gpu_memory_usage(device: &Device) -> ChartData {
    

    let mut process_memory_utilisation = get_running_graphics_processes(&device);
    process_memory_utilisation.push(get_unused_memory(&device));
    process_memory_utilisation.sort_by(|a, b| a.memory_used.cmp(&b.memory_used));

    return ChartData {
        labels: build_labels(&process_memory_utilisation),
        datasets: build_chart_datasets(&process_memory_utilisation),
    };
}

fn get_running_graphics_processes(
    device: &Device
) -> Vec<ProcessMemoryUtilisation> {
    let graphics_processes = device
        .running_graphics_processes()
        .expect("error retrieving running graphics processes");

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

fn get_unused_memory(device: &Device) -> ProcessMemoryUtilisation {
    return ProcessMemoryUtilisation {
        name: String::from("Unused Memory"),
        memory_used: device
            .memory_info()
            .expect("error retrieving memory info")
            .free
            / 1_048_576,
    };
}

fn build_chart_datasets(
    process_memory_utilisation: &Vec<ProcessMemoryUtilisation>,
) -> Vec<ChartDataSets> {
    let mut chart_data_sets = Vec::new();
    chart_data_sets.push(ChartDataSets {
        data: process_memory_utilisation
            .iter()
            .map(|process| process.memory_used)
            .collect(),
        background_color: COLOUR_HEX_CODES[..process_memory_utilisation.len()].to_vec(),
    });

    return chart_data_sets;
}

fn build_labels(process_memory_utilisation: &Vec<ProcessMemoryUtilisation>) -> Vec<String> {
    return process_memory_utilisation
        .iter()
        .map(|process| format!("{}: {} MB", process.name, process.memory_used))
        .collect();
}
