use helix_tui::{
    backend::{Backend, TestBackend},
    buffer::Cell,
    terminal::{Config, Terminal},
};
use helix_view::graphics::{Color, CursorKind, Modifier, Rect, Style, UnderlineStyle};

#[cfg(feature = "ratatui-migration")]
use helix_tui::backend::RatatuiBackendAdapter;

/// Test basic backend operations work consistently across implementations
#[test]
fn test_backend_trait_consistency() {
    let mut test_backend = TestBackend::new(80, 24);
    let config1 = Config { enable_mouse_capture: false };
    let config2 = Config { enable_mouse_capture: false };
    let config3 = Config { enable_mouse_capture: false };
    
    // Test all backend operations
    assert!(test_backend.claim(config1).is_ok());
    assert!(test_backend.reconfigure(config2).is_ok());
    assert!(test_backend.hide_cursor().is_ok());
    assert!(test_backend.show_cursor(CursorKind::Block).is_ok());
    assert!(test_backend.set_cursor(10, 5).is_ok());
    assert!(test_backend.get_cursor().is_ok());
    assert!(test_backend.clear().is_ok());
    assert!(test_backend.flush().is_ok());
    assert!(test_backend.restore(config3).is_ok());
    
    // Test size
    let size = test_backend.size().unwrap();
    assert_eq!(size.width, 80);
    assert_eq!(size.height, 24);
}

/// Test backend with Terminal integration
#[test]
fn test_backend_terminal_integration() {
    let backend = TestBackend::new(60, 20);
    let mut terminal = Terminal::new(backend).unwrap();
    let config1 = Config { enable_mouse_capture: false };
    let config2 = Config { enable_mouse_capture: false };
    
    // Test terminal operations
    assert!(terminal.claim(config1).is_ok());
    assert!(terminal.hide_cursor().is_ok());
    assert!(terminal.show_cursor(CursorKind::Bar).is_ok());
    assert!(terminal.clear().is_ok());
    
    // Test buffer access
    let buffer = terminal.current_buffer_mut();
    buffer[(0, 0)] = Cell {
        symbol: "A".to_string(),
        fg: Color::Red,
        bg: Color::Blue,
        modifier: Modifier::BOLD,
        underline_color: Color::Green,
        underline_style: UnderlineStyle::Line,
    };
    
    // Test drawing
    assert!(terminal.draw(Some((5, 5)), CursorKind::Block).is_ok());
    assert!(terminal.restore(config2).is_ok());
}

/// Test backend cell drawing operations
#[test]
fn test_backend_cell_drawing() {
    let mut backend = TestBackend::new(40, 15);
    
    // Create test cells with various styles
    let test_cells = vec![
        (0, 0, Cell {
            symbol: "H".to_string(),
            fg: Color::Red,
            bg: Color::Reset,
            modifier: Modifier::BOLD,
            underline_color: Color::Reset,
            underline_style: UnderlineStyle::Reset,
        }),
        (1, 0, Cell {
            symbol: "i".to_string(),
            fg: Color::Green,
            bg: Color::Yellow,
            modifier: Modifier::ITALIC,
            underline_color: Color::Reset,
            underline_style: UnderlineStyle::Reset,
        }),
        (5, 2, Cell {
            symbol: "!".to_string(),
            fg: Color::Blue,
            bg: Color::Reset,
            modifier: Modifier::CROSSED_OUT,
            underline_color: Color::Cyan,
            underline_style: UnderlineStyle::Curl,
        }),
    ];
    
    // Draw cells to backend
    let cell_iter = test_cells.iter().map(|(x, y, cell)| (*x, *y, cell));
    assert!(backend.draw(cell_iter).is_ok());
    
    // Verify cells were drawn
    let buffer = backend.buffer();
    assert_eq!(buffer[(0, 0)].symbol, "H");
    assert_eq!(buffer[(1, 0)].symbol, "i");
    assert_eq!(buffer[(5, 2)].symbol, "!");
}

/// Test backend edge cases and error handling
#[test]
fn test_backend_edge_cases() {
    let mut backend = TestBackend::new(10, 5);
    
    // Test out-of-bounds operations
    assert!(backend.set_cursor(15, 10).is_ok()); // Should not panic
    
    // Test zero-size operations
    let mut zero_backend = TestBackend::new(0, 0);
    assert!(zero_backend.clear().is_ok());
    assert!(zero_backend.flush().is_ok());
    
    // Test drawing to empty buffer
    let empty_cells: Vec<(u16, u16, &Cell)> = vec![];
    assert!(backend.draw(empty_cells.into_iter()).is_ok());
}

/// Test backend resizing behavior
#[test]
fn test_backend_resize_behavior() {
    let mut backend = TestBackend::new(20, 10);
    
    // Set some initial content
    let cell = Cell {
        symbol: "X".to_string(),
        fg: Color::White,
        bg: Color::Black,
        modifier: Modifier::empty(),
        underline_color: Color::Reset,
        underline_style: UnderlineStyle::Reset,
    };
    
    let cells = vec![(5, 5, &cell)];
    assert!(backend.draw(cells.into_iter()).is_ok());
    
    // Resize backend
    backend.resize(40, 20);
    
    // Verify new size
    let size = backend.size().unwrap();
    assert_eq!(size.width, 40);
    assert_eq!(size.height, 20);
    
    // Previous content should be cleared after resize
    let buffer = backend.buffer();
    assert_eq!(buffer.area, Rect::new(0, 0, 40, 20));
}

#[cfg(feature = "ratatui-migration")]
mod ratatui_tests {
    use super::*;
    use ratatui::backend::TestBackend as RatatuiTestBackend;
    
    /// Test ratatui backend adapter basic operations
    #[test]
    fn test_ratatui_backend_adapter() {
        let ratatui_backend = RatatuiTestBackend::new(80, 24);
        let mut adapter = RatatuiBackendAdapter::new(ratatui_backend).unwrap();
        let config1 = Config { enable_mouse_capture: false };
        let config2 = Config { enable_mouse_capture: false };
        let config3 = Config { enable_mouse_capture: false };
        
        // Test all backend trait methods
        assert!(adapter.claim(config1).is_ok());
        assert!(adapter.reconfigure(config2).is_ok());
        assert!(adapter.hide_cursor().is_ok());
        assert!(adapter.show_cursor(CursorKind::Underline).is_ok());
        assert!(adapter.set_cursor(20, 10).is_ok());
        assert!(adapter.clear().is_ok());
        assert!(adapter.flush().is_ok());
        assert!(adapter.restore(config3).is_ok());
        
        // Test size
        let size = adapter.size().unwrap();
        assert_eq!(size.width, 80);
        assert_eq!(size.height, 24);
    }
    
    /// Test ratatui adapter with Terminal
    #[test]
    fn test_ratatui_adapter_terminal_integration() {
        let ratatui_backend = RatatuiTestBackend::new(50, 18);
        let adapter = RatatuiBackendAdapter::new(ratatui_backend).unwrap();
        let mut terminal = Terminal::new(adapter).unwrap();
        let config1 = Config { enable_mouse_capture: false };
        let config2 = Config { enable_mouse_capture: false };
        
        // Test terminal operations with ratatui adapter
        assert!(terminal.claim(config1).is_ok());
        
        // Test buffer operations
        let buffer = terminal.current_buffer_mut();
        buffer[(10, 5)] = Cell {
            symbol: "R".to_string(),
            fg: Color::Magenta,
            bg: Color::Cyan,
            modifier: Modifier::BOLD | Modifier::ITALIC,
            underline_color: Color::Yellow,
            underline_style: UnderlineStyle::Dotted,
        };
        
        // Test drawing
        assert!(terminal.draw(Some((10, 5)), CursorKind::Bar).is_ok());
        assert!(terminal.restore(config2).is_ok());
    }
    
    /// Test cell conversion between helix and ratatui formats
    #[test]
    fn test_cell_conversion_compatibility() {
        use helix_tui::compat::ratatui_compat::{convert_cell, convert_cell_back};
        
        let helix_cell = Cell {
            symbol: "★".to_string(),
            fg: Color::Rgb(255, 128, 0),
            bg: Color::Indexed(42),
            modifier: Modifier::BOLD | Modifier::REVERSED,
            underline_color: Color::Green,
            underline_style: UnderlineStyle::Curl,
        };
        
        // Convert to ratatui and back
        let ratatui_cell = convert_cell(&helix_cell);
        let converted_back = convert_cell_back(&ratatui_cell);
        
        // Verify core properties are preserved
        assert_eq!(helix_cell.symbol, converted_back.symbol);
        assert_eq!(helix_cell.fg, converted_back.fg);
        assert_eq!(helix_cell.bg, converted_back.bg);
        assert_eq!(helix_cell.modifier, converted_back.modifier);
        // Note: underline properties may not round-trip perfectly due to differences
    }
    
    /// Test adapter drawing with styled content
    #[test]
    fn test_ratatui_adapter_styled_drawing() {
        let ratatui_backend = RatatuiTestBackend::new(30, 12);
        let mut adapter = RatatuiBackendAdapter::new(ratatui_backend).unwrap();
        
        // Create styled cells
        let styled_cells = vec![
            (0, 0, Cell {
                symbol: "A".to_string(),
                fg: Color::Red,
                bg: Color::Reset,
                modifier: Modifier::BOLD,
                underline_color: Color::Reset,
                underline_style: UnderlineStyle::Reset,
            }),
            (1, 0, Cell {
                symbol: "B".to_string(),
                fg: Color::Green,
                bg: Color::Blue,
                modifier: Modifier::ITALIC,
                underline_color: Color::Yellow,
                underline_style: UnderlineStyle::Line,
            }),
            (2, 0, Cell {
                symbol: "C".to_string(),
                fg: Color::Rgb(128, 64, 192),
                bg: Color::Indexed(8),
                modifier: Modifier::DIM | Modifier::CROSSED_OUT,
                underline_color: Color::Magenta,
                underline_style: UnderlineStyle::Dashed,
            }),
        ];
        
        let cell_iter = styled_cells.iter().map(|(x, y, cell)| (*x, *y, cell));
        assert!(adapter.draw(cell_iter).is_ok());
        assert!(adapter.flush().is_ok());
    }
    
    /// Test adapter cursor operations
    #[test]
    fn test_ratatui_adapter_cursor_operations() {
        let ratatui_backend = RatatuiTestBackend::new(25, 10);
        let mut adapter = RatatuiBackendAdapter::new(ratatui_backend).unwrap();
        
        // Test different cursor kinds
        let cursor_kinds = vec![
            CursorKind::Block,
            CursorKind::Bar,
            CursorKind::Underline,
            CursorKind::Hidden,
        ];
        
        for kind in cursor_kinds {
            assert!(adapter.show_cursor(kind).is_ok());
            assert!(adapter.set_cursor(12, 6).is_ok());
            let (x, y) = adapter.get_cursor().unwrap();
            assert_eq!(x, 12);
            assert_eq!(y, 6);
        }
        
        assert!(adapter.hide_cursor().is_ok());
    }
}

/// Test backend performance with large content
#[test]
fn test_backend_performance_baseline() {
    let mut backend = TestBackend::new(200, 100);
    
    // Create large amount of content
    let mut cells = Vec::new();
    for y in 0u16..100 {
        for x in 0u16..200 {
            cells.push((x, y, Cell {
                symbol: (((x + y) % 26 + 65) as u8 as char).to_string(),
                fg: Color::Indexed(((x + y) % 256) as u8),
                bg: Color::Reset,
                modifier: if (x + y) % 3 == 0 { Modifier::BOLD } else { Modifier::empty() },
                underline_color: Color::Reset,
                underline_style: UnderlineStyle::Reset,
            }));
        }
    }
    
    let cell_refs: Vec<(u16, u16, &Cell)> = cells.iter().map(|(x, y, cell)| (*x, *y, cell)).collect();
    
    // This should complete without panicking or taking excessive time
    assert!(backend.draw(cell_refs.into_iter()).is_ok());
    assert!(backend.flush().is_ok());
    
    // Verify some content was drawn
    let buffer = backend.buffer();
    assert!(!buffer.content().is_empty());
}

/// Test concurrent backend operations don't interfere
#[test]
fn test_backend_isolation() {
    let mut backend1 = TestBackend::new(20, 10);
    let mut backend2 = TestBackend::new(30, 15);
    
    // Set different content on each backend
    let cell1 = Cell {
        symbol: "1".to_string(),
        fg: Color::Red,
        bg: Color::Reset,
        modifier: Modifier::BOLD,
        underline_color: Color::Reset,
        underline_style: UnderlineStyle::Reset,
    };
    
    let cell2 = Cell {
        symbol: "2".to_string(),
        fg: Color::Blue,
        bg: Color::Yellow,
        modifier: Modifier::ITALIC,
        underline_color: Color::Reset,
        underline_style: UnderlineStyle::Reset,
    };
    
    assert!(backend1.draw(vec![(5, 5, &cell1)].into_iter()).is_ok());
    assert!(backend2.draw(vec![(10, 8, &cell2)].into_iter()).is_ok());
    
    // Verify isolation - each backend should have its own content
    assert_eq!(backend1.buffer()[(5, 5)].symbol, "1");
    assert_eq!(backend2.buffer()[(10, 8)].symbol, "2");
    
    // Backend1 should not have backend2's content
    assert_ne!(backend1.buffer()[(10, 8)].symbol, "2");
}

/// Test backend compatibility with various cell styles
#[test]
fn test_backend_style_compatibility() {
    let mut backend = TestBackend::new(50, 20);
    
    let style_test_cases = vec![
        ("Basic colors", Style::default().fg(Color::Red).bg(Color::Blue)),
        ("RGB colors", Style::default().fg(Color::Rgb(128, 64, 255)).bg(Color::Rgb(32, 128, 16))),
        ("Indexed colors", Style::default().fg(Color::Indexed(196)).bg(Color::Indexed(46))),
        ("Bold modifier", Style::default().add_modifier(Modifier::BOLD)),
        ("Multiple modifiers", Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC | Modifier::CROSSED_OUT)),
        ("Underline style", Style::default().underline_color(Color::Cyan).underline_style(UnderlineStyle::Curl)),
        ("Complex style", Style::default()
            .fg(Color::LightMagenta)
            .bg(Color::Gray)
            .add_modifier(Modifier::DIM | Modifier::REVERSED)
            .underline_color(Color::LightGreen)
            .underline_style(UnderlineStyle::DoubleLine)),
    ];
    
    for (i, (name, style)) in style_test_cases.iter().enumerate() {
        let cell = Cell {
            symbol: format!("{}", i),
            fg: style.fg.unwrap_or(Color::Reset),
            bg: style.bg.unwrap_or(Color::Reset),
            modifier: style.add_modifier,
            underline_color: style.underline_color.unwrap_or(Color::Reset),
            underline_style: style.underline_style.unwrap_or(UnderlineStyle::Reset),
        };
        
        assert!(backend.draw(vec![(i as u16, 0, &cell)].into_iter()).is_ok(), 
                "Failed to draw cell for style test: {}", name);
    }
    
    assert!(backend.flush().is_ok());
}

#[cfg(feature = "ratatui-migration")]
/// Test compatibility between helix-tui and ratatui backends
#[test]
fn test_cross_backend_compatibility() {
    use ratatui::backend::TestBackend as RatatuiTestBackend;
    
    // Create both types of backends
    let helix_backend = TestBackend::new(40, 20);
    let ratatui_backend = RatatuiBackendAdapter::new(
        RatatuiTestBackend::new(40, 20)
    ).unwrap();
    
    // Create terminals with both backends
    let mut helix_terminal = Terminal::new(helix_backend).unwrap();
    let mut ratatui_terminal = Terminal::new(ratatui_backend).unwrap();
    
    let config1 = Config { enable_mouse_capture: false };
    let config2 = Config { enable_mouse_capture: false };
    let config3 = Config { enable_mouse_capture: false };
    let config4 = Config { enable_mouse_capture: false };
    
    // Test that both terminals support the same operations
    assert!(helix_terminal.claim(config1).is_ok());
    assert!(ratatui_terminal.claim(config2).is_ok());
    
    // Both should support the same size
    assert_eq!(helix_terminal.size().unwrap(), ratatui_terminal.size().unwrap());
    
    // Both should support drawing operations
    assert!(helix_terminal.draw(Some((10, 10)), CursorKind::Block).is_ok());
    assert!(ratatui_terminal.draw(Some((10, 10)), CursorKind::Block).is_ok());
    
    assert!(helix_terminal.restore(config3).is_ok());
    assert!(ratatui_terminal.restore(config4).is_ok());
}

#[cfg(feature = "ratatui-migration")]
/// Test that conversion functions work correctly
#[test]
fn test_conversion_functions() {
    use helix_tui::compat::ratatui_compat::{
        convert_color, convert_color_back, 
        convert_rect, convert_rect_back,
        convert_style,
    };
    
    // Test color round-trip
    let colors = vec![
        Color::Reset, Color::Red, Color::Green, Color::Blue,
        Color::Rgb(255, 128, 64), Color::Indexed(42),
    ];
    
    for color in colors {
        let ratatui_color = convert_color(color);
        let back_color = convert_color_back(ratatui_color);
        assert_eq!(color, back_color, "Color conversion round-trip failed for {:?}", color);
    }
    
    // Test rect round-trip
    let rect = Rect::new(15, 25, 100, 50);
    let ratatui_rect = convert_rect(rect);
    let back_rect = convert_rect_back(ratatui_rect);
    assert_eq!(rect, back_rect);
    
    // Test style conversion
    let style = Style::default()
        .fg(Color::Yellow)
        .bg(Color::Magenta)
        .add_modifier(Modifier::BOLD | Modifier::ITALIC);
    
    let ratatui_style = convert_style(style);
    assert_eq!(ratatui_style.fg, Some(ratatui::style::Color::Yellow));
    assert_eq!(ratatui_style.bg, Some(ratatui::style::Color::Magenta));
    assert!(ratatui_style.add_modifier.contains(ratatui::style::Modifier::BOLD));
    assert!(ratatui_style.add_modifier.contains(ratatui::style::Modifier::ITALIC));
}
