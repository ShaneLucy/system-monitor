use super::chart::{ChartData, ChartDataSets};

use nvml_wrapper::enum_wrappers::device::TemperatureSensor;

use nvml_wrapper::enums::device::UsedGpuMemory;
use nvml_wrapper::struct_wrappers::device::*;
use sysinfo::{Pid, System};

#[derive(serde::Serialize)]
pub struct GpuStats {
    pub gpu_utilisation: u32,
    pub memory_utilisation: u32,
    pub free_memory: u64,
    pub memory_used: u64,
    pub fan_speed: u32,
    pub temperature: u32,
    pub running_graphics_processes: Vec<RunningGraphicsProcess>,
    pub power_usage: u32,
}

#[derive(serde::Serialize)]
pub struct StaticGpuStats {
    pub name: String,
    pub id: u32,
    pub total_memory: u64,
}

#[derive(serde::Serialize)]
pub struct GpuMemoryUsage {
    pub free_memory: u64,
    pub used_memory: u64,
}

#[derive(serde::Serialize)]
pub struct RunningGraphicsProcess {
    pub name: String,
    pub memory_used: u64,
}

pub fn read_gpu_stats(device: &nvml_wrapper::Device) -> GpuStats {
    return GpuStats {
        gpu_utilisation: device
            .utilization_rates()
            .expect("error retrieving gpu utilisation")
            .gpu,
        memory_utilisation: device
            .utilization_rates()
            .expect("error retrieving gpu utilisation")
            .memory,
        free_memory: device
            .memory_info()
            .expect("error retrieving gpu memory info")
            .free,
        memory_used: device
            .memory_info()
            .expect("error retrieving gpu memory info")
            .used,
        fan_speed: device.fan_speed(0).expect("error retrieving gpu fan speed"), // Currently only take one fan, will add more fan readings,
        temperature: device
            .temperature(TemperatureSensor::Gpu)
            .expect("error retrieving device temperature"),
        running_graphics_processes: get_running_graphics_processes(
            device
                .running_graphics_processes()
                .expect("error retrieving running gpu processes"),
        ),
        power_usage: device
            .power_usage()
            .expect("error retrieving device power usage"),
    };
}

pub fn read_static_gpu_stats(device: &nvml_wrapper::Device) -> StaticGpuStats {
    return StaticGpuStats {
        name: device.name().expect("error retrieving device name"),
        id: device.index().expect("error retrieving device index"),
        total_memory: device
            .memory_info()
            .expect("error retrieving memory info")
            .total,
    };
}

pub fn get_gpu_memory_usage(device: &nvml_wrapper::Device) -> GpuMemoryUsage {
    let memory_info = device.memory_info().expect("error retrieving memory info");

    return GpuMemoryUsage {
        free_memory: memory_info.free / 1_048_576,
        used_memory: memory_info.used / 1_048_576,
    };
}

pub fn get_running_graphics_processes(
    graphics_processes: Vec<ProcessInfo>,
) -> Vec<RunningGraphicsProcess> {
    let mut system = System::new();
    system.refresh_all();

    let mut all_running_graphics_process: Vec<RunningGraphicsProcess> = Vec::new();
    for ele in graphics_processes {
        let used_mem = match ele.used_gpu_memory {
            UsedGpuMemory::Used(used) => used,
            _ => 0,
        };
        let running_graphics_process = RunningGraphicsProcess {
            name: system
                .process(Pid::from_u32(ele.pid))
                .expect("error retrieving process id")
                .name()
                .to_string(),
            memory_used: used_mem / 1_048_576,
        };

        all_running_graphics_process.push(running_graphics_process);
    }

    all_running_graphics_process.sort_by(|a, b| b.memory_used.cmp(&a.memory_used));
    return all_running_graphics_process;
}
