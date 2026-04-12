pub mod positions;
pub use positions::ComputedPosition;

pub fn compute_position(request: &SelectedSatelliteRequest) -> ComputedPosition;