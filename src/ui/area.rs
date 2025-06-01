use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct Area {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColumnLayout {
    pub width: u16,
    pub spacing: u16,
    pub centred: bool,
}

impl std::default::Default for ColumnLayout {
    fn default() -> Self {
        Self {
            width: 40,
            spacing: 5,
            centred: true,
        }
    }
}

impl Area {
    pub fn split_horizontally(
        &self,
        column_layout: &ColumnLayout,
        max_num_columns: &u16,
    ) -> Vec<Area> {
        let mut areas = Vec::new();

        if column_layout.width == 0 {
            return areas; // avoid division by zero
        }

        let total_column_space = column_layout.width + column_layout.spacing;
        let num_columns =
            ((self.width + column_layout.spacing) / total_column_space).min(*max_num_columns);

        if num_columns == 0 {
            return areas;
        }

        let mut x = self.x;
        if column_layout.centred {
            let total_width =
                num_columns * column_layout.width + (num_columns - 1) * column_layout.spacing;
            x += (self.width.saturating_sub(total_width)) / 2;
        }

        for _ in 0..num_columns {
            areas.push(Area {
                x,
                y: self.y,
                width: column_layout.width,
                height: self.height,
            });
            x += column_layout.width + column_layout.spacing;
        }

        areas
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_no_centering() {
        let area = Area {
            x: 0,
            y: 0,
            width: 50,
            height: 10,
        };
        let layout = ColumnLayout {
            width: 10,
            spacing: 2,
            centred: false,
        };
        let result = area.split_horizontally(&layout, &10);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].x, 0);
        assert_eq!(result[1].x, 12);
        assert_eq!(result[2].x, 24);
        assert_eq!(result[3].x, 36);
    }

    #[test]
    fn test_split_centered() {
        let area = Area {
            x: 0,
            y: 0,
            width: 52,
            height: 10,
        };
        let layout = ColumnLayout {
            width: 10,
            spacing: 2,
            centred: true,
        };
        let result = area.split_horizontally(&layout, &10);
        assert_eq!(result.len(), 4);
        // The total width of all columns including spacing is 46 (4 * 10 + 3 * 2)
        // The area is 52, so the padding on each side is (52 - 46) / 2 = 3
        assert_eq!(result[0].x, 3);
    }

    #[test]
    fn test_split_too_small() {
        let area = Area {
            x: 0,
            y: 0,
            width: 5,
            height: 10,
        };
        let layout = ColumnLayout {
            width: 10,
            spacing: 2,
            centred: false,
        };
        let result = area.split_horizontally(&layout, &10);
        assert!(result.is_empty());
    }

    #[test]
    fn test_zero_width_column() {
        let area = Area {
            x: 0,
            y: 0,
            width: 50,
            height: 10,
        };
        let layout = ColumnLayout {
            width: 0,
            spacing: 2,
            centred: false,
        };
        let result = area.split_horizontally(&layout, &10);
        assert!(result.is_empty());
    }

    #[test]
    fn test_max_columns() {
        let area = Area {
            x: 0,
            y: 0,
            width: 100,
            height: 10,
        };
        let layout = ColumnLayout {
            width: 20,
            spacing: 5,
            centred: false,
        };
        let result = area.split_horizontally(&layout, &3);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].x, 0);
        assert_eq!(result[1].x, 25);
        assert_eq!(result[2].x, 50);
    }

    #[test]
    fn test_max_columns_centred() {
        let area = Area {
            x: 0,
            y: 0,
            width: 100,
            height: 10,
        };
        let layout = ColumnLayout {
            width: 20,
            spacing: 5,
            centred: true,
        };
        let result = area.split_horizontally(&layout, &3);
        assert_eq!(result.len(), 3);
        // The total width of all columns including spacing is 70 (3 * 20 + 2 * 5)
        // The area is 100, so the padding on each side is (100 - 70) / 2 = 15
        assert_eq!(result[0].x, 15);
    }
}
