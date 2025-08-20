//! Compatibility layer for migrating from helix-tui to ratatui
//!
//! This module provides adapters and wrappers to gradually migrate
//! from the helix-tui fork to the ratatui library while maintaining
//! API compatibility with existing Helix code.

#[cfg(feature = "ratatui-migration")]
pub mod ratatui_compat {
    use helix_view::graphics::{Color, Style, Rect, CursorKind};
    
    /// Convert helix graphics Color to ratatui Color
    pub fn convert_color(color: Color) -> ratatui::style::Color {
        match color {
            Color::Reset => ratatui::style::Color::Reset,
            Color::Black => ratatui::style::Color::Black,
            Color::Red => ratatui::style::Color::Red,
            Color::Green => ratatui::style::Color::Green,
            Color::Yellow => ratatui::style::Color::Yellow,
            Color::Blue => ratatui::style::Color::Blue,
            Color::Magenta => ratatui::style::Color::Magenta,
            Color::Cyan => ratatui::style::Color::Cyan,
            Color::Gray => ratatui::style::Color::DarkGray,
            Color::LightRed => ratatui::style::Color::LightRed,
            Color::LightGreen => ratatui::style::Color::LightGreen,
            Color::LightYellow => ratatui::style::Color::LightYellow,
            Color::LightBlue => ratatui::style::Color::LightBlue,
            Color::LightMagenta => ratatui::style::Color::LightMagenta,
            Color::LightCyan => ratatui::style::Color::LightCyan,
            Color::LightGray => ratatui::style::Color::Gray,
            Color::White => ratatui::style::Color::White,
            Color::Rgb(r, g, b) => ratatui::style::Color::Rgb(r, g, b),
            Color::Indexed(i) => ratatui::style::Color::Indexed(i),
        }
    }
    
    /// Convert ratatui Color to helix graphics Color
    pub fn convert_color_back(color: ratatui::style::Color) -> Color {
        match color {
            ratatui::style::Color::Reset => Color::Reset,
            ratatui::style::Color::Black => Color::Black,
            ratatui::style::Color::Red => Color::Red,
            ratatui::style::Color::Green => Color::Green,
            ratatui::style::Color::Yellow => Color::Yellow,
            ratatui::style::Color::Blue => Color::Blue,
            ratatui::style::Color::Magenta => Color::Magenta,
            ratatui::style::Color::Cyan => Color::Cyan,
            ratatui::style::Color::DarkGray => Color::Gray,
            ratatui::style::Color::LightRed => Color::LightRed,
            ratatui::style::Color::LightGreen => Color::LightGreen,
            ratatui::style::Color::LightYellow => Color::LightYellow,
            ratatui::style::Color::LightBlue => Color::LightBlue,
            ratatui::style::Color::LightMagenta => Color::LightMagenta,
            ratatui::style::Color::LightCyan => Color::LightCyan,
            ratatui::style::Color::Gray => Color::LightGray,
            ratatui::style::Color::White => Color::White,
            ratatui::style::Color::Rgb(r, g, b) => Color::Rgb(r, g, b),
            ratatui::style::Color::Indexed(i) => Color::Indexed(i),
        }
    }
    
    /// Convert helix graphics Style to ratatui Style
    pub fn convert_style(style: Style) -> ratatui::style::Style {
        let mut ratatui_style = ratatui::style::Style::default();
        
        if let Some(fg) = style.fg {
            ratatui_style = ratatui_style.fg(convert_color(fg));
        }
        
        if let Some(bg) = style.bg {
            ratatui_style = ratatui_style.bg(convert_color(bg));
        }
        
        // Note: underline_color not available in ratatui 0.26
        // Will be handled in future versions
        
        // Convert modifiers
        ratatui_style = ratatui_style.add_modifier(convert_modifier(style.add_modifier));
        
        ratatui_style
    }
    
    /// Convert helix graphics Rect to ratatui Rect
    pub fn convert_rect(rect: Rect) -> ratatui::layout::Rect {
        ratatui::layout::Rect {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }
    
    /// Convert ratatui Rect to helix graphics Rect
    pub fn convert_rect_back(rect: ratatui::layout::Rect) -> Rect {
        Rect {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }
    
    /// Convert helix CursorKind - will be handled at backend level
    /// For now, just return the original kind for backend processing
    pub fn convert_cursor_kind(kind: CursorKind) -> CursorKind {
        // For now, pass through - backend will handle conversion
        kind
    }
    
    /// Convert helix-tui Cell to ratatui Cell
    pub fn convert_cell(cell: &crate::buffer::Cell) -> ratatui::buffer::Cell {
        let style = convert_style(cell.style());
        let mut ratatui_cell = ratatui::buffer::Cell::default();
        ratatui_cell.set_symbol(&cell.symbol);
        ratatui_cell.set_style(style);
        ratatui_cell
    }
    
    /// Convert ratatui Cell to helix-tui Cell
    pub fn convert_cell_back(cell: &ratatui::buffer::Cell) -> crate::buffer::Cell {
        use helix_view::graphics::UnderlineStyle;
        
        crate::buffer::Cell {
            symbol: cell.symbol().to_string(),
            fg: cell.style().fg.map(convert_color_back).unwrap_or(Color::Reset),
            bg: cell.style().bg.map(convert_color_back).unwrap_or(Color::Reset), 
            modifier: convert_modifier_back(cell.style().add_modifier),
            underline_color: Color::Reset, // Default fallback
            underline_style: UnderlineStyle::Reset, // Default fallback
        }
    }
    
    /// Convert helix Modifier to ratatui Modifier
    pub fn convert_modifier(modifier: helix_view::graphics::Modifier) -> ratatui::style::Modifier {
        let mut result = ratatui::style::Modifier::empty();
        
        use helix_view::graphics::Modifier;
        if modifier.contains(Modifier::BOLD) {
            result |= ratatui::style::Modifier::BOLD;
        }
        if modifier.contains(Modifier::DIM) {
            result |= ratatui::style::Modifier::DIM;
        }
        if modifier.contains(Modifier::ITALIC) {
            result |= ratatui::style::Modifier::ITALIC;
        }
        // Note: helix doesn't have UNDERLINED modifier, but has underline_style
        // This will be handled through the underline_style field conversion if needed
        if modifier.contains(Modifier::SLOW_BLINK) {
            result |= ratatui::style::Modifier::SLOW_BLINK;
        }
        if modifier.contains(Modifier::RAPID_BLINK) {
            result |= ratatui::style::Modifier::RAPID_BLINK;
        }
        if modifier.contains(Modifier::REVERSED) {
            result |= ratatui::style::Modifier::REVERSED;
        }
        if modifier.contains(Modifier::HIDDEN) {
            result |= ratatui::style::Modifier::HIDDEN;
        }
        if modifier.contains(Modifier::CROSSED_OUT) {
            result |= ratatui::style::Modifier::CROSSED_OUT;
        }
        
        result
    }
    
    /// Convert ratatui Modifier to helix Modifier
    pub fn convert_modifier_back(modifier: ratatui::style::Modifier) -> helix_view::graphics::Modifier {
        let mut result = helix_view::graphics::Modifier::empty();
        
        use helix_view::graphics::Modifier;
        if modifier.contains(ratatui::style::Modifier::BOLD) {
            result |= Modifier::BOLD;
        }
        if modifier.contains(ratatui::style::Modifier::DIM) {
            result |= Modifier::DIM;
        }
        if modifier.contains(ratatui::style::Modifier::ITALIC) {
            result |= Modifier::ITALIC;
        }
        // Note: ratatui UNDERLINED doesn't map directly to helix modifiers
        // since helix uses underline_style field instead
        if modifier.contains(ratatui::style::Modifier::SLOW_BLINK) {
            result |= Modifier::SLOW_BLINK;
        }
        if modifier.contains(ratatui::style::Modifier::RAPID_BLINK) {
            result |= Modifier::RAPID_BLINK;
        }
        if modifier.contains(ratatui::style::Modifier::REVERSED) {
            result |= Modifier::REVERSED;
        }
        if modifier.contains(ratatui::style::Modifier::HIDDEN) {
            result |= Modifier::HIDDEN;
        }
        if modifier.contains(ratatui::style::Modifier::CROSSED_OUT) {
            result |= Modifier::CROSSED_OUT;
        }
        
        result
    }
    
    /// Convert helix Spans to ratatui Line for Block titles
    pub fn convert_spans_to_line(spans: crate::text::Spans<'_>) -> ratatui::text::Line<'_> {
        // Convert helix Spans to ratatui Spans by converting each Span
        let ratatui_spans: Vec<ratatui::text::Span> = spans.0
            .into_iter()
            .map(|span| {
                let style = convert_style(span.style);
                ratatui::text::Span::styled(span.content, style)
            })
            .collect();
        ratatui::text::Line::from(ratatui_spans)
    }
    
    /// Convert helix Text to ratatui Text for Paragraph widgets (consuming version)
    pub fn convert_text<'a>(helix_text: crate::text::Text<'a>) -> ratatui::text::Text<'a> {
        // Convert each line (Spans) to ratatui Line
        let ratatui_lines: Vec<ratatui::text::Line> = helix_text.lines
            .into_iter()
            .map(|spans| convert_spans_to_line(spans))
            .collect();
        ratatui::text::Text::from(ratatui_lines)
    }
    
    /// Convert helix Text to ratatui Text for Paragraph widgets (borrowing version)
    pub fn convert_text_ref<'a>(helix_text: &'a crate::text::Text<'a>) -> ratatui::text::Text<'a> {
        // Convert each line (Spans) to ratatui Line
        let ratatui_lines: Vec<ratatui::text::Line> = helix_text.lines
            .iter()
            .map(|spans| convert_spans_to_line(spans.clone()))
            .collect();
        ratatui::text::Text::from(ratatui_lines)
    }
    
    /// Convert helix Borders to ratatui Borders (both use bitflags)
    pub fn convert_borders(borders: crate::widgets::Borders) -> ratatui::widgets::Borders {
        // Handle compound flags by checking each bit
        let mut result = ratatui::widgets::Borders::empty();
        if borders.contains(crate::widgets::Borders::TOP) {
            result |= ratatui::widgets::Borders::TOP;
        }
        if borders.contains(crate::widgets::Borders::RIGHT) {
            result |= ratatui::widgets::Borders::RIGHT;
        }
        if borders.contains(crate::widgets::Borders::BOTTOM) {
            result |= ratatui::widgets::Borders::BOTTOM;
        }
        if borders.contains(crate::widgets::Borders::LEFT) {
            result |= ratatui::widgets::Borders::LEFT;
        }
        result
    }
    
    /// Convert helix BorderType to ratatui BorderType
    pub fn convert_border_type(border_type: crate::widgets::BorderType) -> ratatui::widgets::BorderType {
        match border_type {
            crate::widgets::BorderType::Plain => ratatui::widgets::BorderType::Plain,
            crate::widgets::BorderType::Rounded => ratatui::widgets::BorderType::Rounded,
            crate::widgets::BorderType::Double => ratatui::widgets::BorderType::Double,
            crate::widgets::BorderType::Thick => ratatui::widgets::BorderType::Thick,
        }
    }
    
    /// Convert helix Block to ratatui Block for widgets
    pub fn convert_block<'a>(block: crate::widgets::Block<'a>) -> ratatui::widgets::Block<'a> {
        let mut ratatui_block = ratatui::widgets::Block::new();
        
        // Convert borders
        ratatui_block = ratatui_block.borders(convert_borders(block.get_borders()));
        
        // Convert border type
        ratatui_block = ratatui_block.border_type(convert_border_type(block.get_border_type()));
        
        // Convert style
        ratatui_block = ratatui_block.style(convert_style(block.get_style()));
        
        // Convert titles (left, center, right)
        if let Some(title) = block.get_title() {
            ratatui_block = ratatui_block.title(convert_spans_to_line(title.clone()));
        }
        
        ratatui_block
    }
    
    /// Convert helix Constraint to ratatui Constraint
    pub fn convert_constraint(constraint: crate::layout::Constraint) -> ratatui::layout::Constraint {
        match constraint {
            crate::layout::Constraint::Length(n) => ratatui::layout::Constraint::Length(n),
            crate::layout::Constraint::Max(n) => ratatui::layout::Constraint::Max(n),
            crate::layout::Constraint::Min(n) => ratatui::layout::Constraint::Min(n),
            crate::layout::Constraint::Percentage(n) => ratatui::layout::Constraint::Percentage(n),
            crate::layout::Constraint::Ratio(a, b) => ratatui::layout::Constraint::Ratio(a, b),
        }
    }
    
    /// Convert slice of helix Constraints to ratatui Constraints
    pub fn convert_constraints(constraints: &[crate::layout::Constraint]) -> Vec<ratatui::layout::Constraint> {
        constraints.iter().map(|c| convert_constraint(*c)).collect()
    }
    
    /// Convert helix TableState to ratatui TableState
    pub fn convert_table_state(state: &crate::widgets::TableState) -> ratatui::widgets::TableState {
        let mut ratatui_state = ratatui::widgets::TableState::default();
        ratatui_state.select(state.selected);
        // Note: ratatui TableState doesn't have a direct offset setter
        // We'll need to use scroll_to methods for proper scrolling behavior
        if let Some(selected) = state.selected {
            // Use select to set both selected index and handle scrolling
            ratatui_state.select(Some(selected));
        }
        ratatui_state
    }
    
    /// Update helix TableState from ratatui TableState
    pub fn update_table_state_from_ratatui(helix_state: &mut crate::widgets::TableState, ratatui_state: &ratatui::widgets::TableState) {
        helix_state.selected = ratatui_state.selected();
        // Note: ratatui TableState doesn't expose offset publicly
        // This will be handled through the selection mechanism
    }
    
    /// Render a ratatui widget with buffer conversion
    pub fn render_ratatui_widget<W>(
        widget: W,
        area: Rect, 
        surface: &mut crate::buffer::Buffer
    ) where 
        W: ratatui::widgets::Widget,
    {
        // Create a temporary ratatui buffer
        let ratatui_area = convert_rect(area);
        let mut ratatui_buffer = ratatui::buffer::Buffer::empty(ratatui_area);
        
        // Render the ratatui widget to the ratatui buffer
        widget.render(ratatui_area, &mut ratatui_buffer);
        
        // Copy the rendered content back to the helix buffer
        for y in ratatui_area.top()..ratatui_area.bottom() {
            for x in ratatui_area.left()..ratatui_area.right() {
                // ratatui buffer.get() returns &Cell directly (not Option)
                let ratatui_cell = ratatui_buffer.get(x, y);
                if let Some(helix_cell_pos) = surface.get_mut(x, y) {
                    let helix_cell = convert_cell_back(ratatui_cell);
                    *helix_cell_pos = helix_cell;
                }
            }
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        use helix_view::graphics::{Modifier};
        
        #[test]
        fn test_color_conversion() {
            // Test basic colors
            assert_eq!(convert_color(Color::Red), ratatui::style::Color::Red);
            assert_eq!(convert_color(Color::Blue), ratatui::style::Color::Blue);
            assert_eq!(convert_color(Color::Green), ratatui::style::Color::Green);
            
            // Test RGB
            assert_eq!(convert_color(Color::Rgb(255, 128, 64)), ratatui::style::Color::Rgb(255, 128, 64));
            
            // Test indexed
            assert_eq!(convert_color(Color::Indexed(42)), ratatui::style::Color::Indexed(42));
            
            // Test round-trip conversion
            let original = Color::LightCyan;
            let converted = convert_color(original);
            let back = convert_color_back(converted);
            assert_eq!(original, back);
        }
        
        #[test]
        fn test_style_conversion() {
            let style = Style::default()
                .fg(Color::Red)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD | Modifier::ITALIC);
            
            let ratatui_style = convert_style(style);
            
            assert_eq!(ratatui_style.fg, Some(ratatui::style::Color::Red));
            assert_eq!(ratatui_style.bg, Some(ratatui::style::Color::Blue));
            assert!(ratatui_style.add_modifier.contains(ratatui::style::Modifier::BOLD));
            assert!(ratatui_style.add_modifier.contains(ratatui::style::Modifier::ITALIC));
        }
        
        #[test] 
        fn test_rect_conversion() {
            let rect = Rect::new(10, 20, 80, 24);
            let ratatui_rect = convert_rect(rect);
            let back_rect = convert_rect_back(ratatui_rect);
            
            assert_eq!(rect, back_rect);
            assert_eq!(ratatui_rect.x, 10);
            assert_eq!(ratatui_rect.y, 20);
            assert_eq!(ratatui_rect.width, 80);
            assert_eq!(ratatui_rect.height, 24);
        }
    }
}

#[cfg(not(feature = "ratatui-migration"))]
pub mod ratatui_compat {
    //! Stub module when ratatui migration feature is disabled
    //! This allows code to conditionally compile compatibility layers
}
