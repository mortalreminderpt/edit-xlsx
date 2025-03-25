use crate::api::theme::Theme;
use crate::WorkSheet;

impl WorkSheet {
    pub fn get_theme(&self, theme_id: u8) -> Option<Theme> {
        let binding = self.themes.borrow();
        binding
            .themes
            .get(theme_id as usize)
            .map(|theme| theme.to_api_theme())
    }
}
