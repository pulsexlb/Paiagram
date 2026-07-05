//! Mod tab widget — renders a mod's UI tree as an egui tab.
//!
//! The struct itself is always compiled (needed for `MainTab` enum serialization),
//! but the rendering body is gated to native since the mod system is unavailable
//! on WASM targets.

use std::borrow::Cow;

use bevy::ecs::entity::{EntityMapper, MapEntities};
use bevy::ecs::world::World;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

use super::Tab;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub(crate) struct ModTabWidget {
    pub(crate) tab_id: String,
    pub(crate) title: String,
}

impl Tab for ModTabWidget {
    const NAME: &'static str = "ModTab";

    fn main_display(&mut self, world: &mut World, ui: &mut Ui) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Phase 1 — collect UI tree from ECS (no egui borrow).
            let Some(mut tree) = crate::mod_ui::collect_ui_tree(world, &self.tab_id) else {
                ui.label("(no UI)");
                return;
            };

            // Phase 2 — render to egui and collect interactions.
            let interactions = crate::mod_ui::render_ui_tree(&mut tree, ui);

            // Phase 3 — apply interactions back to entities.
            if !interactions.is_empty() {
                crate::mod_ui::apply_interactions(world, interactions);
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (world, ui);
        }
    }

    fn title(&self) -> WidgetText {
        self.title.as_str().into()
    }

    fn id(&self) -> Id {
        Id::new(format!("mod-tab-{}", self.tab_id))
    }

    fn icon(&self) -> Cow<'static, str> {
        "🧩".into()
    }
}

impl MapEntities for ModTabWidget {
    fn map_entities<M: EntityMapper>(&mut self, _mapper: &mut M) {}
}
