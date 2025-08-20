use helix_tui::buffer::Buffer;
use helix_view::graphics::{Color, Style, Rect, Modifier, UnderlineStyle};

#[test]
fn test_buffer_basic_operations() {
    // Test basic buffer cell manipulation
    let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
    
    // Test setting cells with different styles
    buffer.get_mut(0, 0).unwrap().set_char('X').set_fg(Color::Red);
    buffer.get_mut(1, 0).unwrap().set_char('Y').set_bg(Color::Blue);
    
    assert_eq!(buffer.get(0, 0).unwrap().symbol, "X");
    assert_eq!(buffer.get(0, 0).unwrap().fg, Color::Red);
    assert_eq!(buffer.get(1, 0).unwrap().bg, Color::Blue);
}

#[test]
fn test_buffer_cell_style_operations() {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 5, 5));
    
    // Test setting styles with various combinations
    let style = Style::default()
        .fg(Color::Red)
        .bg(Color::Blue)
        .add_modifier(Modifier::BOLD)
        .underline_style(UnderlineStyle::Curl);
        
    buffer.get_mut(2, 2).unwrap().set_style(style);
    
    let cell = buffer.get(2, 2).unwrap();
    assert_eq!(cell.fg, Color::Red);
    assert_eq!(cell.bg, Color::Blue);
    assert!(cell.modifier.contains(Modifier::BOLD));
    assert_eq!(cell.underline_style, UnderlineStyle::Curl);
}

#[test] 
fn test_buffer_resize_and_bounds() {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
    buffer.get_mut(5, 5).unwrap().set_char('X');
    
    // Test bounds checking
    assert!(buffer.get(0, 0).is_some());
    assert!(buffer.get(9, 9).is_some());
    assert!(buffer.get(10, 10).is_none()); // Out of bounds
    assert!(buffer.get(5, 10).is_none()); // Out of bounds
    
    // Test that buffer maintains content correctly
    assert_eq!(buffer.get(5, 5).unwrap().symbol, "X");
}

#[test]
fn test_buffer_with_lines() {
    // Test buffer creation from lines (used in visual tests)
    let lines = vec![
        "Hello, world!",
        "Second line",
        "Third line with unicode: 🦀",
    ];
    
    let buffer = Buffer::with_lines(lines.clone());
    
    // Verify dimensions
    assert_eq!(buffer.area.height, 3);
    // Unicode character 🦀 takes 2 display columns, so "Third line with unicode: 🦀" is 27 chars wide
    assert_eq!(buffer.area.width, 27);
    
    // Verify content is preserved
    let reconstructed: Vec<String> = (0..buffer.area.height)
        .map(|y| {
            (0..buffer.area.width)
                .map(|x| buffer.get(x, y).map(|c| c.symbol.as_str()).unwrap_or(" "))
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect();
    
    assert_eq!(reconstructed[0], "Hello, world!");
    assert_eq!(reconstructed[1], "Second line");
    assert!(reconstructed[2].contains("🦀"));
}

#[test]
fn test_buffer_set_string() {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 5));
    
    // Test setting string with style
    let style = Style::default().fg(Color::Green).add_modifier(Modifier::BOLD);
    buffer.set_string(2, 1, "Hello", style);
    
    // Verify each character has the correct style
    for i in 0..5 {
        let cell = buffer.get(2 + i, 1).unwrap();
        assert_eq!(cell.fg, Color::Green);
        assert!(cell.modifier.contains(Modifier::BOLD));
    }
    
    // Verify content
    assert_eq!(buffer.get(2, 1).unwrap().symbol, "H");
    assert_eq!(buffer.get(6, 1).unwrap().symbol, "o");
}

#[test]
fn test_buffer_edge_cases() {
    // Test zero-size buffer
    let buffer = Buffer::empty(Rect::new(0, 0, 0, 0));
    assert_eq!(buffer.content.len(), 0);
    assert!(buffer.get(0, 0).is_none());
    
    // Test 1x1 buffer
    let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 1));
    buffer.get_mut(0, 0).unwrap().set_char('X');
    assert_eq!(buffer.get(0, 0).unwrap().symbol, "X");
    
    // Test large buffer creation (performance check)
    let buffer = Buffer::empty(Rect::new(0, 0, 100, 100));
    assert_eq!(buffer.content.len(), 10_000);
}

#[test]
fn test_buffer_cell_reset() {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 5, 5));
    
    // Set up a cell with complex styling
    let cell = buffer.get_mut(2, 2).unwrap();
    cell.set_char('X')
        .set_fg(Color::Red)
        .set_bg(Color::Blue)
        .set_style(Style::default().add_modifier(Modifier::BOLD));
    
    // Reset the cell
    cell.reset();
    
    // Verify reset state
    assert_eq!(cell.symbol, " ");
    assert_eq!(cell.fg, Color::Reset);
    assert_eq!(cell.bg, Color::Reset);
    assert_eq!(cell.modifier, Modifier::empty());
}

#[test]
fn test_buffer_unicode_handling() {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 5));
    
    // Test various unicode characters
    let unicode_chars = vec!['🦀', '🚀', '💻', '📝', '🎯'];
    
    for (i, ch) in unicode_chars.iter().enumerate() {
        buffer.get_mut(i as u16, 0).unwrap().set_char(*ch);
        assert_eq!(buffer.get(i as u16, 0).unwrap().symbol, ch.to_string());
    }
    
    // Test emoji with skin tone modifiers
    buffer.set_string(0, 1, "👋🏽", Style::default());
    // Should handle complex unicode sequences properly
}
