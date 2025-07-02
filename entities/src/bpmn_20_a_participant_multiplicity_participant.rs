//! bpmn_20_association_a_participant_multiplicity_participant

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_participantMultiplicity_participant",
//     name: "A_participantMultiplicity_participant",
//     visibility: Private,
//     member_end: (
//         "Participant-participantMultiplicity",
//         "A_participantMultiplicity_participant-participant",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_participantMultiplicity_participant-participant",
//                 name: "participant",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Participant",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "A_participantMultiplicity_participant",
//                 association: Some(
//                     "A_participantMultiplicity_participant",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

