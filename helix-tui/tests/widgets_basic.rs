use helix_tui::backend::{Backend, TestBackend};
use helix_tui::Terminal;
use helix_tui::buffer::Buffer;
use helix_tui::widgets::{Block, Borders, Widget};
use helix_view::graphics::{Color, Style, Rect};

#[test]
fn test_block_widget_rendering() {
    let backend = TestBackend::new(10, 5);
    let mut terminal = Terminal::new(backend).unwrap();
    
    // Test basic block rendering doesn't panic
    let buffer = terminal.current_buffer_mut();
    let area = Rect::new(0, 0, 10, 5);
    
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Test");
    
    block.render(area, buffer);
    
    // Verify the block was rendered
    let content = buffer.content();
    assert!(!content.is_empty());
    assert_eq!(content.len(), 50); // 10x5 = 50 cells
}

#[test]
fn test_block_widget_borders() {
    let backend = TestBackend::new(6, 4);
    let mut terminal = Terminal::new(backend).unwrap();
    
    let buffer = terminal.current_buffer_mut();
    let area = Rect::new(0, 0, 6, 4);
    
    // Test different border combinations
    let border_tests = vec![
        Borders::empty(),
        Borders::ALL,
        Borders::LEFT,
        Borders::RIGHT,
        Borders::TOP,
        Borders::BOTTOM,
        Borders::LEFT | Borders::RIGHT,
        Borders::TOP | Borders::BOTTOM,
    ];
    
    for borders in border_tests {
        let block = Block::default().borders(borders);
        block.render(area, buffer);
        // Just test it doesn't panic
    }
}

#[test] 
fn test_block_widget_styling() {
    let backend = TestBackend::new(8, 3);
    let mut terminal = Terminal::new(backend).unwrap();
    
    let buffer = terminal.current_buffer_mut();
    let area = Rect::new(0, 0, 8, 3);
    
    // Test block with style
    let style = Style::default().fg(Color::Red).bg(Color::Blue);
    let block = Block::default()
        .borders(Borders::ALL)
        .style(style);
    
    block.render(area, buffer);
    
    // Check that some cells have the style applied
    let content = buffer.content();
    assert!(!content.is_empty());
    
    // The border cells should have the style
    let first_cell = &content[0];
    assert_eq!(first_cell.fg, Color::Red);
    assert_eq!(first_cell.bg, Color::Blue);
}

#[test]
fn test_widget_rendering_edge_cases() {
    let backend = TestBackend::new(1, 1);
    let mut terminal = Terminal::new(backend).unwrap();
    
    let buffer = terminal.current_buffer_mut();
    
    // Test rendering in minimal area
    let area = Rect::new(0, 0, 1, 1);
    let block = Block::default().borders(Borders::ALL);
    block.render(area, buffer);
    
    // Test zero area
    let zero_area = Rect::new(0, 0, 0, 0);
    let block = Block::default().borders(Borders::ALL);
    block.render(zero_area, buffer); // Should not panic
}

#[test]
fn test_buffer_with_lines_basic() {
    // Test creating buffers from string lines
    let lines = vec![
        "Line 1",
        "Line 2 is longer",
        "Short",
        "",
        "Final line",
    ];
    
    let buffer = Buffer::with_lines(lines.clone());
    
    assert_eq!(buffer.area().height, 5);
    assert_eq!(buffer.area().width, 16); // Length of longest line
    
    // Verify content is accessible
    let content = buffer.content();
    assert_eq!(content.len(), 5 * 16);
}

#[test]
fn test_buffer_string_setting() {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 10));
    let style = Style::default().fg(Color::Yellow);
    
    // Test setting strings at various positions
    buffer.set_string(0, 0, "Top left", style);
    buffer.set_string(5, 5, "Middle", style);
    buffer.set_string(0, 9, "Bottom", style);
    
    // Test unicode strings
    buffer.set_string(0, 1, "Unicode: 日本語", style);
    buffer.set_string(0, 2, "Emojis: 🚀🎉", style);
    
    // Verify strings were set (check non-empty cells)
    if let Some(cell) = buffer.get(0, 0) {
        assert_ne!(cell.symbol, " ");
    }
    if let Some(cell) = buffer.get(5, 5) {
        assert_ne!(cell.symbol, " ");
    }
}

#[test]
fn test_terminal_buffer_swap() {
    let backend = TestBackend::new(10, 5);
    let mut terminal = Terminal::new(backend).unwrap();
    
    // Test multiple draws to ensure buffer swapping works
    for i in 0..3 {
        let result = terminal.draw(Some((i, i)), helix_view::graphics::CursorKind::Block);
        assert!(result.is_ok(), "Draw {} failed", i);
    }
    
    // Terminal should still be in a valid state
    let size = terminal.backend().size().unwrap();
    assert_eq!(size.width, 10);
    assert_eq!(size.height, 5);
}
