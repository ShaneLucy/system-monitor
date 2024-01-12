use super::colours::COLOUR_HEX_CODES;
use sysinfo::{Cpu, CpuRefreshKind, RefreshKind, System};

use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Manager};

#[derive(Clone, serde::Serialize)]
pub struct InitialCpuUtilisationChartData {
    pub data: Vec<f32>,
    pub background_color: String,
    pub index: usize,
    pub label: String,
}

#[derive(Clone, serde::Serialize)]
struct CpuUtilisationChartData {
    pub data: Vec<f32>,
    pub index: usize,
}

#[derive(Clone, serde::Serialize)]
struct UpdatedCpuUtilisationResponse {
    pub labels: Vec<String>,
    pub dataset: Vec<CpuUtilisationChartData>,
}

#[tauri::command]
pub fn get_initial_cpu_usage() -> Vec<InitialCpuUtilisationChartData> {
    let mut system =
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));

    system.refresh_cpu();
    return build_initial_chart_data(system.cpus());
}

#[tauri::command]
pub fn get_cpu_usage(app: AppHandle) {
    let mut system =
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));

    system.refresh_cpu();

    let mut chart_data = UpdatedCpuUtilisationResponse {
        labels: Vec::new(),
        dataset: Vec::new(),
    };

    for (index, cpu) in system.cpus().iter().enumerate() {
        chart_data.dataset.push(CpuUtilisationChartData {
            data: Vec::new(),
            index: index,
        });
    }

    thread::spawn(move || loop {
        system.refresh_cpu();
        thread::sleep(Duration::from_millis(100)); // 500 seems ok
        //  if(chart_data.dataset[0].data.len() > chart_data.labels.len()){
        chart_data.labels.push(String::from(""));
        // }
        for (i, cpu) in system.cpus().iter().enumerate() {
            chart_data.dataset[i].data.push(cpu.cpu_usage());

            // if (chart_data.dataset[i].data.len() > 50) {
            //     chart_data.dataset[i].data.remove(0);
            // }
        }

       
        if (chart_data.labels.len() > 50) {
            chart_data.labels.remove(0);
        }

        let string_rep =
            serde_json::to_string(&chart_data).expect("error serialising chart data to json");
        print!("{}", string_rep);
        app.emit_all("update-cpu-usage", string_rep).unwrap();
    });
}

fn build_initial_chart_data(cpus: &[Cpu]) -> Vec<InitialCpuUtilisationChartData> {
    let mut data: Vec<f32> = Vec::new();
    data.push(0.0);
    let mut chart_data: Vec<InitialCpuUtilisationChartData> = Vec::new();
    for (index, cpu) in cpus.iter().enumerate() {
        chart_data.push(InitialCpuUtilisationChartData {
            data: data.clone(),
            background_color: COLOUR_HEX_CODES
                .get(index)
                .expect("Error setting background colour for cpu")
                .to_string(),
            index: index,
            label: String::from(cpu.name()),
        });
    }

    chart_data.sort_by(|a, b| a.index.cmp(&b.index));
    return chart_data;
}
