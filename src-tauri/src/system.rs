use systemstat::{saturating_sub_bytes, Platform};

use systemstat::platform::linux::PlatformImpl;

// use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(serde::Serialize)]
pub struct SystemStats {
    pub cpu_temp: f32,
    pub memory_used: u64,
    pub swap_used: u64,
}

#[derive(serde::Serialize)]
pub struct StaticSystemStats {
    pub memory_total: u64,
    pub swap_total: u64,
}

pub fn read_system_stats(system: &PlatformImpl) -> SystemStats {
    let memory = system.memory().expect("error accessing system memory info");
    let swap = system.swap().expect("error accessing system swap info");

    return SystemStats {
        memory_used: saturating_sub_bytes(memory.total, memory.free).as_u64(),
        swap_used: saturating_sub_bytes(swap.total, swap.free).as_u64(),
        cpu_temp: system.cpu_temp().expect("error retrieving cpu temperature"),
    };
}

pub fn read_static_system_stats(system: &PlatformImpl) -> StaticSystemStats {
    let memory = system.memory().expect("error accessing system memory info");
    let swap = system.swap().expect("error accessing system swap info");

    return StaticSystemStats {
        memory_total: memory.total.as_u64(),
        swap_total: swap.total.as_u64(),
    };
}
