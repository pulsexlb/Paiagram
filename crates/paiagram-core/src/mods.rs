//! WASM mod management for Paiagram.
//!
//! Discovers `.wasm` files in the `mods/` directory next to the executable
//! and exchanges mod actions (load / unload) between the Settings UI and
//! the Bevy schedule via [`ModActionQueue`] + an exclusive system.

use bevy::prelude::*;
use std::collections::HashMap;

/// A single discovered WASM mod on disk.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug)]
pub struct ModEntry {
    /// Full filesystem path to the `.wasm` file.
    pub path: String,
}

/// Resource tracking all discovered WASM mod files.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Resource, Default)]
pub struct AvailableMods {
    pub mods: Vec<ModEntry>,
}

/// path to actual mod name mapping, updated when a mod is loaded.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Resource, Default)]
pub struct ModNameRegistry {
    pub path_to_name: HashMap<String, String>,
}

/// Actions queued from the Settings UI.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Resource, Default)]
pub struct ModActionQueue {
    items: Vec<ModAction>,
}

#[cfg(not(target_arch = "wasm32"))]
pub enum ModAction {
    Load(String),
    Unload(String),
}

#[cfg(not(target_arch = "wasm32"))]
impl ModActionQueue {
    pub fn push(&mut self, action: ModAction) {
        self.items.push(action);
    }
}

/// Integrates WASM mod support into Paiagram.
///
/// On native targets adds `bevy_moder_runtime::WasmModPlugin`, the
/// [`AvailableMods`] / [`ModActionQueue`] / [`ModNameRegistry`] resources,
/// a startup system that scans the `mods/` directory, and an exclusive
/// system that processes queued load/unload actions.
///
/// On WASM targets the plugin is a no-op.
pub struct PaiagramModsPlugin;

impl Plugin for PaiagramModsPlugin {
    #[cfg(not(target_arch = "wasm32"))]
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_moder_runtime::WasmModPlugin::default());
        app.init_resource::<AvailableMods>();
        app.init_resource::<ModActionQueue>();
        app.init_resource::<ModNameRegistry>();
        app.add_systems(Startup, scan_mods_directory);
        app.add_systems(Update, process_mod_actions);
    }

    #[cfg(target_arch = "wasm32")]
    fn build(&self, _app: &mut App) {}
}

#[cfg(not(target_arch = "wasm32"))]
fn scan_mods_directory(mut available: ResMut<AvailableMods>) {
    fn mods_directory() -> Option<std::path::PathBuf> {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                let d = parent.join("mods");
                if d.is_dir() {
                    return Some(d);
                }
            }
        }
        let d = std::path::PathBuf::from("mods");
        if d.is_dir() {
            return Some(d);
        }
        None
    }

    let Some(dir) = mods_directory() else {
        return;
    };

    let mut mods = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "wasm") {
                mods.push(ModEntry {
                    path: path.to_string_lossy().to_string(),
                });
            }
        }
    }
    mods.sort_by(|a, b| a.path.cmp(&b.path));
    available.mods = mods;
}

#[cfg(not(target_arch = "wasm32"))]
fn process_mod_actions(world: &mut World) {
    let items = {
        let mut q = world.resource_mut::<ModActionQueue>();
        std::mem::take(&mut q.items)
    };
    if items.is_empty() {
        return;
    }

    let mut unloads = Vec::new();
    let mut loads = Vec::new();
    for a in items {
        match a {
            ModAction::Load(p) => loads.push(p),
            ModAction::Unload(name) => unloads.push(name),
        }
    }

    // Unload mods
    for name in &unloads {
        if let Err(e) = bevy_moder_runtime::unload_single_mod(world, name) {
            bevy::log::error!("Failed to unload mod '{}': {}", name, e);
        } else {
            bevy::log::info!("Unloaded mod '{}'", name);
        }
    }
    {
        let mut registry = world.resource_mut::<ModNameRegistry>();
        registry.path_to_name.retain(|_, v| !unloads.contains(v));
    }

    // Load mods
    for path in &loads {
        match bevy_moder_runtime::ModLoader::load_sync(world, path) {
            Ok(name) => {
                bevy::log::info!("Loaded mod '{}' from {}", name, path);
                let mut registry = world.resource_mut::<ModNameRegistry>();
                registry.path_to_name.insert(path.clone(), name);
            }
            Err(e) => {
                bevy::log::error!("Failed to load mod '{}': {}", path, e);
            }
        }
    }
}
