use super::PageMargins;

pub trait CanvasConfig {
    fn page_width_mm(&self) -> f64;
    fn page_height_mm(&self) -> f64;
    fn bleed_mm(&self) -> f64;
    /// Margins per edge. Canvas size and bleed handling are the same on left-
    /// and right-hand pages, so only the sums and `any_positive` matter here.
    fn margins(&self) -> PageMargins;
    fn gap_mm(&self) -> f64;
    fn bleed_threshold_mm(&self) -> f64;
}
