#[derive(Debug, Clone)]
pub struct Satellite {
    pub name: String,
    pub line_1: String,
    pub line_2: String,
}

#[derive(Debug, Clone)]
pub struct GroundStation {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}
