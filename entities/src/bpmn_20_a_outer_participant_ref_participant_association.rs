//! bpmn_20_association_a_outer_participant_ref_participant_association

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_outerParticipantRef_participantAssociation",
//     name: "A_outerParticipantRef_participantAssociation",
//     visibility: Private,
//     member_end: (
//         "ParticipantAssociation-outerParticipantRef",
//         "A_outerParticipantRef_participantAssociation-participantAssociation",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_outerParticipantRef_participantAssociation-participantAssociation",
//                 name: "participantAssociation",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ParticipantAssociation",
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
//                 owning_association: "A_outerParticipantRef_participantAssociation",
//                 association: Some(
//                     "A_outerParticipantRef_participantAssociation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

