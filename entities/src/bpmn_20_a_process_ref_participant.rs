//! bpmn_20_association_a_process_ref_participant

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_processRef_participant",
//     name: "A_processRef_participant",
//     visibility: Private,
//     member_end: (
//         "Participant-processRef",
//         "A_processRef_participant-participant",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_processRef_participant-participant",
//                 name: "participant",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Participant",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "A_processRef_participant",
//                 association: Some(
//                     "A_processRef_participant",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

