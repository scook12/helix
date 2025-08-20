use helix_tui::backend::TestBackend;
use helix_tui::Terminal;
use helix_tui::widgets::{Block, Borders, Widget, Paragraph, Table, Row, Cell};
use helix_tui::text::{Spans, Span, Text};
use helix_tui::layout::Constraint;
use helix_view::graphics::{Color, Style, Modifier, Rect};

/// Test that Block widget rendering produces consistent output
#[test]
fn test_block_widget_migration_compatibility() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    
    let buffer = terminal.current_buffer_mut();
    let area = Rect::new(0, 0, 20, 10);
    
    // Test various block configurations
    let test_cases = vec![
        ("Basic block", Block::default().borders(Borders::ALL)),
        ("Block with title", Block::default().borders(Borders::ALL).title("Test Title")),
        ("Styled block", Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Red))),
        ("Partial borders", Block::default().borders(Borders::LEFT | Borders::RIGHT)),
    ];
    
    for (name, block) in test_cases {
        // Clear buffer for clean test
        buffer.reset();
        
        // Render block
        block.render(area, buffer);
        
        // Verify rendering didn't panic and produced content
        let content = buffer.content();
        assert!(!content.is_empty(), "Block '{}' produced no content", name);
        
        // Verify border cells are set (not default spaces)
        if area.width > 0 && area.height > 0 {
            let first_cell = &content[0];
            // Border should have non-space content
            assert_ne!(first_cell.symbol, " ", "Block '{}' didn't render border", name);
        }
    }
}

/// Test that Paragraph widget rendering works correctly
#[test]
fn test_paragraph_widget_migration_compatibility() {
    let backend = TestBackend::new(30, 8);
    let mut terminal = Terminal::new(backend).unwrap();
    
    let buffer = terminal.current_buffer_mut();
    let area = Rect::new(0, 0, 30, 8);
    
    let test_texts = vec![
        ("Simple text", Text::from("Simple paragraph text")),
        ("Multi-line", Text::from("Line 1\nLine 2\nLine 3")),
        ("Styled text", Text::from(Spans::from(vec![
            Span::raw("Normal "),
            Span::styled("Bold", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(" Red", Style::default().fg(Color::Red)),
        ]))),
    ];
    
    for (name, text) in test_texts {
        buffer.reset();
        
        let paragraph = Paragraph::new(&text)
            .block(Block::default().borders(Borders::ALL));
        
        paragraph.render(area, buffer);
        
        // Verify content was rendered
        let content = buffer.content();
        assert!(!content.is_empty(), "Paragraph '{}' produced no content", name);
        
        // Check that text content exists (non-border, non-space cells)
        let has_text_content = content.iter().any(|cell| {
            cell.symbol != " " && 
            cell.symbol != "┌" && cell.symbol != "┐" && 
            cell.symbol != "└" && cell.symbol != "┘" &&
            cell.symbol != "─" && cell.symbol != "│"
        });
        assert!(has_text_content, "Paragraph '{}' has no text content", name);
    }
}

/// Test Table widget rendering compatibility
#[test]
fn test_table_widget_migration_compatibility() {
    let backend = TestBackend::new(40, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    
    let buffer = terminal.current_buffer_mut();
    let area = Rect::new(0, 0, 40, 12);
    
    // Create test table data
    let header = Row::new(vec!["Col1", "Col2", "Col3"]);
    let rows = vec![
        Row::new(vec!["Row1-1", "Row1-2", "Row1-3"]),
        Row::new(vec!["Row2-1", "Row2-2", "Row2-3"]),
        Row::new(vec![
            Cell::from("Styled"),
            Cell::from("Cell").style(Style::default().fg(Color::Yellow)),
            Cell::from("Content"),
        ]),
    ];
    
    buffer.reset();
    
    let table = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL))
        .widths(&[
            Constraint::Length(10),
            Constraint::Length(10), 
            Constraint::Length(10),
        ]);
    
    table.render(area, buffer);
    
    // Verify table was rendered
    let content = buffer.content();
    assert!(!content.is_empty(), "Table produced no content");
    
    // Check for table content (should have header and row data)
    let text_content: Vec<&str> = content.iter()
        .map(|cell| cell.symbol.as_str())
        .filter(|s| !s.is_empty() && *s != " " && !is_border_char(s))
        .collect();
    
    assert!(!text_content.is_empty(), "Table has no text content");
}

/// Test widget style propagation
#[test] 
fn test_widget_style_migration() {
    let backend = TestBackend::new(15, 6);
    let mut terminal = Terminal::new(backend).unwrap();
    
    let buffer = terminal.current_buffer_mut();
    let area = Rect::new(0, 0, 15, 6);
    
    let base_style = Style::default().fg(Color::Green).bg(Color::Blue);
    let text_style = Style::default().fg(Color::Yellow);
    
    buffer.reset();
    
    let text = Text::from(Spans::from(vec![
        Span::styled("Styled", text_style),
        Span::raw(" text"),
    ]));
    
    let paragraph = Paragraph::new(&text)
        .block(Block::default()
            .borders(Borders::ALL)
            .style(base_style));
    
    paragraph.render(area, buffer);
    
    // Verify styles are applied
    let content = buffer.content();
    let styled_cells: Vec<_> = content.iter()
        .filter(|cell| cell.fg == Color::Green || cell.fg == Color::Yellow)
        .collect();
    
    assert!(!styled_cells.is_empty(), "No styled cells found");
}

/// Test widget edge cases and error handling
#[test]
fn test_widget_edge_cases() {
    let backend = TestBackend::new(5, 3);
    let mut terminal = Terminal::new(backend).unwrap();
    
    let buffer = terminal.current_buffer_mut();
    
    // Test zero-size area
    let zero_area = Rect::new(0, 0, 0, 0);
    let block = Block::default().borders(Borders::ALL);
    block.render(zero_area, buffer); // Should not panic
    
    // Test minimal area
    let minimal_area = Rect::new(0, 0, 1, 1);
    let block = Block::default().borders(Borders::ALL);
    block.render(minimal_area, buffer); // Should not panic
    
    // Test paragraph with empty text
    let empty_text = Text::from("");
    let paragraph = Paragraph::new(&empty_text);
    paragraph.render(Rect::new(0, 0, 5, 3), buffer); // Should not panic
    
    // Test table with no rows
    let empty_table = Table::new(vec![])
        .widths(&[Constraint::Length(5)]);
    empty_table.render(Rect::new(0, 0, 5, 3), buffer); // Should not panic
}

/// Test widget layout and positioning
#[test]
fn test_widget_positioning() {
    let backend = TestBackend::new(25, 15);
    let mut terminal = Terminal::new(backend).unwrap();
    
    let buffer = terminal.current_buffer_mut();
    
    // Test rendering at different positions
    let areas = vec![
        Rect::new(0, 0, 10, 5),    // Top-left
        Rect::new(10, 5, 15, 10),  // Bottom-right (overlapping)
        Rect::new(5, 2, 8, 4),     // Middle
    ];
    
    for (i, area) in areas.iter().enumerate() {
        let title = format!("Block {}", i + 1);
        let block = Block::default()
            .borders(Borders::ALL)
            .title(title.as_str());
        
        block.render(*area, buffer);
    }
    
    // Verify multiple blocks were rendered
    let content = buffer.content();
    let border_cells = content.iter()
        .filter(|cell| is_border_char(&cell.symbol))
        .count();
    
    assert!(border_cells > 10, "Expected multiple blocks to be rendered");
}

/// Helper function to check if a string is a border character
fn is_border_char(s: &str) -> bool {
    matches!(s, "┌" | "┐" | "└" | "┘" | "─" | "│" | "├" | "┤" | "┬" | "┴" | "┼")
}
