use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, RuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct StyleCommaleading;

impl SyntaxRule for StyleCommaleading {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };
        match node {
            RefNode::Symbol(x) => {
                let t = syntax_tree.get_str(*x).unwrap();
                if t.starts_with(",") && t != ", " {
                    RuleResult::Fail
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("style_commaleading")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from(format!(
            "Follow each comma with a single space (comma-leading format)."
        ))
    }

    fn reason(&self) -> String {
        String::from("Consistent style enhances readability.")
    }
}
