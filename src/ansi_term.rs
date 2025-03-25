use crate::api::worksheet::WorkSheet;

use crate::api::cell::location::{Location, LocationRange};
use crate::api::cell::values::CellDisplay;
use crate::{FormatFill, FormatFont, Read, WorkSheetCol, WorkSheetResult, WorkSheetRow};
use ansi_term::{ANSIString, Colour, Style};

impl WorkSheet {
    pub fn ansi_strings<L: Location>(&self, loc: L) -> WorkSheetResult<Vec<ANSIString>> {
        fn add_ansi_font_style(worksheet: &WorkSheet, format_font: &FormatFont, style: &mut Style) {
            style.is_bold |= format_font.bold;
            style.is_italic |= format_font.italic;
            style.is_underline |= format_font.underline;
            style.foreground = format_font.color.to_rgb(worksheet).map(|color| {
                let (r, g, b) = color;
                Colour::RGB(r, g, b)
            });
        }
        fn add_ansi_color_style(
            worksheet: &WorkSheet,
            format_fill: &FormatFill,
            style: &mut Style,
        ) {
            style.background = format_fill.fg_color.to_rgb(worksheet).map(|color| {
                let (r, g, b) = color;
                Colour::RGB(r, g, b)
            });
        }

        let (row, col) = loc.to_location();
        let mut style = Style::new();

        let (_, format) = self.get_row_with_format(row)?;
        if let Some(format) = format {
            add_ansi_font_style(self, &format.font, &mut style);
            add_ansi_color_style(self, &format.fill, &mut style);
        }

        let col_range = (row, col, row, col).to_col_range_ref();
        let col_map = self.get_columns_with_format(&col_range)?;
        if let Some((_, Some(format))) = col_map.get(&col_range) {
            add_ansi_font_style(self, &format.font, &mut style);
            add_ansi_color_style(self, &format.fill, &mut style);
        }

        let mut ansi_string_vec = Vec::new();
        if let Ok(cell) = self.read_cell((row, col)) {
            if let Some(format) = cell.format {
                add_ansi_font_style(self, &format.font, &mut style);
                add_ansi_color_style(self, &format.fill, &mut style);
            }
            let text = if let Some(text) = &cell.text {
                text.to_display()
            } else {
                "".to_string()
            };
            if let Some(rich_text) = &cell.rich_text {
                rich_text.words.iter().for_each(|w| {
                    let mut style = style;
                    if let Some(format_font) = &w.font {
                        add_ansi_font_style(self, &format_font, &mut style);
                    }
                    ansi_string_vec.push(style.paint(w.text.clone()));
                });
            } else {
                ansi_string_vec.push(style.paint(text));
            }
        }
        Ok(ansi_string_vec)
    }
}
