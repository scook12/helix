//! Compatibility helper for Table widget migration from helix_tui to ratatui
//!
//! This module provides a unified interface for rendering tables that works
//! with both the legacy helix_tui and the new ratatui backend.

use helix_view::graphics::Rect;
use tui::buffer::Buffer as Surface;
use tui::widgets::{TableState, Table};

/// Render a table widget using the appropriate backend based on feature flags
///
/// This function abstracts the rendering logic to work with both helix_tui
/// and ratatui Table widgets, handling the conversion and buffer management
/// automatically.
pub fn render_table<'a>(
    table: Table<'a>,
    area: Rect,
    surface: &mut Surface,
    state: &mut TableState,
    _truncate_start: bool,
) {
    #[cfg(not(feature = "ratatui-migration"))]
    {
        // Use the original helix_tui Table rendering
        table.render_table(area, surface, state, _truncate_start);
    }
    
    #[cfg(feature = "ratatui-migration")]
    {
        // Convert to ratatui and render
        let ratatui_table = table.to_ratatui_table();
        let mut ratatui_state = tui::compat::ratatui_compat::convert_table_state(state);
        
        // Render with ratatui
        use ratatui::widgets::StatefulWidget;
        let ratatui_area = tui::compat::ratatui_compat::convert_rect(area);
        let mut ratatui_buffer = ratatui::buffer::Buffer::empty(ratatui_area);
        ratatui_table.render(ratatui_area, &mut ratatui_buffer, &mut ratatui_state);
        
        // Copy back to helix buffer
        for y in ratatui_area.top()..ratatui_area.bottom() {
            for x in ratatui_area.left()..ratatui_area.right() {
                let ratatui_cell = ratatui_buffer.get(x, y);
                if let Some(helix_cell_pos) = surface.get_mut(x, y) {
                    let mut helix_cell = tui::compat::ratatui_compat::convert_cell_back(ratatui_cell);
                    
                    // If the original helix cell has a background but the ratatui cell doesn't,
                    // preserve the original background to maintain popup styling
                    if helix_cell.bg == helix_view::graphics::Color::Reset && helix_cell_pos.bg != helix_view::graphics::Color::Reset {
                        helix_cell.bg = helix_cell_pos.bg;
                    }
                    
                    *helix_cell_pos = helix_cell;
                }
            }
        }
        
        // Update state
        tui::compat::ratatui_compat::update_table_state_from_ratatui(state, &ratatui_state);
    }
}

/// Helper to create a TableState with the given parameters
#[inline]
pub fn table_state(offset: usize, selected: Option<usize>) -> TableState {
    TableState { offset, selected }
}
