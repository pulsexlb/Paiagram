//! UI component types for mod developers.
//!
//! Mods spawn entities with these components to declare a tab and
//! populate it with UI widgets.  The host reads these entities each
//! frame and translates them into egui controls.  Interaction results
//! are written back to the component fields (e.g. `Button.clicked`),
//! which the mod reads next frame via `query_mut!`.

use bevy_moder_sdk::component;

/// Declares that this mod wants a tab in the main UI.
#[component]
pub struct ModTab {
    pub tab_id: String,
    pub title: String,
}

impl ModTab {
    /// Create a new `ModTab` with a unique tab_id for the given mod.
    ///
    /// `mod_name` should be a unique identifier for your mod (e.g. the
    /// package name from Cargo.toml).  All UI components spawned by the
    /// same mod must use the same `tab_id` to associate themselves with
    /// this tab.
    pub fn new(mod_name: &str, title: &str) -> Self {
        Self {
            tab_id: format!("{}:{}:tab", mod_name, title),
            title: title.to_owned(),
        }
    }
}

// Note: manual serde derives must use ::bevy_moder_sdk::serde (not workspace serde)
// because #[component] internally uses bevy_moder_sdk's re-exported serde_core.
#[derive(::bevy_moder_sdk::serde::Serialize, ::bevy_moder_sdk::serde::Deserialize)]
#[serde(crate = "::bevy_moder_sdk::serde")]
pub enum ContainerLayout {
    Column,
    Row,
    ScrollArea,
}

/// A layout container (column, row, scroll area).
#[component]
pub struct Container {
    pub node_id: String,
    pub tab_id: String,
    pub parent_node_id: Option<String>,
    pub layout: ContainerLayout,
}

/// Static label.
#[component]
pub struct Label {
    pub tab_id: String,
    pub node_id: String,
    pub parent_node_id: Option<String>,
    pub text: String,
}

/// Clickable button.  The host sets `clicked = true` when the user
/// presses it; the mod reads and resets it.
#[component]
pub struct Button {
    pub tab_id: String,
    pub node_id: String,
    pub parent_node_id: Option<String>,
    pub text: String,
    pub clicked: bool,
}

/// Checkbox.  The host updates `checked` when the user toggles it.
#[component]
pub struct Checkbox {
    pub tab_id: String,
    pub node_id: String,
    pub parent_node_id: Option<String>,
    pub text: String,
    pub checked: bool,
}

/// Single-line text input.
#[component]
pub struct TextEdit {
    pub tab_id: String,
    pub node_id: String,
    pub parent_node_id: Option<String>,
    pub text: String,
    pub hint: String,
}

/// Collapsible section header.
#[component]
pub struct CollapsingHeader {
    pub tab_id: String,
    pub node_id: String,
    pub parent_node_id: Option<String>,
    pub text: String,
    pub open: bool,
}

/// Separator / divider line.
#[component]
pub struct Separator {
    pub tab_id: String,
    pub node_id: String,
    pub parent_node_id: Option<String>,
}

/// Slider for a numeric value.
#[component]
pub struct Slider {
    pub tab_id: String,
    pub node_id: String,
    pub parent_node_id: Option<String>,
    pub text: String,
    pub min: f64,
    pub max: f64,
    pub value: f64,
}
