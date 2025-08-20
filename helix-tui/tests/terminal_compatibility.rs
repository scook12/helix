use helix_tui::backend::{Backend, TestBackend};
use helix_tui::Terminal;
use helix_view::graphics::{CursorKind, Rect};

#[test]
fn test_terminal_creation_and_size() {
    // Test terminal creation with various sizes
    let test_sizes = vec![
        (1, 1),       // Minimal
        (80, 24),     // Standard terminal
        (120, 40),    // Larger terminal
        (200, 50),    // Very wide
    ];
    
    for (width, height) in test_sizes {
        let backend = TestBackend::new(width, height);
        let terminal = Terminal::new(backend).unwrap();
        let size = terminal.backend().size().unwrap();
        
        assert_eq!(size.width, width);
        assert_eq!(size.height, height);
        assert_eq!(size, Rect::new(0, 0, width, height));
    }
}

#[test]
fn test_terminal_zero_size() {
    // Test edge case: zero size terminal
    let backend = TestBackend::new(0, 0);
    let terminal = Terminal::new(backend).unwrap();
    let size = terminal.backend().size().unwrap();
    
    assert_eq!(size.width, 0);
    assert_eq!(size.height, 0);
    assert_eq!(size.area(), 0);
}

#[test]
fn test_backend_cursor_operations() {
    let mut backend = TestBackend::new(80, 24);
    
    // Test cursor positioning
    assert!(backend.set_cursor(10, 5).is_ok());
    assert_eq!(backend.get_cursor().unwrap(), (10, 5));
    
    // Test cursor at edges
    assert!(backend.set_cursor(0, 0).is_ok());
    assert_eq!(backend.get_cursor().unwrap(), (0, 0));
    
    assert!(backend.set_cursor(79, 23).is_ok());
    assert_eq!(backend.get_cursor().unwrap(), (79, 23));
}

#[test]
fn test_backend_cursor_kinds() {
    let mut backend = TestBackend::new(80, 24);
    
    // Test all cursor kinds work without panic
    let cursor_kinds = vec![
        CursorKind::Block,
        CursorKind::Bar, 
        CursorKind::Underline,
        CursorKind::Hidden,
    ];
    
    for cursor_kind in cursor_kinds {
        assert!(backend.show_cursor(cursor_kind).is_ok());
        assert!(backend.hide_cursor().is_ok());
    }
}

#[test]
fn test_backend_basic_operations() {
    let mut backend = TestBackend::new(80, 24);
    
    // Test basic backend operations don't panic
    assert!(backend.clear().is_ok());
    assert!(backend.flush().is_ok());
    
    // Test initial cursor position
    let (x, y) = backend.get_cursor().unwrap();
    assert!(x < 80);
    assert!(y < 24);
}

#[test]
fn test_terminal_draw_basic() {
    let backend = TestBackend::new(10, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    
    // Test basic draw operation doesn't panic
    let result = terminal.draw(None, CursorKind::Block);
    assert!(result.is_ok());
    
    // Test draw with cursor position
    let result = terminal.draw(Some((5, 5)), CursorKind::Bar);
    assert!(result.is_ok());
}

#[test]
fn test_terminal_multiple_draws() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    
    // Test multiple consecutive draws
    for i in 0..5 {
        let result = terminal.draw(None, CursorKind::Block);
        assert!(result.is_ok(), "Draw iteration {} failed", i);
    }
}
