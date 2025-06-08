//! node
#[allow(unused)]
#[allow(unused_imports)]

use crate::di::*;
use crate::Builder;

/// Conversion of Node (Class : Node)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Node",
///     name: "Node",
///     is_abstract: true,
///     super_class: Some(
///         "DiagramElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct Node<'a> {
}

