//! Host-side mod UI support.
//!
//! Defines the registered component types (matching `paiagram-sdk::ui`),
//! a per-frame entity index, and functions to collect/render a mod's
//! UI tree to egui.

#![cfg(not(target_arch = "wasm32"))]

use bevy::prelude::*;
use bevy_moder_runtime::register_component;

/// Registered component — mirrors `paiagram_sdk::ui::ModTab`.
#[register_component]
#[derive(Component, Debug, Clone)]
pub struct ModTab {
    pub tab_id: String,
    pub title: String,
}

/// ContainerLayout must be Serialize/Deserialize + Reflect for
/// #[register_component] on Container to compile.
#[derive(
    ::bevy_moder_runtime::serde::Serialize,
    ::bevy_moder_runtime::serde::Deserialize,
    Debug, Clone, Copy, PartialEq, Eq, Default, Reflect,
)]
#[serde(crate = "::bevy_moder_runtime::serde")]
#[reflect(Default)]
pub enum ContainerLayout {
    #[default]
    Column,
    Row,
    ScrollArea,
}

#[register_component]
#[derive(Component, Debug, Clone)]
pub struct Container {
    pub node_id: String,
    pub tab_id: String,
    pub parent_node_id: Option<String>,
    pub layout: ContainerLayout,
}

#[register_component]
#[derive(Component, Debug, Clone)]
pub struct Label {
    pub tab_id: String,
    pub node_id: String,
    pub parent_node_id: Option<String>,
    pub text: String,
}

#[register_component]
#[derive(Component, Debug, Clone)]
pub struct Button {
    pub tab_id: String,
    pub node_id: String,
    pub parent_node_id: Option<String>,
    pub text: String,
    pub clicked: bool,
}

#[register_component]
#[derive(Component, Debug, Clone)]
pub struct Checkbox {
    pub tab_id: String,
    pub node_id: String,
    pub parent_node_id: Option<String>,
    pub text: String,
    pub checked: bool,
}

#[register_component]
#[derive(Component, Debug, Clone)]
pub struct TextEdit {
    pub tab_id: String,
    pub node_id: String,
    pub parent_node_id: Option<String>,
    pub text: String,
    pub hint: String,
}

#[register_component]
#[derive(Component, Debug, Clone)]
pub struct CollapsingHeader {
    pub tab_id: String,
    pub node_id: String,
    pub parent_node_id: Option<String>,
    pub text: String,
    pub open: bool,
}

#[register_component]
#[derive(Component, Debug, Clone)]
pub struct Separator {
    pub tab_id: String,
    pub node_id: String,
    pub parent_node_id: Option<String>,
}

#[register_component]
#[derive(Component, Debug, Clone)]
pub struct Slider {
    pub tab_id: String,
    pub node_id: String,
    pub parent_node_id: Option<String>,
    pub text: String,
    pub min: f64,
    pub max: f64,
    pub value: f64,
}

/// UI tree collected from the ECS — no borrows held during egui rendering.
pub(crate) struct UiTree {
    pub(crate) roots: Vec<UiNodeData>,
}

pub(crate) struct UiNodeData {
    pub(crate) entity: Entity,
    pub(crate) kind: UiNodeKind,
    pub(crate) children: Vec<UiNodeData>,
}

pub(crate) enum UiNodeKind {
    Column,
    Row,
    ScrollArea,
    Label(String),
    Button { text: String, clicked: bool },
    Checkbox { text: String, checked: bool },
    TextEdit { text: String, hint: String },
    CollapsingHeader { text: String, open: bool },
    Separator,
    Slider { text: String, min: f64, max: f64, value: f64 },
}

/// An interaction the user performed (collected during rendering).
pub(crate) enum Interaction {
    Clicked(Entity),
    Checked(Entity, bool),
    TextChanged(Entity, String),
    SliderChanged(Entity, f64),
}

struct FlatNode {
    entity: Entity,
    node_id: String,
    parent_node_id: Option<String>,
    kind: UiNodeKind,
}

pub(crate) fn collect_ui_tree(world: &mut World, tab_id: &str) -> Option<UiTree> {
    let mut flat = Vec::new();

    let mut containers = world.query::<(Entity, &Container)>();
    for (entity, c) in containers.iter(world) {
        if c.tab_id == tab_id {
            let layout = match c.layout {
                ContainerLayout::Column => UiNodeKind::Column,
                ContainerLayout::Row => UiNodeKind::Row,
                ContainerLayout::ScrollArea => UiNodeKind::ScrollArea,
            };
            flat.push(FlatNode {
                entity,
                node_id: c.node_id.clone(),
                parent_node_id: c.parent_node_id.clone(),
                kind: layout,
            });
        }
    }

    let mut labels = world.query::<(Entity, &Label)>();
    for (entity, l) in labels.iter(world) {
        if l.tab_id == tab_id {
            flat.push(FlatNode {
                entity,
                node_id: l.node_id.clone(),
                parent_node_id: l.parent_node_id.clone(),
                kind: UiNodeKind::Label(l.text.clone()),
            });
        }
    }

    let mut buttons = world.query::<(Entity, &Button)>();
    for (entity, b) in buttons.iter(world) {
        if b.tab_id == tab_id {
            flat.push(FlatNode {
                entity,
                node_id: b.node_id.clone(),
                parent_node_id: b.parent_node_id.clone(),
                kind: UiNodeKind::Button {
                    text: b.text.clone(),
                    clicked: b.clicked,
                },
            });
        }
    }

    let mut checks = world.query::<(Entity, &Checkbox)>();
    for (entity, c) in checks.iter(world) {
        if c.tab_id == tab_id {
            flat.push(FlatNode {
                entity,
                node_id: c.node_id.clone(),
                parent_node_id: c.parent_node_id.clone(),
                kind: UiNodeKind::Checkbox {
                    text: c.text.clone(),
                    checked: c.checked,
                },
            });
        }
    }

    let mut edits = world.query::<(Entity, &TextEdit)>();
    for (entity, t) in edits.iter(world) {
        if t.tab_id == tab_id {
            flat.push(FlatNode {
                entity,
                node_id: t.node_id.clone(),
                parent_node_id: t.parent_node_id.clone(),
                kind: UiNodeKind::TextEdit {
                    text: t.text.clone(),
                    hint: t.hint.clone(),
                },
            });
        }
    }

    let mut headers = world.query::<(Entity, &CollapsingHeader)>();
    for (entity, h) in headers.iter(world) {
        if h.tab_id == tab_id {
            flat.push(FlatNode {
                entity,
                node_id: h.node_id.clone(),
                parent_node_id: h.parent_node_id.clone(),
                kind: UiNodeKind::CollapsingHeader {
                    text: h.text.clone(),
                    open: h.open,
                },
            });
        }
    }

    let mut separators = world.query::<(Entity, &Separator)>();
    for (entity, s) in separators.iter(world) {
        if s.tab_id == tab_id {
            flat.push(FlatNode {
                entity,
                node_id: s.node_id.clone(),
                parent_node_id: s.parent_node_id.clone(),
                kind: UiNodeKind::Separator,
            });
        }
    }

    let mut sliders = world.query::<(Entity, &Slider)>();
    for (entity, s) in sliders.iter(world) {
        if s.tab_id == tab_id {
            flat.push(FlatNode {
                entity,
                node_id: s.node_id.clone(),
                parent_node_id: s.parent_node_id.clone(),
                kind: UiNodeKind::Slider {
                    text: s.text.clone(),
                    min: s.min,
                    max: s.max,
                    value: s.value,
                },
            });
        }
    }

    if flat.is_empty() {
        return None;
    }

    build_tree(&flat)
}

fn build_tree(flat: &[FlatNode]) -> Option<UiTree> {
    use std::collections::HashMap;

    // Index by node_id for fast child lookup.
    // Group children by parent_node_id.
    let mut children_of: HashMap<Option<&str>, Vec<&FlatNode>> = HashMap::new();
    for node in flat {
        children_of
            .entry(node.parent_node_id.as_deref())
            .or_default()
            .push(node);
    }

    fn build_subtree(
        node: &FlatNode,
        children_of: &HashMap<Option<&str>, Vec<&FlatNode>>,
    ) -> UiNodeData {
        let child_nodes = children_of
            .get(&Some(node.node_id.as_str()))
            .map(|kids| {
                kids.iter()
                    .map(|c| build_subtree(c, children_of))
                    .collect()
            })
            .unwrap_or_default();

        UiNodeData {
            entity: node.entity,
            kind: match &node.kind {
                // Clone all variant data since we need owned data.
                UiNodeKind::Column => UiNodeKind::Column,
                UiNodeKind::Row => UiNodeKind::Row,
                UiNodeKind::ScrollArea => UiNodeKind::ScrollArea,
                UiNodeKind::Label(s) => UiNodeKind::Label(s.clone()),
                UiNodeKind::Button { text, clicked } => {
                    UiNodeKind::Button {
                        text: text.clone(),
                        clicked: *clicked,
                    }
                }
                UiNodeKind::Checkbox { text, checked } => {
                    UiNodeKind::Checkbox {
                        text: text.clone(),
                        checked: *checked,
                    }
                }
                UiNodeKind::TextEdit { text, hint } => {
                    UiNodeKind::TextEdit {
                        text: text.clone(),
                        hint: hint.clone(),
                    }
                }
                UiNodeKind::CollapsingHeader { text, open } => {
                    UiNodeKind::CollapsingHeader {
                        text: text.clone(),
                        open: *open,
                    }
                }
                UiNodeKind::Separator => UiNodeKind::Separator,
                UiNodeKind::Slider {
                    text,
                    min,
                    max,
                    value,
                } => UiNodeKind::Slider {
                    text: text.clone(),
                    min: *min,
                    max: *max,
                    value: *value,
                },
            },
            children: child_nodes,
        }
    }

    let roots: Vec<UiNodeData> = children_of
        .get(&None)
        .map(|nodes| {
            nodes
                .iter()
                .map(|n| build_subtree(n, &children_of))
                .collect()
        })
        .unwrap_or_default();

    if roots.is_empty() {
        return None;
    }

    Some(UiTree { roots })
}

/// Render a mod's UI tree and collect user interactions.
pub(crate) fn render_ui_tree(tree: &mut UiTree, ui: &mut egui::Ui) -> Vec<Interaction> {
    let mut interactions = Vec::new();
    for root in &mut tree.roots {
        render_node(root, ui, &mut interactions);
    }
    interactions
}

fn render_node(
    node: &mut UiNodeData,
    ui: &mut egui::Ui,
    interactions: &mut Vec<Interaction>,
) {
    match &mut node.kind {
        UiNodeKind::Column => {
            ui.vertical(|ui| {
                for child in &mut node.children {
                    render_node(child, ui, interactions);
                }
            });
        }
        UiNodeKind::Row => {
            ui.horizontal(|ui| {
                for child in &mut node.children {
                    render_node(child, ui, interactions);
                }
            });
        }
        UiNodeKind::ScrollArea => {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for child in &mut node.children {
                    render_node(child, ui, interactions);
                }
            });
        }
        UiNodeKind::Label(text) => {
            ui.label(text.as_str());
        }
        UiNodeKind::Button { text, clicked } => {
            if ui.button(text.as_str()).clicked() {
                *clicked = true;
                interactions.push(Interaction::Clicked(node.entity));
            }
        }
        UiNodeKind::Checkbox { text, checked } => {
            let before = *checked;
            ui.checkbox(checked, text.as_str());
            if *checked != before {
                interactions.push(Interaction::Checked(node.entity, *checked));
            }
        }
        UiNodeKind::TextEdit { text, hint } => {
            let before = text.clone();
            let mut te = egui::TextEdit::singleline(text);
            if !hint.is_empty() {
                te = te.hint_text(hint.as_str());
            }
            ui.add(te);
            if *text != before {
                interactions.push(Interaction::TextChanged(node.entity, text.clone()));
            }
        }
        UiNodeKind::CollapsingHeader { text, open } => {
            let before = *open;
            egui::CollapsingHeader::new(text.as_str())
                .default_open(*open)
                .show(ui, |ui| {
                    for child in &mut node.children {
                        render_node(child, ui, interactions);
                    }
                });
            // CollapsingHeader doesn't expose open/close state directly;
            // we keep the previous value for now.
            *open = before;
        }
        UiNodeKind::Separator => {
            ui.separator();
        }
        UiNodeKind::Slider {
            text,
            min,
            max,
            value,
        } => {
            let before = *value;
            ui.add(
                egui::Slider::new(value, (*min)..=(*max))
                    .text(text.as_str()),
            );
            if (*value - before).abs() > f64::EPSILON {
                interactions.push(Interaction::SliderChanged(node.entity, *value));
            }
        }
    }
}

/// Apply collected interactions to world entities.
/// Call this AFTER rendering, with `&mut World`.
pub(crate) fn apply_interactions(world: &mut World, interactions: Vec<Interaction>) {
    for interaction in interactions {
        match interaction {
            Interaction::Clicked(entity) => {
                if let Some(mut b) = world.entity_mut(entity).get_mut::<Button>() {
                    b.clicked = true;
                }
            }
            Interaction::Checked(entity, checked) => {
                if let Some(mut c) = world.entity_mut(entity).get_mut::<Checkbox>() {
                    c.checked = checked;
                }
            }
            Interaction::TextChanged(entity, text) => {
                if let Some(mut t) = world.entity_mut(entity).get_mut::<TextEdit>() {
                    t.text = text;
                }
            }
            Interaction::SliderChanged(entity, value) => {
                if let Some(mut s) = world.entity_mut(entity).get_mut::<Slider>() {
                    s.value = value;
                }
            }
        }
    }
}
