//! Widget compatibility layer for gradual migration to ratatui
//!
//! This module provides helper functions to render various widgets
//! with conditional compilation, allowing the use of either helix_tui
//! or ratatui backends based on the feature flag.

use helix_view::graphics::Rect;
use tui::buffer::Buffer as Surface;

use tui::compat::ratatui_compat::render_paragraph_ref;

/// Render a Block widget with compatibility support
pub fn render_block_widget(block: tui::widgets::Block<'_>, area: Rect, surface: &mut Surface) {
    use tui::widgets::Widget;
    block.render(area, surface);
}

/// Render a Paragraph widget with compatibility support
pub fn render_paragraph_widget(
    text: &tui::text::Text<'_>,
    wrap: Option<tui::widgets::Wrap>,
    area: Rect,
    surface: &mut Surface,
) {
    render_paragraph_ref(text, wrap, area, surface);
}

/// Get inner area from Block with compatibility support
pub fn get_block_inner_area(block: &tui::widgets::Block<'_>, area: Rect) -> Rect {
    block.inner(area)
}

/// Create a bordered Block with compatibility support
pub fn bordered_block() -> tui::widgets::Block<'static> {
    tui::widgets::Block::bordered()
}
