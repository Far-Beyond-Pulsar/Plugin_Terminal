//! Terminal Plugin
//! 
//! Provides a terminal emulator as an editor

mod mappings;
mod rendering;
mod terminal_core;
mod terminal_drawer;
mod terminal_element;

use plugin_editor_api::*;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use gpui::*;
use ui::dock::PanelView;
use terminal_drawer::TerminalDrawer;

/// Storage for editor instances owned by the plugin
struct EditorStorage {
    panel: Arc<dyn PanelView>,
    wrapper: Box<TerminalEditorWrapper>,
}

/// The Terminal Plugin
pub struct TerminalPlugin {
    editors: Arc<Mutex<HashMap<usize, EditorStorage>>>,
    next_editor_id: Arc<Mutex<usize>>,
}

impl Default for TerminalPlugin {
    fn default() -> Self {
        Self {
            editors: Arc::new(Mutex::new(HashMap::new())),
            next_editor_id: Arc::new(Mutex::new(0)),
        }
    }
}

impl EditorPlugin for TerminalPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: PluginId::new("com.pulsar.terminal"),
            name: "Terminal".into(),
            version: "0.1.0".into(),
            author: "Pulsar".into(),
            description: "Integrated terminal emulator".into(),
        }
    }

    fn statusbar_buttons(&self) -> Vec<StatusbarButtonDefinition> {
        vec![
            StatusbarButtonDefinition::new(
                "terminal.open",
                ui::IconName::SquareTerminal,
                "Open Terminal",
                StatusbarPosition::Left,
                StatusbarAction::OpenEditor {
                    editor_id: EditorId::new("terminal-editor"),
                    file_path: None,
                },
            )
            .with_priority(50),
        ]
    }

    fn file_types(&self) -> Vec<FileTypeDefinition> {
        vec![]
    }

    fn editors(&self) -> Vec<EditorMetadata> {
        vec![EditorMetadata {
            id: EditorId::new("terminal-editor"),
            display_name: "Terminal".into(),
            supported_file_types: vec![],
        }]
    }

    fn create_editor(
        &self,
        editor_id: EditorId,
        _file_path: PathBuf,
        window: &mut Window,
        cx: &mut App,
        logger: &EditorLogger,
    ) -> Result<(Arc<dyn PanelView>, Box<dyn EditorInstance>), PluginError> {
        logger.info("Creating terminal editor");
        
        if editor_id.as_str() == "terminal-editor" {
            // Create terminal drawer
            let panel = cx.new(|cx| TerminalDrawer::new(window, cx));

            // Wrap the panel in Arc
            let panel_arc: Arc<dyn PanelView> = Arc::new(panel.clone());

            // Create the wrapper for EditorInstance
            let wrapper = Box::new(TerminalEditorWrapper {
                panel: panel.into(),
            });

            // Generate unique ID
            let id = {
                let mut next_id = self.next_editor_id.lock().unwrap();
                let id = *next_id;
                *next_id += 1;
                id
            };

            // Store Arc and Box in plugin's HashMap
            self.editors.lock().unwrap().insert(id, EditorStorage {
                panel: panel_arc.clone(),
                wrapper: wrapper.clone(),
            });

            tracing::info!("Created terminal editor instance {}", id);

            Ok((panel_arc, wrapper))
        } else {
            Err(PluginError::EditorNotFound { editor_id })
        }
    }

    fn on_load(&mut self) {
        tracing::info!("Terminal Plugin loaded");
    }

    fn on_unload(&mut self) {
        let mut editors = self.editors.lock().unwrap();
        let count = editors.len();
        editors.clear();
        tracing::info!("Terminal Plugin unloaded (cleaned up {} editors)", count);
    }
}

/// Wrapper to bridge Entity<TerminalDrawer> to EditorInstance trait
#[derive(Clone)]
pub struct TerminalEditorWrapper {
    panel: Entity<TerminalDrawer>,
}

impl EditorInstance for TerminalEditorWrapper {
    fn file_path(&self) -> &PathBuf {
        // Terminal has no file
        static EMPTY_PATH: PathBuf = PathBuf::new();
        &EMPTY_PATH
    }

    fn save(&mut self, _window: &mut Window, _cx: &mut App) -> Result<(), PluginError> {
        // Terminal doesn't save
        Ok(())
    }

    fn reload(&mut self, _window: &mut Window, _cx: &mut App) -> Result<(), PluginError> {
        // Terminal doesn't reload
        Ok(())
    }

    fn is_dirty(&self) -> bool {
        false
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

export_plugin!(TerminalPlugin);
