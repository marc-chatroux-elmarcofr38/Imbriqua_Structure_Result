//! bpmn_20_association_a_partner_entity_ref_participant_ref

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_partnerEntityRef_participantRef",
//     name: "A_partnerEntityRef_participantRef",
//     visibility: Private,
//     member_end: (
//         "A_partnerEntityRef_participantRef-partnerEntityRef",
//         "PartnerEntity-participantRef",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_partnerEntityRef_participantRef-partnerEntityRef",
//                 name: "partnerEntityRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "PartnerEntity",
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
//                 is_derived: true,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "A_partnerEntityRef_participantRef",
//                 association: Some(
//                     "A_partnerEntityRef_participantRef",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

