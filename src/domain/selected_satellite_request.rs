use crate::domain::{GroundStation, Satellite};
use chrono::{DateTime, Utc};


#[derive(Debug, Clone)]
pub struct SatelliteSelection{
    pub satellite: Satellite,
    pub ground_station: GroundStation,
    pub observation_time: DateTime<Utc>,
}