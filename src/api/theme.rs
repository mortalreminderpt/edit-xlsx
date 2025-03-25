pub struct Theme {
    colors_rgb: Vec<(u8, u8, u8)>,
}

impl Theme {
    pub fn new(colors_rgb: Vec<(u8, u8, u8)>) -> Theme {
        Theme { colors_rgb }
    }

    pub fn theme_to_rgb(&self, color_theme: u8, tint: f64) -> Option<(u8, u8, u8)> {
        self.colors_rgb.get(color_theme as usize).cloned()
    }

    pub fn index_to_rgb(index: u8) -> Option<(u8, u8, u8)> {
        match index {
            0 => Some((0, 0, 0)),        // Black
            1 => Some((255, 255, 255)),  // White
            2 => Some((255, 0, 0)),      // Red
            3 => Some((0, 255, 0)),      // Bright Green
            4 => Some((0, 0, 255)),      // Blue
            5 => Some((255, 255, 0)),    // Yellow
            6 => Some((255, 0, 255)),    // Pink
            7 => Some((0, 255, 255)),    // Turquoise
            8 => Some((0, 0, 0)),        // Black
            9 => Some((255, 255, 255)),  // White
            10 => Some((255, 0, 0)),     // Red
            11 => Some((0, 255, 0)),     // Bright Green
            12 => Some((0, 0, 255)),     // Blue
            13 => Some((255, 255, 0)),   // Yellow
            14 => Some((255, 0, 255)),   // Pink
            15 => Some((0, 255, 255)),   // Turquoise
            16 => Some((128, 0, 0)),     // Dark Red
            17 => Some((0, 128, 0)),     // Green
            18 => Some((0, 0, 128)),     // Dark Blue
            19 => Some((128, 128, 0)),   // Dark Yellow
            20 => Some((128, 0, 128)),   // Violet
            21 => Some((0, 128, 128)),   // Teal
            22 => Some((192, 192, 192)), // Gray-25%
            23 => Some((128, 128, 128)), // Gray-50%
            24 => Some((153, 153, 255)), // Periwinkle
            25 => Some((153, 51, 102)),  // Plum
            26 => Some((255, 255, 204)), // Ivory
            27 => Some((204, 255, 255)), // Light Turquoise
            28 => Some((102, 0, 102)),   // Dark Purple
            29 => Some((255, 128, 128)), // Coral
            30 => Some((0, 102, 204)),   // Ocean Blue
            31 => Some((204, 204, 255)), // Ice Blue
            32 => Some((0, 0, 128)),     // Dark Blue
            33 => Some((255, 0, 255)),   // Pink
            34 => Some((255, 255, 0)),   // Yellow
            35 => Some((0, 255, 255)),   // Turquoise
            36 => Some((128, 0, 128)),   // Violet
            37 => Some((128, 0, 0)),     // Dark Red
            38 => Some((0, 128, 128)),   // Teal
            39 => Some((0, 0, 255)),     // Blue
            40 => Some((0, 204, 255)),   // Sky Blue
            41 => Some((204, 255, 255)), // Light Turquoise
            42 => Some((204, 255, 204)), // Light Green
            43 => Some((255, 255, 153)), // Light Yellow
            44 => Some((153, 204, 255)), // Pale Blue
            45 => Some((255, 153, 204)), // Rose
            46 => Some((204, 153, 255)), // Lavender
            47 => Some((255, 204, 153)), // Tan
            48 => Some((51, 102, 255)),  // Light Blue
            49 => Some((51, 204, 204)),  // Aqua
            50 => Some((153, 204, 0)),   // Lime
            51 => Some((255, 204, 0)),   // Gold
            52 => Some((255, 153, 0)),   // Light Orange
            53 => Some((255, 102, 0)),   // Orange
            54 => Some((102, 102, 153)), // Blue-Gray
            55 => Some((150, 150, 150)), // Gray-Gray40%
            56 => Some((0, 51, 102)),    // Dark Teal
            57 => Some((51, 153, 102)),  // Sea Green
            58 => Some((0, 51, 0)),      // Dark Green
            59 => Some((51, 51, 0)),     // Olive Green
            60 => Some((153, 51, 0)),    // Brown
            61 => Some((153, 51, 102)),  // Plum
            62 => Some((51, 51, 153)),   // Indigo
            63 => Some((51, 51, 51)),    // Gray-80%
            _ => None,                   // Default to Black for unspecified indices
        }
    }
}
