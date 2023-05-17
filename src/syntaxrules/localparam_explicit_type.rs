use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, RuleResult};
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct LocalparamExplicitType;

impl SyntaxRule for LocalparamExplicitType {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
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
            RefNode::LocalParameterDeclarationParam(x) => {
                let t = unwrap_node!(*x, ImplicitDataType);
                if t.is_some() {
                    RuleResult::Fail
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("localparam_explicit_type")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Provide an explicit type in `localparam` declaration.")
    }

    fn reason(&self) -> String {
        String::from("Explicit parameter types clarify intent and improve readability.")
    }
}
