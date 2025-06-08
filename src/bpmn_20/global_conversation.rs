//! global_conversation
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of GlobalConversation (Class : GlobalConversation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "GlobalConversation",
///     name: "GlobalConversation",
///     is_abstract: false,
///     super_class: Some(
///         "Collaboration",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct GlobalConversation<'a> {
}

