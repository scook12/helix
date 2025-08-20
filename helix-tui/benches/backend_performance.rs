//! Performance benchmarks for backend implementations
//! 
//! This module benchmarks the performance of different backend implementations
//! to ensure the ratatui adapter doesn't introduce significant overhead.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use helix_tui::{
    backend::{Backend, TestBackend},
    buffer::Cell,
    terminal::Config,
};
use helix_view::graphics::{Color, CursorKind, Modifier, Style, UnderlineStyle};

#[cfg(feature = "ratatui-migration")]
use helix_tui::backend::RatatuiBackendAdapter;

/// Create a test cell with styling for benchmarks
fn create_styled_cell(symbol: &str, fg: Color, bg: Color) -> Cell {
    Cell {
        symbol: symbol.to_string(),
        fg,
        bg,
        modifier: Modifier::BOLD | Modifier::ITALIC,
        underline_color: Color::Cyan,
        underline_style: UnderlineStyle::Line,
    }
}

/// Benchmark native helix-tui TestBackend performance
fn bench_native_backend(c: &mut Criterion) {
    c.bench_function("native_backend_operations", |b| {
        b.iter(|| {
            let mut backend = TestBackend::new(80, 24);
            let config1 = Config { enable_mouse_capture: false };
            let config2 = Config { enable_mouse_capture: false };
            
            // Basic operations
            black_box(backend.claim(config1).unwrap());
            black_box(backend.hide_cursor().unwrap());
            black_box(backend.show_cursor(CursorKind::Block).unwrap());
            black_box(backend.set_cursor(10, 5).unwrap());
            black_box(backend.clear().unwrap());
            black_box(backend.flush().unwrap());
            black_box(backend.restore(config2).unwrap());
        })
    });
}

/// Benchmark drawing performance with native backend
fn bench_native_drawing(c: &mut Criterion) {
    c.bench_function("native_backend_drawing", |b| {
        let mut backend = TestBackend::new(100, 30);
        
        b.iter(|| {
            // Create test content
            let cells: Vec<(u16, u16, Cell)> = (0..100)
                .flat_map(|x| {
                    (0..30).map(move |y| {
                        (x, y, create_styled_cell(
                            &(((x + y) % 26 + 65) as u8 as char).to_string(),
                            Color::Indexed(((x + y) % 256) as u8),
                            Color::Reset,
                        ))
                    })
                })
                .collect();
            
            let cell_refs: Vec<(u16, u16, &Cell)> = cells.iter()
                .map(|(x, y, cell)| (*x, *y, cell))
                .collect();
            
            black_box(backend.draw(cell_refs.into_iter()).unwrap());
            black_box(backend.flush().unwrap());
        })
    });
}

#[cfg(feature = "ratatui-migration")]
/// Benchmark ratatui adapter performance
fn bench_ratatui_adapter(c: &mut Criterion) {
    c.bench_function("ratatui_adapter_operations", |b| {
        b.iter(|| {
            let ratatui_backend = ratatui::backend::TestBackend::new(80, 24);
            let mut adapter = RatatuiBackendAdapter::new(ratatui_backend).unwrap();
            let config1 = Config { enable_mouse_capture: false };
            let config2 = Config { enable_mouse_capture: false };
            
            // Basic operations
            black_box(adapter.claim(config1).unwrap());
            black_box(adapter.hide_cursor().unwrap());
            black_box(adapter.show_cursor(CursorKind::Block).unwrap());
            black_box(adapter.set_cursor(10, 5).unwrap());
            black_box(adapter.clear().unwrap());
            black_box(adapter.flush().unwrap());
            black_box(adapter.restore(config2).unwrap());
        })
    });
}

#[cfg(feature = "ratatui-migration")]
/// Benchmark drawing performance with ratatui adapter
fn bench_ratatui_drawing(c: &mut Criterion) {
    c.bench_function("ratatui_adapter_drawing", |b| {
        let ratatui_backend = ratatui::backend::TestBackend::new(100, 30);
        let mut adapter = RatatuiBackendAdapter::new(ratatui_backend).unwrap();
        
        b.iter(|| {
            // Create test content
            let cells: Vec<(u16, u16, Cell)> = (0..100)
                .flat_map(|x| {
                    (0..30).map(move |y| {
                        (x, y, create_styled_cell(
                            &(((x + y) % 26 + 65) as u8 as char).to_string(),
                            Color::Indexed(((x + y) % 256) as u8),
                            Color::Reset,
                        ))
                    })
                })
                .collect();
            
            let cell_refs: Vec<(u16, u16, &Cell)> = cells.iter()
                .map(|(x, y, cell)| (*x, *y, cell))
                .collect();
            
            black_box(adapter.draw(cell_refs.into_iter()).unwrap());
            black_box(adapter.flush().unwrap());
        })
    });
}

#[cfg(feature = "ratatui-migration")]
/// Benchmark conversion overhead
fn bench_conversion_overhead(c: &mut Criterion) {
    use helix_tui::compat::ratatui_compat::{convert_cell, convert_color, convert_style, convert_rect};
    use helix_view::graphics::Rect;
    
    c.bench_function("conversion_functions", |b| {
        let cell = create_styled_cell("A", Color::Red, Color::Blue);
        let style = Style::default().fg(Color::Green).bg(Color::Yellow).add_modifier(Modifier::BOLD);
        let rect = Rect::new(10, 20, 80, 40);
        let color = Color::Rgb(128, 64, 192);
        
        b.iter(|| {
            black_box(convert_cell(&cell));
            black_box(convert_style(style));
            black_box(convert_rect(rect));
            black_box(convert_color(color));
        })
    });
}

/// Benchmark buffer operations
fn bench_buffer_operations(c: &mut Criterion) {
    c.bench_function("buffer_large_content", |b| {
        let mut backend = TestBackend::new(200, 100);
        
        b.iter(|| {
            // Create large buffer content
            let cells: Vec<(u16, u16, Cell)> = (0..200)
                .flat_map(|x| {
                    (0..100).map(move |y| {
                        (x, y, create_styled_cell(
                            &(((x + y) % 26 + 65) as u8 as char).to_string(),
                            Color::Indexed(((x + y) % 256) as u8),
                            Color::Reset,
                        ))
                    })
                })
                .collect();
            
            let cell_refs: Vec<(u16, u16, &Cell)> = cells.iter()
                .map(|(x, y, cell)| (*x, *y, cell))
                .collect();
            
            black_box(backend.draw(cell_refs.into_iter()).unwrap());
        })
    });
}

// Configure benchmarks based on available features
#[cfg(feature = "ratatui-migration")]
criterion_group!(
    benches,
    bench_native_backend,
    bench_native_drawing,
    bench_ratatui_adapter,
    bench_ratatui_drawing,
    bench_conversion_overhead,
    bench_buffer_operations
);

#[cfg(not(feature = "ratatui-migration"))]
criterion_group!(
    benches,
    bench_native_backend,
    bench_native_drawing,
    bench_buffer_operations
);

criterion_main!(benches);
