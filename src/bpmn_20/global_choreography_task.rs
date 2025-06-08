//! global_choreography_task
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of GlobalChoreographyTask (Class : GlobalChoreographyTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "GlobalChoreographyTask",
///     name: "GlobalChoreographyTask",
///     is_abstract: false,
///     super_class: Some(
///         "Choreography",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "GlobalChoreographyTask-initiatingParticipantRef",
///                 name: "initiatingParticipantRef",
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
///                 association: "A_initiatingParticipantRef_globalChoreographyTask",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct GlobalChoreographyTask<'a> {
    #[builder(setter(into))]
    pub initiating_participant_ref: &'a Participant<'a>,
}

