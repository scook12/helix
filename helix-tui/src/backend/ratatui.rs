//! Ratatui backend adapter that implements the helix-tui Backend trait
//! using ratatui's backend implementations

#[cfg(feature = "ratatui-migration")]
pub mod adapter {
    use crate::backend::Backend;
    use crate::buffer::Cell;
    use crate::compat::ratatui_compat::{convert_cell, convert_rect_back};
    use crate::terminal::Config;
    use helix_view::graphics::{CursorKind, Rect};
    use std::io;
    
    /// Adapter that wraps a ratatui backend to implement helix-tui Backend trait
    pub struct RatatuiBackendAdapter<B>
    where
        B: ratatui::backend::Backend,
    {
        backend: B,
        /// Buffer to temporarily hold ratatui cells for conversion
        ratatui_buffer: ratatui::buffer::Buffer,
    }
    
    impl<B> RatatuiBackendAdapter<B>
    where
        B: ratatui::backend::Backend,
    {
        pub fn new(backend: B) -> io::Result<Self> {
            let size = backend.size()?;
            let ratatui_buffer = ratatui::buffer::Buffer::empty(size);
            Ok(Self {
                backend,
                ratatui_buffer,
            })
        }
        
        pub fn inner(&self) -> &B {
            &self.backend
        }
        
        pub fn inner_mut(&mut self) -> &mut B {
            &mut self.backend
        }
        
        /// Resize the internal ratatui buffer to match terminal size
        fn sync_buffer_size(&mut self) -> io::Result<()> {
            let size = self.backend.size()?;
            if self.ratatui_buffer.area != size {
                self.ratatui_buffer.resize(size);
            }
            Ok(())
        }
    }
    
    impl<B> Backend for RatatuiBackendAdapter<B>
    where
        B: ratatui::backend::Backend,
    {
        fn claim(&mut self, config: Config) -> Result<(), io::Error> {
            // For ratatui backends, we need to handle terminal setup manually
            // since ratatui doesn't expose claim/restore methods
            use crossterm::{
                terminal::{enable_raw_mode, EnterAlternateScreen},
                event::{EnableFocusChange, EnableBracketedPaste, EnableMouseCapture},
                execute,
            };
            
            enable_raw_mode()?;
            let mut stdout = std::io::stdout();
            execute!(stdout, EnterAlternateScreen, EnableFocusChange)?;
            
            // Enable bracketed paste
            if let Err(err) = execute!(stdout, EnableBracketedPaste) {
                if err.kind() != io::ErrorKind::Unsupported {
                    return Err(err);
                }
            }
            
            // Enable mouse capture if requested
            if config.enable_mouse_capture {
                execute!(stdout, EnableMouseCapture)?;
            }
            
            Ok(())
        }
        
        fn reconfigure(&mut self, config: Config) -> Result<(), io::Error> {
            // Handle mouse capture configuration changes
            use crossterm::{event::{EnableMouseCapture, DisableMouseCapture}, execute};
            
            let mut stdout = std::io::stdout();
            if config.enable_mouse_capture {
                execute!(stdout, EnableMouseCapture)?;
            } else {
                execute!(stdout, DisableMouseCapture)?;
            }
            
            Ok(())
        }
        
        fn restore(&mut self, config: Config) -> Result<(), io::Error> {
            // Restore terminal to normal state
            use crossterm::{
                terminal::{disable_raw_mode, LeaveAlternateScreen},
                event::{DisableFocusChange, DisableBracketedPaste, DisableMouseCapture},
                execute,
            };
            
            let mut stdout = std::io::stdout();
            
            // Disable mouse capture if it was enabled
            if config.enable_mouse_capture {
                execute!(stdout, DisableMouseCapture)?;
            }
            
            // Disable other features
            execute!(stdout, DisableBracketedPaste)?;
            execute!(stdout, DisableFocusChange, LeaveAlternateScreen)?;
            
            disable_raw_mode()?;
            Ok(())
        }
        
        fn force_restore() -> Result<(), io::Error> {
            // Force restore - ignore errors and restore as much as possible
            use crossterm::{
                terminal::{disable_raw_mode, LeaveAlternateScreen},
                event::{DisableFocusChange, DisableBracketedPaste, DisableMouseCapture},
                execute,
            };
            
            let mut stdout = std::io::stdout();
            
            // Ignore individual errors but try to restore everything
            let _ = execute!(stdout, DisableMouseCapture);
            let _ = execute!(stdout, DisableBracketedPaste);
            let _ = execute!(stdout, DisableFocusChange, LeaveAlternateScreen);
            let _ = disable_raw_mode();
            
            Ok(())
        }
        
        fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
        where
            I: Iterator<Item = (u16, u16, &'a Cell)>,
        {
            // Ensure buffer size matches backend size
            self.sync_buffer_size()?;
            
            // Convert helix-tui cells to ratatui cells and update buffer
            for (x, y, helix_cell) in content {
                // Use ratatui's buffer coordinate system to set cells
                let ratatui_cell = convert_cell(helix_cell);
                if x < self.ratatui_buffer.area.width && y < self.ratatui_buffer.area.height {
                    let index = (y as usize * self.ratatui_buffer.area.width as usize) + x as usize;
                    if index < self.ratatui_buffer.content.len() {
                        self.ratatui_buffer.content[index] = ratatui_cell;
                    }
                }
            }
            
            // Draw the buffer content through ratatui backend
            let buffer_content = self.ratatui_buffer.content.iter()
                .enumerate()
                .map(|(i, cell)| {
                    let x = (i % self.ratatui_buffer.area.width as usize) as u16;
                    let y = (i / self.ratatui_buffer.area.width as usize) as u16;
                    (x, y, cell)
                });
            
            self.backend.draw(buffer_content)
        }
        
        fn hide_cursor(&mut self) -> Result<(), io::Error> {
            self.backend.hide_cursor()
        }
        
        fn show_cursor(&mut self, kind: CursorKind) -> Result<(), io::Error> {
            // ratatui's show_cursor doesn't take parameters, so we ignore the kind
            // This is a limitation of the compatibility layer
            match kind {
                CursorKind::Hidden => self.backend.hide_cursor(),
                _ => self.backend.show_cursor(),
            }
        }
        
        fn get_cursor(&mut self) -> Result<(u16, u16), io::Error> {
            self.backend.get_cursor()
        }
        
        fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), io::Error> {
            self.backend.set_cursor(x, y)
        }
        
        fn clear(&mut self) -> Result<(), io::Error> {
            self.backend.clear()
        }
        
        fn size(&self) -> Result<Rect, io::Error> {
            let ratatui_size = self.backend.size()?;
            Ok(convert_rect_back(ratatui_size))
        }
        
        fn flush(&mut self) -> Result<(), io::Error> {
            self.backend.flush()
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        use ratatui::backend::TestBackend;
        
        #[test]
        fn test_ratatui_adapter_creation() {
            let ratatui_backend = TestBackend::new(80, 24);
            let adapter = RatatuiBackendAdapter::new(ratatui_backend);
            assert!(adapter.is_ok());
            
            let adapter = adapter.unwrap();
            let size = adapter.size().unwrap();
            assert_eq!(size.width, 80);
            assert_eq!(size.height, 24);
        }
        
        #[test]
        fn test_backend_operations() {
            let ratatui_backend = TestBackend::new(40, 20);
            let mut adapter = RatatuiBackendAdapter::new(ratatui_backend).unwrap();
            
            // Test basic operations don't panic
            assert!(adapter.claim(Config { enable_mouse_capture: false }).is_ok());
            assert!(adapter.clear().is_ok());
            assert!(adapter.flush().is_ok());
            assert!(adapter.hide_cursor().is_ok());
            assert!(adapter.show_cursor(CursorKind::Block).is_ok());
            
            // Test size
            let size = adapter.size().unwrap();
            assert_eq!(size.width, 40);
            assert_eq!(size.height, 20);
        }
        
        #[test]
        fn test_cursor_operations() {
            let ratatui_backend = TestBackend::new(30, 15);
            let mut adapter = RatatuiBackendAdapter::new(ratatui_backend).unwrap();
            
            // Test cursor positioning
            assert!(adapter.set_cursor(10, 5).is_ok());
            let (x, y) = adapter.get_cursor().unwrap();
            assert_eq!(x, 10);
            assert_eq!(y, 5);
            
            // Test cursor kinds
            assert!(adapter.show_cursor(CursorKind::Bar).is_ok());
            assert!(adapter.show_cursor(CursorKind::Underline).is_ok());
            assert!(adapter.show_cursor(CursorKind::Hidden).is_ok());
        }
    }
}

#[cfg(not(feature = "ratatui-migration"))]
pub mod adapter {
    //! Stub module when ratatui migration feature is disabled
}
