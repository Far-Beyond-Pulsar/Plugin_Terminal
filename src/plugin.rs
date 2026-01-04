//! Terminal Plugin
//! 
//! Provides a terminal emulator with statusbar integration

use plugin_editor_api::*;

pub mod mappings;
pub mod rendering;
pub mod terminal_core;
pub mod terminal_drawer;
pub mod terminal_element;

pub use terminal_core::{Terminal, TERMINAL_CONTEXT, Event as TerminalEvent, init};
pub use terminal_drawer::TerminalDrawer;
pub use terminal_element::TerminalElement;

#[derive(Default)]
struct TerminalPlugin;

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
                "terminal.toggle",
                ui::IconName::SquareTerminal,
                "Toggle Terminal",
                StatusbarPosition::Left,
                StatusbarAction::Custom,
            )
            .with_priority(50)
            .with_callback(toggle_terminal),
        ]
    }

    fn file_types(&self) -> Vec<FileTypeDefinition> {
        vec![]
    }

    fn editors(&self) -> Vec<EditorMetadata> {
        vec![]
    }

    fn create_editor(
        &self,
        _editor_id: EditorId,
        _file_path: std::path::PathBuf,
        _window: &mut Window,
        _cx: &mut App,
        _logger: &EditorLogger,
    ) -> Result<(std::sync::Arc<dyn ui::dock::PanelView>, Box<dyn EditorInstance>), PluginError> {
        Err(PluginError::Other {
            message: "Terminal plugin does not support editor instances".into(),
        })
    }
}

fn toggle_terminal(_window: &mut Window, cx: &mut App) {
    use gpui::*;
    
    let _ = cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: Point {
                    x: px(150.0),
                    y: px(150.0),
                },
                size: size(px(1000.0), px(700.0)),
            })),
            titlebar: Some(TitlebarOptions {
                title: None,
                appears_transparent: true,
                traffic_light_position: None,
            }),
            kind: WindowKind::Normal,
            is_resizable: true,
            window_decorations: Some(WindowDecorations::Client),
            window_min_size: Some(Size {
                width: px(600.),
                height: px(400.),
            }),
            ..Default::default()
        },
        |window, cx| {
            let terminal_drawer = cx.new(|cx| TerminalDrawer::new(window, cx));
            cx.new(|cx| ui::root::Root::new(terminal_drawer.into(), window, cx))
        },
    );
}

export_plugin!(TerminalPlugin);
