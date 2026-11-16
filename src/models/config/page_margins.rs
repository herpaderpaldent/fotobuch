//! Margins of a page per edge, relative to the binding.

use crate::models::PageSide;

/// Margins of a page in mm, with the horizontal edges named relative to the
/// binding (`inside` = spine edge, `outside` = fore edge).
///
/// Which physical edge is the inside edge follows [`PageSide`]. A page without
/// a side (the cover) uses the same value on all edges, see [`Self::uniform`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PageMargins {
    pub inside: f64,
    pub outside: f64,
    pub top: f64,
    pub bottom: f64,
}

impl PageMargins {
    /// Same margin on all four edges.
    pub fn uniform(margin_mm: f64) -> Self {
        Self {
            inside: margin_mm,
            outside: margin_mm,
            top: margin_mm,
            bottom: margin_mm,
        }
    }

    /// Sum of the left and right margin.
    pub fn horizontal(&self) -> f64 {
        self.inside + self.outside
    }

    /// Sum of the top and bottom margin.
    pub fn vertical(&self) -> f64 {
        self.top + self.bottom
    }

    /// `true` if any edge has a margin. Photos only run into the bleed when
    /// this is `false`.
    pub fn any_positive(&self) -> bool {
        [self.inside, self.outside, self.top, self.bottom]
            .iter()
            .any(|&m| m > 0.0)
    }

    /// `(left, right)` margin of a page on the given side.
    pub fn left_right(&self, side: PageSide) -> (f64, f64) {
        side.to_left_right(self.inside, self.outside)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cewe() -> PageMargins {
        PageMargins {
            inside: 12.0,
            outside: 9.0,
            top: 9.0,
            bottom: 9.0,
        }
    }

    #[test]
    fn inside_margin_is_on_spine_edge() {
        assert_eq!(cewe().left_right(PageSide::Right), (12.0, 9.0));
        assert_eq!(cewe().left_right(PageSide::Left), (9.0, 12.0));
    }

    #[test]
    fn sums_per_axis() {
        assert_eq!(cewe().horizontal(), 21.0);
        assert_eq!(cewe().vertical(), 18.0);
    }

    #[test]
    fn any_positive_only_when_all_zero_is_false() {
        assert!(!PageMargins::uniform(0.0).any_positive());
        assert!(PageMargins::uniform(1.0).any_positive());
        let only_inside = PageMargins {
            inside: 12.0,
            ..PageMargins::uniform(0.0)
        };
        assert!(only_inside.any_positive());
    }
}
