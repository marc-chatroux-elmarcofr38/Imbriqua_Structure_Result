//! ParticipantAssociation
#![allow(unused_imports)]

/// Conversion of ParticipantAssociation (Class : ParticipantAssociation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ParticipantAssociation",
///     name: "ParticipantAssociation",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ParticipantAssociation-innerParticipantRef",
///                 name: "innerParticipantRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Participant",
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
///                 association: "A_innerParticipantRef_participantAssociation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ParticipantAssociation-outerParticipantRef",
///                 name: "outerParticipantRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Participant",
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
///                 association: "A_outerParticipantRef_participantAssociation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Debug, Clone)]
pub struct ParticipantAssociation {
    pub inner_participant_ref: Participant,
    pub outer_participant_ref: Participant,
}

