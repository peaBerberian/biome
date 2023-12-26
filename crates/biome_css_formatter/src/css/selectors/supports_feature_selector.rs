use crate::prelude::*;
use biome_css_syntax::CssSupportsFeatureSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsFeatureSelector;
impl FormatNodeRule<CssSupportsFeatureSelector> for FormatCssSupportsFeatureSelector {
    fn fmt_fields(
        &self,
        node: &CssSupportsFeatureSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
