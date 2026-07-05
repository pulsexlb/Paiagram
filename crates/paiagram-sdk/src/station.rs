//! Station event types for mod developers.
//!
//! Binary-compatible with `paiagram_core::station::CreateNewStation`.
//! Trigger from a mod system to create a station in the Paiagram world.

use bevy_moder_sdk::event;

/// Geographic coordinates (WGS84). Binary-compatible with
/// `paiagram_core::graph::NodeCoor`.
#[derive(Clone, ::bevy_moder_sdk::serde::Serialize, ::bevy_moder_sdk::serde::Deserialize)]
#[serde(crate = "::bevy_moder_sdk::serde")]
pub struct NodeCoor {
    pub lon: f64,
    pub lat: f64,
}

/// Request the host to create a new station.
///
/// ```ignore
/// use paiagram_sdk::station::{CreateNewStation, NodeCoor};
///
/// trigger_event!(CreateNewStation, CreateNewStation {
///     name: Some("My Station".into()),
///     coor: NodeCoor { lon: 116.4, lat: 39.9 },
/// });
/// ```
#[event]
pub struct CreateNewStation {
    pub name: Option<String>,
    pub coor: NodeCoor,
}
