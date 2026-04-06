#[derive(Debug, Clone)]
struct Satellite {
    name: String,
    line_1: String,
    line_2: String,
}

#[derive(Debug, Clone)]
struct GroundStation {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}