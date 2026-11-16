use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::info;

use crate::models::PageMargins;
use crate::output::typst::{self, BleedSpec};
use crate::state_manager::StateManager;

/// Data captured from `StateManager` before it is consumed by `finish`/`finish_always`.
pub struct RenderContext {
    pub project_root: PathBuf,
    pub project_name: String,
    pub bleed: BleedSpec,
    /// `true` if inner pages have margins that differ per edge.
    pub per_edge_margins: bool,
}

impl RenderContext {
    pub fn capture(mgr: &StateManager) -> Self {
        let book = &mgr.state.config.book;
        Self {
            project_root: mgr.project_root().to_owned(),
            project_name: mgr.project_name().to_string(),
            bleed: BleedSpec::from_book(&mgr.state.config.book),
            per_edge_margins: book.margins() != PageMargins::uniform(book.margin_mm),
        }
    }
}

pub enum PdfTarget {
    Preview,
    Final,
}

/// Renders the PDF or returns only the output path when `skip_pdf` is true.
pub fn render_pdf(ctx: &RenderContext, target: PdfTarget, skip_pdf: bool) -> Result<PathBuf> {
    let root = &ctx.project_root;
    let name = &ctx.project_name;
    let bleed = &ctx.bleed;
    if skip_pdf {
        return Ok(root.join(format!("{name}.pdf")));
    }
    if ctx.per_edge_margins {
        ensure_template_supports_per_edge_margins(&root.join(format!("{name}.typ")))?;
    }
    match target {
        PdfTarget::Preview => {
            let path = typst::compile_preview(root, name, bleed)?;
            info!("PDF updated: {}", path.display());
            Ok(path)
        }
        PdfTarget::Final => {
            let path = typst::compile_final(root, name, bleed)?;
            info!("Final PDF generated: {}", path.display());
            Ok(path)
        }
    }
}

/// Templates from before the per-edge margins only know `margin_mm`. They would
/// place the photos with the wrong offset, so stop instead of rendering silently.
fn ensure_template_supports_per_edge_margins(template: &Path) -> Result<()> {
    let content = std::fs::read_to_string(template)
        .with_context(|| format!("Failed to read template: {}", template.display()))?;
    anyhow::ensure!(
        content.contains("margin_inside_mm"),
        "{} does not support margin_inside_mm, margin_outside_mm, margin_top_mm or \
         margin_bottom_mm. Update the .typ template from the current fotobuch version.",
        template.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::project::new::template::generate_template;

    #[test]
    fn current_template_supports_per_edge_margins() {
        let temp = tempfile::TempDir::new().unwrap();
        let path = temp.path().join("book.typ");
        std::fs::write(&path, generate_template("book").unwrap()).unwrap();
        assert!(ensure_template_supports_per_edge_margins(&path).is_ok());
    }

    #[test]
    fn old_template_is_rejected() {
        let temp = tempfile::TempDir::new().unwrap();
        let path = temp.path().join("book.typ");
        std::fs::write(&path, "#let margin = data.config.book.margin_mm * 1mm\n").unwrap();
        let err = ensure_template_supports_per_edge_margins(&path).unwrap_err();
        assert!(err.to_string().contains("margin_inside_mm"), "{err}");
    }
}
