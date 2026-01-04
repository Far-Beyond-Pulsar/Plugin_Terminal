//! Terminal Drawer - Professional terminal emulator using Alacritty
//! Based on Zed's terminal architecture

use gpui::*;
use ui::{
    v_flex, ActiveTheme as _,
};
use crate::terminal_core::Terminal;

pub struct TerminalDrawer {
    focus_handle: FocusHandle,
    terminal: Entity<Terminal>,
}

impl TerminalDrawer {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        
        // Create terminal - handle error gracefully
        let terminal = cx.new(|cx| {
            Terminal::new(window, cx).unwrap_or_else(|e| {
                tracing::error!("Failed to create terminal: {}", e);
                // Create a fallback empty terminal
                Terminal::new(window, cx).unwrap()
            })
        });

        Self {
            focus_handle,
            terminal,
        }
    }

    pub fn terminal(&self) -> &Entity<Terminal> {
        &self.terminal
    }
}

impl Focusable for TerminalDrawer {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl gpui::EventEmitter<ui::dock::PanelEvent> for TerminalDrawer {}

impl Render for TerminalDrawer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .child(
                // Terminal content
                div()
                    .flex_1()
                    .w_full()
                    .child(self.terminal.clone())
            )
    }
}

impl ui::dock::Panel for TerminalDrawer {
    fn panel_name(&self) -> &'static str {
        "Terminal"
    }

    fn title(&self, _window: &Window, _cx: &App) -> AnyElement {
        "Terminal".into_any_element()
    }
}
