use helix_tui::buffer::{Buffer, Cell};
use helix_view::graphics::{Color, Style, Rect};
use std::time::Instant;

#[test]
fn test_buffer_creation_performance() {
    // Test buffer creation time for various sizes
    let test_cases = vec![
        (80, 24),      // Standard terminal
        (120, 40),     // Larger terminal  
        (200, 50),     // Very wide terminal
        (300, 100),    // Large terminal
    ];

    for (width, height) in test_cases {
        let start = Instant::now();
        let _buffer = Buffer::empty(Rect::new(0, 0, width, height));
        let duration = start.elapsed();
        
        // Performance baseline: buffer creation should be under 10ms
        assert!(duration.as_millis() < 10, 
                "Buffer creation for {}x{} took {}ms, expected < 10ms", 
                width, height, duration.as_millis());
    }
}

#[test]
fn test_buffer_resize_performance() {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 24));
    
    let resize_operations = vec![
        (120, 40),
        (200, 50), 
        (80, 24),
        (40, 12),
        (300, 100),
    ];

    for (width, height) in resize_operations {
        let start = Instant::now();
        buffer.resize(Rect::new(0, 0, width, height));
        let duration = start.elapsed();
        
        // Performance baseline: resize should be under 5ms
        assert!(duration.as_millis() < 5,
                "Buffer resize to {}x{} took {}ms, expected < 5ms",
                width, height, duration.as_millis());
    }
}

#[test]
fn test_cell_manipulation_performance() {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 200, 100));
    let style = Style::default().fg(Color::Red).bg(Color::Blue);
    
    let start = Instant::now();
    
    // Fill buffer with styled content
    for y in 0..100 {
        for x in 0..200 {
            if let Some(cell) = buffer.get_mut(x, y) {
                cell.set_symbol("X").set_style(style);
            }
        }
    }
    
    let duration = start.elapsed();
    
    // Performance baseline: filling 20k cells should be under 50ms
    assert!(duration.as_millis() < 50,
            "Filling 20,000 cells took {}ms, expected < 50ms",
            duration.as_millis());
}

#[test]
fn test_buffer_diff_performance() {
    let area = Rect::new(0, 0, 100, 50);
    let mut buffer1 = Buffer::empty(area);
    let mut buffer2 = Buffer::empty(area);
    
    // Fill buffers with different content
    for y in 0..50 {
        for x in 0..100 {
            if let Some(cell) = buffer1.get_mut(x, y) {
                cell.set_symbol("A");
            }
            if let Some(cell) = buffer2.get_mut(x, y) {
                cell.set_symbol("B");
            }
        }
    }
    
    let start = Instant::now();
    let _diff = buffer1.diff(&buffer2);
    let duration = start.elapsed();
    
    // Performance baseline: diffing 5k cells should be under 20ms
    assert!(duration.as_millis() < 20,
            "Diffing 5,000 cells took {}ms, expected < 20ms", 
            duration.as_millis());
}

#[test]
fn test_string_rendering_performance() {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 100, 50));
    let style = Style::default().fg(Color::Green);
    
    let test_strings = vec![
        "Short string",
        "This is a much longer string that spans more characters",
        "Unicode: こんにちは世界 🌍 Émojis: 🚀🎉",
        "Mixed: ASCII + Unicode + Émoji 💻 Programming",
    ];
    
    let start = Instant::now();
    
    for (i, test_str) in test_strings.iter().enumerate() {
        let y = i as u16;
        buffer.set_string(0, y, test_str, style);
        buffer.set_string(0, y + 10, test_str, style);
        buffer.set_string(0, y + 20, test_str, style);
        buffer.set_string(0, y + 30, test_str, style);
    }
    
    let duration = start.elapsed();
    
    // Performance baseline: rendering strings should be under 10ms
    assert!(duration.as_millis() < 10,
            "String rendering took {}ms, expected < 10ms",
            duration.as_millis());
}

#[test]
fn test_large_buffer_access_patterns() {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 500, 200));
    
    let start = Instant::now();
    
    // Test random access pattern
    for i in 0..1000 {
        let x = (i * 7) % 500;
        let y = (i * 11) % 200;
        if let Some(cell) = buffer.get_mut(x, y) {
            cell.set_symbol("*");
        }
    }
    
    let duration = start.elapsed();
    
    // Performance baseline: 1000 random accesses should be under 5ms
    assert!(duration.as_millis() < 5,
            "1000 random buffer accesses took {}ms, expected < 5ms",
            duration.as_millis());
}

#[test]
fn test_buffer_clone_performance() {
    let area = Rect::new(0, 0, 150, 75);
    let mut original = Buffer::empty(area);
    let style = Style::default().fg(Color::Cyan);
    
    // Fill with some content
    for y in 0..75 {
        for x in 0..150 {
            if (x + y) % 3 == 0 {
                if let Some(cell) = original.get_mut(x, y) {
                    cell.set_symbol("@").set_style(style);
                }
            }
        }
    }
    
    let start = Instant::now();
    let _cloned = original.clone();
    let duration = start.elapsed();
    
    // Performance baseline: cloning 11.25k cells should be under 20ms  
    assert!(duration.as_millis() < 20,
            "Cloning buffer with 11,250 cells took {}ms, expected < 20ms",
            duration.as_millis());
}
