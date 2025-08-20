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
