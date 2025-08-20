use crate::compositor::{Component, Context};
use helix_view::graphics::Rect;
use helix_view::info::Info;
use tui::buffer::Buffer as Surface;

#[cfg(feature = "ratatui-migration")]
use {
    ratatui::widgets::Block,
    tui::compat::ratatui_compat::{convert_style, render_ratatui_widget},
};

#[cfg(not(feature = "ratatui-migration"))]
use {
    tui::widgets::{Block, Paragraph, Widget},
    tui::text::Text,
    helix_view::graphics::Margin,
};

impl Component for Info {
    fn render(&mut self, viewport: Rect, surface: &mut Surface, cx: &mut Context) {
        let text_style = cx.editor.theme.get("ui.text.info");
        let popup_style = cx.editor.theme.get("ui.popup.info");

        // Calculate the area of the terminal to modify. Because we want to
        // render at the bottom right, we use the viewport's width and height
        // which evaluate to the most bottom right coordinate.
        let width = self.width + 2 + 2; // +2 for border, +2 for margin
        let height = self.height + 2; // +2 for border
        let area = viewport.intersection(Rect::new(
            viewport.width.saturating_sub(width),
            viewport.height.saturating_sub(height + 2), // +2 for statusline
            width,
            height,
        ));
        surface.clear_with(area, popup_style);

        #[cfg(not(feature = "ratatui-migration"))]
        let inner = {
            let block = Block::bordered()
                .title(self.title.as_ref())
                .border_style(popup_style);

            let margin = Margin::horizontal(1);
            let inner = block.inner(area).inner(margin);
            block.render(area, surface);
            inner
        };

        #[cfg(feature = "ratatui-migration")]
        let inner = {
            // Create ratatui Block with title (title is Cow<'static, str>, not Option)
            let block = Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .title(self.title.as_ref()) // title is always present
                .border_style(convert_style(popup_style));

            // Calculate inner area with margin (need to convert types)
            let ratatui_area = tui::compat::ratatui_compat::convert_rect(area);
            let ratatui_inner = block.inner(ratatui_area);
            
            // Apply margin using ratatui's margin system
            let margin = ratatui::layout::Margin { horizontal: 1, vertical: 0 };
            let ratatui_inner_with_margin = ratatui_inner.inner(&margin);
            
            // Convert back to helix rect for compatibility
            let inner = tui::compat::ratatui_compat::convert_rect_back(ratatui_inner_with_margin);
            
            // Render the ratatui Block widget
            render_ratatui_widget(block, area, surface);
            inner
        };

        #[cfg(not(feature = "ratatui-migration"))]
        {
            Paragraph::new(&Text::from(self.text.as_str()))
                .style(text_style)
                .render(inner, surface);
        }
        
        #[cfg(feature = "ratatui-migration")]
        {
            // Use ratatui Paragraph with converted text and style
            let ratatui_text = ratatui::text::Text::from(self.text.as_str());
            let ratatui_style = convert_style(text_style);
            let paragraph = ratatui::widgets::Paragraph::new(ratatui_text)
                .style(ratatui_style);
            
            render_ratatui_widget(paragraph, inner, surface);
        }
    }
}
