#[derive(Clone, serde::Serialize)]
pub struct ChartDataSets {
    pub data: Vec<u64>,
    pub background_color: Vec<String>,
}

#[derive(Clone, serde::Serialize)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub datasets: Vec<ChartDataSets>,
}
