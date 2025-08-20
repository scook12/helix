//! `widgets` module now primarily handles Table widgets and compatibility
//! 
//! Most widgets (Block, Paragraph, List) have been migrated to use ratatui
//! directly through the compatibility layer. The Table widget and related
//! types are still maintained here for helix-specific functionality.

mod reflow;
mod table;

pub use self::table::{Cell, Row, Table, TableState};

use crate::buffer::Buffer;
use bitflags::bitflags;

use helix_view::graphics::Rect;

bitflags! {
    /// Bitflags that can be composed to set the visible borders essentially on the block widget.
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
    pub struct Borders: u8 {
        /// Show the top border
        const TOP = 0b0000_0001;
        /// Show the right border
        const RIGHT = 0b0000_0010;
        /// Show the bottom border
        const BOTTOM = 0b000_0100;
        /// Show the left border
        const LEFT = 0b0000_1000;
        /// Show all borders
        const ALL = Self::TOP.bits() | Self::RIGHT.bits() | Self::BOTTOM.bits() | Self::LEFT.bits();
    }
}

/// Border type for blocks - kept for compatibility with ratatui conversion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderType {
    Plain,
    Rounded,
    Double,
    Thick,
}

impl Default for BorderType {
    fn default() -> BorderType {
        BorderType::Plain
    }
}

impl BorderType {
    /// Compatibility method for line_symbols
    pub fn line_symbols(border_type: BorderType) -> BorderType {
        border_type
    }
    
    /// Get horizontal border character for compatibility
    pub fn horizontal(&self) -> &'static str {
        "-"  // Simple horizontal line for compatibility
    }
}

/// Text wrapping configuration - kept for compatibility 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Wrap {
    pub trim: bool,
}

impl Default for Wrap {
    fn default() -> Wrap {
        Wrap { trim: false }
    }
}

/// Minimal Block struct for compatibility - most Block functionality
/// is handled by ratatui directly through the compatibility layer
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Block<'a> {
    pub title: Option<&'a str>,
    pub borders: Borders,
    pub border_type: BorderType,
    pub style: helix_view::graphics::Style,
}

impl<'a> Block<'a> {
    pub fn new() -> Self {
        Block::default()
    }
    
    /// Compatibility method for bordered blocks
    pub fn bordered() -> Self {
        Block::new().borders(Borders::ALL)
    }
    
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }
    
    pub fn borders(mut self, borders: Borders) -> Self {
        self.borders = borders;
        self
    }
    
    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }
    
    pub fn style(mut self, style: helix_view::graphics::Style) -> Self {
        self.style = style;
        self
    }
    
    pub fn border_style(mut self, style: helix_view::graphics::Style) -> Self {
        self.style = style;
        self
    }
    
    /// Calculate inner area - simplified version for compatibility
    pub fn inner(&self, area: Rect) -> Rect {
        let mut inner = area;
        if self.borders.contains(Borders::TOP) {
            inner.y = inner.y.saturating_add(1);
            inner.height = inner.height.saturating_sub(1);
        }
        if self.borders.contains(Borders::BOTTOM) {
            inner.height = inner.height.saturating_sub(1);
        }
        if self.borders.contains(Borders::LEFT) {
            inner.x = inner.x.saturating_add(1);
            inner.width = inner.width.saturating_sub(1);
        }
        if self.borders.contains(Borders::RIGHT) {
            inner.width = inner.width.saturating_sub(1);
        }
        inner
    }
}

impl<'a> Widget for Block<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Delegate to ratatui for actual rendering
        use crate::compat::ratatui_compat::{render_ratatui_widget, convert_borders, convert_border_type, convert_style};
        
        let mut ratatui_block = ratatui::widgets::Block::new()
            .borders(convert_borders(self.borders))
            .border_type(convert_border_type(self.border_type))
            .style(convert_style(self.style));
            
        if let Some(title) = self.title {
            ratatui_block = ratatui_block.title(title);
        }
        
        render_ratatui_widget(ratatui_block, area, buf);
    }
}

/// Minimal Paragraph struct for compatibility - delegates to ratatui
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Paragraph<'a> {
    pub text: crate::text::Text<'a>,
    pub block: Option<Block<'a>>,
    pub style: helix_view::graphics::Style,
    pub wrap: Option<Wrap>,
    pub alignment: Alignment,
    pub scroll: (u16, u16),
}

/// Text alignment for paragraphs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

impl Default for Alignment {
    fn default() -> Alignment {
        Alignment::Left
    }
}

impl<'a> Paragraph<'a> {
    pub fn new<T>(text: T) -> Self
    where
        T: Into<crate::text::Text<'a>>,
    {
        Paragraph {
            text: text.into(),
            block: None,
            style: helix_view::graphics::Style::default(),
            wrap: None,
            alignment: Alignment::Left,
            scroll: (0, 0),
        }
    }
    
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
    
    pub fn style(mut self, style: helix_view::graphics::Style) -> Self {
        self.style = style;
        self
    }
    
    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = Some(wrap);
        self
    }
    
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
    
    pub fn scroll(mut self, offset: (u16, u16)) -> Self {
        self.scroll = offset;
        self
    }
    
    /// Compatibility method for required_size - simplified implementation
    pub fn required_size(&self, _max_width: u16) -> (u16, u16) {
        // Return a conservative estimate for compatibility
        // In a real implementation, this would calculate based on text content
        (1, 1)
    }
}

impl<'a> Widget for Paragraph<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Delegate to ratatui for actual rendering
        use crate::compat::ratatui_compat::{render_ratatui_widget, convert_text, convert_style, convert_wrap, convert_borders, convert_border_type};
        
        let ratatui_text = convert_text(self.text);
        let mut ratatui_paragraph = ratatui::widgets::Paragraph::new(ratatui_text)
            .style(convert_style(self.style))
            .scroll(self.scroll);
            
        // Convert alignment
        let ratatui_alignment = match self.alignment {
            Alignment::Left => ratatui::layout::Alignment::Left,
            Alignment::Center => ratatui::layout::Alignment::Center,
            Alignment::Right => ratatui::layout::Alignment::Right,
        };
        ratatui_paragraph = ratatui_paragraph.alignment(ratatui_alignment);
        
        if let Some(wrap) = self.wrap {
            ratatui_paragraph = ratatui_paragraph.wrap(convert_wrap(wrap));
        }
        
        if let Some(block) = self.block {
            let ratatui_block = ratatui::widgets::Block::new()
                .borders(convert_borders(block.borders))
                .border_type(convert_border_type(block.border_type))
                .style(convert_style(block.style));
            let ratatui_block = if let Some(title) = block.title {
                ratatui_block.title(title)
            } else {
                ratatui_block
            };
            ratatui_paragraph = ratatui_paragraph.block(ratatui_block);
        }
        
        render_ratatui_widget(ratatui_paragraph, area, buf);
    }
}

/// Base requirements for a Widget
pub trait Widget {
    /// Draws the current state of the widget in the given buffer. That the only method required to
    /// implement a custom widget.
    fn render(self, area: Rect, buf: &mut Buffer);
}
