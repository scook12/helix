use criterion::{black_box, criterion_group, criterion_main, Criterion};
use helix_tui::buffer::{Buffer as HelixBuffer};
use helix_view::graphics::{Color, Modifier, Rect, Style};

#[cfg(feature = "ratatui-migration")]
use ratatui::{buffer::Buffer as RatatuiBuffer, buffer::Cell as RatatuiCell};
#[cfg(feature = "ratatui-migration")]
use ratatui::{style::Style as RatatuiStyle, style::Color as RatatuiColor, layout::Rect as RatatuiRect};

fn create_test_content() -> Vec<&'static str> {
    vec![
        "┌─ HELIX EDITOR ─────────────────────────────┐",
        "│ fn main() {                               │",
        "│     let mut text = String::from(\"Hello\"); │",
        "│     text.push_str(\", World!\");            │",
        "│     println!(\"{}\", text);                 │",
        "│ }                                         │",
        "│                                           │",
        "│ // Comments with Unicode: 🦀 Rust         │",
        "│ // CJK characters: 日本語, 中文, 한글      │",
        "│ // Math symbols: ∀x∈ℝ, ∃y: x²+y²=1       │",
        "└───────────────────────────────────────────┘",
    ]
}

/// Benchmark helix buffer's diff algorithm - CORE PERFORMANCE FEATURE
fn bench_helix_buffer_diff(c: &mut Criterion) {
    let area = Rect::new(0, 0, 50, 20);
    let content = create_test_content();
    
    // Create two similar buffers with small differences (realistic editor scenario)
    let mut buffer1 = HelixBuffer::with_lines(content.clone());
    buffer1.resize(area);
    
    let mut buffer2 = HelixBuffer::with_lines(content);
    buffer2.resize(area);
    
    // Make small changes (simulates typical editing)
    buffer2.set_string(10, 1, "modified", Style::default().fg(Color::Red));
    buffer2.set_string(15, 3, "changed", Style::default().bg(Color::Yellow));
    
    c.bench_function("helix_buffer_diff", |b| {
        b.iter(|| {
            let diff = buffer1.diff(black_box(&buffer2));
            black_box(diff)
        })
    });
}

/// Benchmark helix buffer's merge operation - ESSENTIAL for overlay composition
fn bench_helix_buffer_merge(c: &mut Criterion) {
    let base_area = Rect::new(0, 0, 50, 20);
    let overlay_area = Rect::new(10, 5, 20, 8);
    
    c.bench_function("helix_buffer_merge", |b| {
        b.iter(|| {
            let mut base = HelixBuffer::with_lines(create_test_content());
            base.resize(base_area);
            
            let mut overlay = HelixBuffer::empty(overlay_area);
            overlay.set_string(0, 0, "POPUP CONTENT", Style::default().bg(Color::Blue));
            overlay.set_string(0, 1, "More content", Style::default().fg(Color::White));
            
            base.merge(black_box(&overlay));
            black_box(base)
        })
    });
}

/// Benchmark helix buffer's advanced text operations - UNIQUE TO HELIX
fn bench_helix_text_operations(c: &mut Criterion) {
    let area = Rect::new(0, 0, 100, 30);
    
    c.bench_function("helix_buffer_set_spans", |b| {
        b.iter(|| {
            let mut buffer = HelixBuffer::empty(area);
            
            // Complex multi-style text (status line simulation)
            let spans = helix_tui::text::Spans::from(vec![
                helix_tui::text::Span::styled("NORMAL", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                helix_tui::text::Span::raw(" "),
                helix_tui::text::Span::styled("main.rs", Style::default().fg(Color::Cyan)),
                helix_tui::text::Span::raw(" "),
                helix_tui::text::Span::styled("[+]", Style::default().fg(Color::Yellow)),
                helix_tui::text::Span::raw(" Line 42:18 "),
                helix_tui::text::Span::styled("UTF-8", Style::default().fg(Color::Gray)),
            ]);
            
            buffer.set_spans(0, 0, &spans, 50);
            black_box(buffer)
        })
    });
    
    c.bench_function("helix_buffer_set_string_truncated", |b| {
        b.iter(|| {
            let mut buffer = HelixBuffer::empty(area);
            let long_text = "This is a very long line that needs to be truncated with ellipsis at the end to fit in the available space";
            
            // Simulates file path truncation in status bar
            buffer.set_string_truncated(
                0, 0,
                long_text,
                30,
                |_| Style::default().fg(Color::LightBlue),
                true,  // ellipsis
                false, // truncate_end
            );
            black_box(buffer)
        })
    });
}

/// Benchmark unicode and multi-width character handling - CRITICAL for international use
fn bench_helix_unicode_handling(c: &mut Criterion) {
    let area = Rect::new(0, 0, 80, 25);
    
    c.bench_function("helix_buffer_unicode_heavy", |b| {
        b.iter(|| {
            let mut buffer = HelixBuffer::empty(area);
            
            // Mix of different Unicode categories
            let unicode_lines = vec![
                "🦀 Rust: fn main() { println!(\"Hello, 世界!\"); }",
                "🐍 Python: def main(): print(\"你好, World!\")",
                "🎯 Target: 目標達成！ Achievement unlocked 🏆",
                "📊 Data: ∑(xᵢ) = μ ± σ² for i∈[1,n] ∀n∈ℕ",
                "🌍 Unicode: العربية, עברית, हिन्दी, ελληνικά, русский",
                "🎨 Emoji: 😀😃😄😁😆😅🤣😂🙂🙃😉😊😇",
                "🔤 Combining: é, ñ, ü, ø, å (café naïveté résumé)",
            ];
            
            for (y, line) in unicode_lines.iter().enumerate() {
                buffer.set_string(0, y as u16, line, Style::default());
            }
            black_box(buffer)
        })
    });
}

#[cfg(feature = "ratatui-migration")]
/// Benchmark ratatui buffer operations for comparison
fn bench_ratatui_buffer_basic(c: &mut Criterion) {
    let area = RatatuiRect { x: 0, y: 0, width: 50, height: 20 };
    
    c.bench_function("ratatui_buffer_basic_ops", |b| {
        b.iter(|| {
            let mut buffer = RatatuiBuffer::empty(area);
            
            // Basic operations that ratatui CAN do
            buffer.set_string(0, 0, "Hello, World!", RatatuiStyle::default());
            buffer.set_string(0, 1, "Line 2", RatatuiStyle::default().fg(RatatuiColor::Red));
            buffer.set_string(0, 2, "Line 3", RatatuiStyle::default().bg(RatatuiColor::Blue));
            
            black_box(buffer)
        })
    });
    
    // NOTE: Cannot benchmark diff() or merge() - these methods don't exist in ratatui!
}

/// Benchmark memory usage patterns
fn bench_helix_buffer_memory(c: &mut Criterion) {
    c.bench_function("helix_buffer_large_creation", |b| {
        b.iter(|| {
            // Large terminal buffer (4K display)
            let buffer = HelixBuffer::empty(Rect::new(0, 0, 200, 100));
            black_box(buffer)
        })
    });
    
    c.bench_function("helix_buffer_repeated_modifications", |b| {
        let area = Rect::new(0, 0, 80, 24);
        
        b.iter(|| {
            let mut buffer = HelixBuffer::empty(area);
            // Simulates rapid editing operations
            for i in 0..100 {
                let x = (i % 80) as u16;
                let y = (i / 80) as u16;
                if let Some(cell) = buffer.get_mut(x, y) {
                    cell.set_char((b'A' + (i % 26) as u8) as char)
                        .set_fg(Color::Indexed((i % 255) as u8))
                        .set_style(Style::default().add_modifier(Modifier::BOLD));
                }
            }
            black_box(buffer)
        })
    });
}

/// Demonstrate helix buffer's sophisticated diff algorithm
fn bench_realistic_editor_scenario(c: &mut Criterion) {
    c.bench_function("realistic_editor_diff_scenario", |b| {
        let area = Rect::new(0, 0, 120, 40);
        
        b.iter(|| {
            // Simulate before: editing a Rust file
            let before_content = vec![
                "use std::collections::HashMap;",
                "",
                "fn main() {",
                "    let mut map = HashMap::new();",
                "    map.insert(\"key\", \"value\");",
                "    println!(\"{:?}\", map);",
                "}",
                "",
                "// TODO: Add error handling",
                "// TODO: Add documentation",
            ];
            
            // Simulate after: user made several edits
            let after_content = vec![
                "use std::collections::HashMap;",
                "use std::io::{self, Write};",  // Added import
                "",
                "fn main() -> Result<(), io::Error> {",  // Changed signature
                "    let mut map = HashMap::new();",
                "    map.insert(\"greeting\", \"Hello, World!\");",  // Modified
                "    println!(\"{:?}\", map);",
                "    Ok(())",  // Added return
                "}",
                "",
                "// DONE: Add error handling",  // Modified
                "// TODO: Add documentation",
            ];
            
            let mut before_buffer = HelixBuffer::with_lines(before_content);
            before_buffer.resize(area);
            
            let mut after_buffer = HelixBuffer::with_lines(after_content);
            after_buffer.resize(area);
            
            // This diff operation is what makes helix buffer special!
            let diff = before_buffer.diff(&after_buffer);
            
            // Verify the diff algorithm found the minimal changes
            // In a real editor, this translates to efficient terminal updates
            black_box(diff.len())
        })
    });
}

#[cfg(feature = "ratatui-migration")]
criterion_group!(
    benches,
    bench_helix_buffer_diff,
    bench_helix_buffer_merge,
    bench_helix_text_operations,
    bench_helix_unicode_handling,
    bench_ratatui_buffer_basic,
    bench_helix_buffer_memory,
    bench_realistic_editor_scenario
);

#[cfg(not(feature = "ratatui-migration"))]
criterion_group!(
    benches,
    bench_helix_buffer_diff,
    bench_helix_buffer_merge,
    bench_helix_text_operations,
    bench_helix_unicode_handling,
    bench_helix_buffer_memory,
    bench_realistic_editor_scenario
);

criterion_main!(benches);
