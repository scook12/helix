//! Widget compatibility layer for gradual migration to ratatui
//!
//! This module provides helper functions to render various widgets
//! with conditional compilation, allowing the use of either helix_tui
//! or ratatui backends based on the feature flag.

use tui::buffer::Buffer as Surface;
use helix_view::graphics::Rect;

#[cfg(feature = "ratatui-migration")]
use tui::compat::ratatui_compat::{render_block, render_paragraph_ref};

#[cfg(not(feature = "ratatui-migration"))]
use tui::widgets::{Block, Paragraph, Widget, Wrap};

/// Render a Block widget with compatibility support
pub fn render_block_widget(block: tui::widgets::Block<'_>, area: Rect, surface: &mut Surface) {
    #[cfg(not(feature = "ratatui-migration"))]
    {
        block.render(area, surface);
    }
    
    #[cfg(feature = "ratatui-migration")]
    {
        render_block(block, area, surface);
    }
}

/// Render a Paragraph widget with compatibility support
pub fn render_paragraph_widget(
    text: &tui::text::Text<'_>,
    wrap: Option<tui::widgets::Wrap>,
    area: Rect,
    surface: &mut Surface,
) {
    #[cfg(not(feature = "ratatui-migration"))]
    {
        let mut paragraph = Paragraph::new(text);
        if let Some(wrap_config) = wrap {
            paragraph = paragraph.wrap(Wrap { trim: wrap_config.trim });
        }
        paragraph.render(area, surface);
    }
    
    #[cfg(feature = "ratatui-migration")]
    {
        render_paragraph_ref(text, wrap, area, surface);
    }
}

/// Get inner area from Block with compatibility support
pub fn get_block_inner_area(block: &tui::widgets::Block<'_>, area: Rect) -> Rect {
    block.inner(area)
}

/// Create a bordered Block with compatibility support
pub fn bordered_block() -> tui::widgets::Block<'static> {
    tui::widgets::Block::bordered()
}
