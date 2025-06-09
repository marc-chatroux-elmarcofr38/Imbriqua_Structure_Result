//! SubConversation
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of SubConversation (Class : SubConversation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "SubConversation",
///     name: "SubConversation",
///     is_abstract: false,
///     super_class: Some(
///         "ConversationNode",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "SubConversation-conversationNodes",
///                 name: "conversationNodes",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ConversationNode",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: true,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_conversationNodes_subConversation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Builder, Debug, Clone)]
pub struct SubConversation {
    #[builder(setter(into, strip_option), default)]
    pub conversation_nodes: Option<Vec<ConversationNode>>,
}

