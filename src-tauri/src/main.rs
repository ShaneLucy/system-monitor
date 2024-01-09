// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod gpu;
mod system;

use gpu::{read_gpu_stats, read_static_gpu_stats, GpuStats, StaticGpuStats};
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            static_system_stats,
            system_stats,
            static_gpu_stats,
            gpu_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
