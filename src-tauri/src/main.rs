// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod chart;
mod colours;
mod gpu;
mod gpu_memory;
mod system;

use chart::ChartData;
use gpu::{read_gpu_stats, read_static_gpu_stats, GpuStats, StaticGpuStats};
use gpu_memory::get_gpu_memory_usage;

use system::{read_static_system_stats, read_system_stats, StaticSystemStats, SystemStats};

use systemstat::{Platform, System};

use nvml_wrapper::Nvml;

#[tauri::command]
fn static_system_stats() -> StaticSystemStats {
    return read_static_system_stats(&System::new());
}

#[tauri::command]
fn system_stats() -> SystemStats {
    return read_system_stats(&System::new());
}

#[tauri::command]
fn static_gpu_stats() -> StaticGpuStats {
    let nvml = Nvml::init().expect("error initialising nvml");
    let gpu = nvml.device_by_index(0).expect("error getting gpu");
    return read_static_gpu_stats(&gpu);
}

#[tauri::command]
fn gpu_stats() -> GpuStats {
    let nvml = Nvml::init().expect("error initialising nvml");
    let gpu = nvml.device_by_index(0).expect("error getting gpu");
    return read_gpu_stats(&gpu);
}

#[tauri::command]
fn gpu_memory_stats() -> ChartData {
    let nvml = Nvml::init().expect("error initialising nvml");
    let gpu = nvml.device_by_index(0).expect("error getting gpu");
    return get_gpu_memory_usage(&gpu);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            static_system_stats,
            system_stats,
            static_gpu_stats,
            gpu_stats,
            gpu_memory_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
