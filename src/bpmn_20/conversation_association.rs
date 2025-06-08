//! conversation_association

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of ConversationAssociation (Class : ConversationAssociation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ConversationAssociation",
///     name: "ConversationAssociation",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ConversationAssociation-innerConversationNodeRef",
///                 name: "innerConversationNodeRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ConversationNode",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_innerConversationNodeRef_conversationAssociation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ConversationAssociation-outerConversationNodeRef",
///                 name: "outerConversationNodeRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ConversationNode",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_outerConversationNodeRef_conversationAssociation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct ConversationAssociation<'a> {
    #[builder(setter(into))]
    pub inner_conversation_node_ref: &'a ConversationNode<'a>,
    #[builder(setter(into))]
    pub outer_conversation_node_ref: &'a ConversationNode<'a>,
}

