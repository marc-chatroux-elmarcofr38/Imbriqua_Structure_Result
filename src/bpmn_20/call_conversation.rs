//! call_conversation
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of CallConversation (Class : CallConversation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CallConversation",
///     name: "CallConversation",
///     is_abstract: false,
///     super_class: Some(
///         "ConversationNode",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CallConversation-calledCollaborationRef",
///                 name: "calledCollaborationRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Collaboration",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
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
///                 association: "A_calledCollaborationRef_callConversation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CallConversation-participantAssociations",
///                 name: "participantAssociations",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ParticipantAssociation",
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
///                 association: "A_participantAssociations_callConversation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct CallConversation<'a> {
    #[builder(setter(into, strip_option), default)]
    pub called_collaboration_ref: Option<&'a Collaboration<'a>>,
    #[builder(setter(into, strip_option), default)]
    pub participant_associations: Option<Vec<&'a ParticipantAssociation<'a>>>,
}

