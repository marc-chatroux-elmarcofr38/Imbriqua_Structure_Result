//! InteractionNode
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of InteractionNode (Class : InteractionNode)
///
/// ```json
/// CMOFClass {
///     xmi_id: "InteractionNode",
///     name: "InteractionNode",
///     is_abstract: true,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "InteractionNode-incomingConversationLinks",
///                 name: "incomingConversationLinks",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ConversationLink",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: true,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_targetRef_incomingConversationLinks",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "InteractionNode-outgoingConversationLinks",
///                 name: "outgoingConversationLinks",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ConversationLink",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: true,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_sourceRef_outgoingConversationLinks",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Builder, Debug, Clone)]
pub struct InteractionNode {
    #[builder(setter(into, strip_option), default)]
    pub incoming_conversation_links: Option<Vec<ConversationLink>>,
    #[builder(setter(into, strip_option), default)]
    pub outgoing_conversation_links: Option<Vec<ConversationLink>>,
}

