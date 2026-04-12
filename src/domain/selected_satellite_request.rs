use crate::domain::{GroundStation, Satellite};
use chrono::{DateTime, Utc};


#[derive(Debug, Clone)]
pub struct SelectedSatelliteRequest{
    pub satellite: Satellite,
    pub ground_station: GroundStation,
    pub observation_time: DateTime<Utc>,
}