use nvml_wrapper::enum_wrappers::device::TemperatureSensor;


#[derive(serde::Serialize)]
pub struct GpuStats {
    pub gpu_utilisation: u32,
    pub memory_utilisation: u32,
    pub fan_speed: u32,
    pub temperature: u32,
    pub power_usage: u32,
}

#[derive(serde::Serialize)]
pub struct StaticGpuStats {
    pub name: String,
    pub id: u32,
    pub total_memory: u64,
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
        fan_speed: device.fan_speed(0).expect("error retrieving gpu fan speed"), // Currently only take one fan, will add more fan readings,
        temperature: device
            .temperature(TemperatureSensor::Gpu)
            .expect("error retrieving device temperature"),
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

