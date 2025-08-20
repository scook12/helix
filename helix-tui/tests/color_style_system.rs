use helix_view::graphics::{Color, Modifier, Style, Rect};
use helix_tui::buffer::{Buffer, Cell};

#[test]
fn test_color_combinations() {
    // Test basic color combinations work without panic
    let colors = vec![
        Color::Black,
        Color::Red, 
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::Gray,
        Color::LightGray,
        Color::LightRed,
        Color::LightGreen,
        Color::LightYellow,
        Color::LightBlue,
        Color::LightMagenta,
        Color::LightCyan,
        Color::White,
        Color::Rgb(128, 128, 128),
        Color::Indexed(42),
    ];

    for fg in &colors {
        for bg in &colors {
            let style = Style::default().fg(*fg).bg(*bg);
            // Test style creation doesn't panic
            assert_eq!(style.fg, Some(*fg));
            assert_eq!(style.bg, Some(*bg));
        }
    }
}

#[test]
fn test_style_modifiers() {
    // Test all modifier combinations
    let modifiers = vec![
        Modifier::BOLD,
        Modifier::DIM,
        Modifier::ITALIC,
        Modifier::SLOW_BLINK,
        Modifier::RAPID_BLINK,
        Modifier::REVERSED,
        Modifier::HIDDEN,
        Modifier::CROSSED_OUT,
    ];

    for modifier in &modifiers {
        let style = Style::default().add_modifier(*modifier);
        assert!(style.add_modifier.contains(*modifier));
        
        let style = Style::default().remove_modifier(*modifier);
        assert!(style.sub_modifier.contains(*modifier));
    }
    
    // Test multiple modifiers
    let style = Style::default()
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::ITALIC)
        .add_modifier(Modifier::CROSSED_OUT);
    
    assert!(style.add_modifier.contains(Modifier::BOLD));
    assert!(style.add_modifier.contains(Modifier::ITALIC));
    assert!(style.add_modifier.contains(Modifier::CROSSED_OUT));
}

#[test]
fn test_style_patches() {
    let base_style = Style::default()
        .fg(Color::Red)
        .bg(Color::Blue)
        .add_modifier(Modifier::BOLD);
    
    let patch_style = Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::ITALIC);
    
    let patched = base_style.patch(patch_style);
    
    // Foreground should be overridden
    assert_eq!(patched.fg, Some(Color::Green));
    // Background should be preserved
    assert_eq!(patched.bg, Some(Color::Blue));
    // Both modifiers should be present
    assert!(patched.add_modifier.contains(Modifier::BOLD));
    assert!(patched.add_modifier.contains(Modifier::ITALIC));
}

#[test]
fn test_buffer_cell_styling() {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
    
    // Test setting styles on buffer cells
    let style1 = Style::default().fg(Color::Red).bg(Color::Blue);
    let style2 = Style::default().fg(Color::Green).add_modifier(Modifier::BOLD);
    
    if let Some(cell1) = buffer.get_mut(0, 0) {
        cell1.set_style(style1);
        let cell_style = cell1.style();
        assert_eq!(cell_style.fg, Some(Color::Red));
        assert_eq!(cell_style.bg, Some(Color::Blue));
    }
    
    if let Some(cell2) = buffer.get_mut(1, 1) {
        cell2.set_style(style2);
        let cell_style = cell2.style();
        assert_eq!(cell_style.fg, Some(Color::Green));
        assert!(cell_style.add_modifier.contains(Modifier::BOLD));
    }
    
    // Test that different cells maintain their styles
    if let Some(cell) = buffer.get(0, 0) {
        let cell_style = cell.style();
        assert_eq!(cell_style.fg, Some(Color::Red));
        assert_eq!(cell_style.bg, Some(Color::Blue));
    }
    if let Some(cell) = buffer.get(1, 1) {
        let cell_style = cell.style();
        assert_eq!(cell_style.fg, Some(Color::Green));
        assert!(cell_style.add_modifier.contains(Modifier::BOLD));
    }
}

#[test]
fn test_cell_symbol_and_style() {
    let mut cell = Cell::default();
    
    // Test symbol setting
    cell.set_symbol("A");
    assert_eq!(cell.symbol, "A");
    
    // Test style setting  
    let style = Style::default().fg(Color::Yellow);
    cell.set_style(style);
    let cell_style = cell.style();
    assert_eq!(cell_style.fg, Some(Color::Yellow));
    
    // Test chaining
    cell.set_symbol("B").set_style(Style::default().bg(Color::Magenta));
    assert_eq!(cell.symbol, "B");
    assert_eq!(cell.style().bg, Some(Color::Magenta));
}

#[test]
fn test_color_parsing_rgb() {
    // Test RGB color creation
    let color1 = Color::Rgb(255, 0, 0); // Red
    let color2 = Color::Rgb(0, 255, 0); // Green  
    let color3 = Color::Rgb(0, 0, 255); // Blue
    
    // Test indexed colors
    for i in 0..=255 {
        let _color = Color::Indexed(i);
        // Just test creation doesn't panic
    }
    
    // Test that colors are equality comparable
    assert_eq!(Color::Red, Color::Red);
    assert_ne!(Color::Red, Color::Blue);
    assert_eq!(Color::Rgb(255, 0, 0), Color::Rgb(255, 0, 0));
    assert_ne!(Color::Rgb(255, 0, 0), Color::Rgb(0, 255, 0));
}

#[test]
fn test_style_reset() {
    let style = Style::default()
        .fg(Color::Red)
        .bg(Color::Blue)
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::ITALIC);
    
    // Test resetting individual components
    let no_fg = Style::default()
        .bg(Color::Blue)
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::ITALIC);
        
    let reset_style = Style {
        fg: None,
        bg: style.bg,
        underline_color: style.underline_color,
        underline_style: style.underline_style,
        add_modifier: style.add_modifier,
        sub_modifier: style.sub_modifier,
    };
    
    assert_eq!(reset_style.fg, None);
    assert_eq!(reset_style.bg, Some(Color::Blue));
}
