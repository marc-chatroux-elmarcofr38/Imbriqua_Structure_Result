//! bpmn_20_association_a_structure_ref_escalation

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_structureRef_escalation",
//     name: "A_structureRef_escalation",
//     visibility: Private,
//     member_end: (
//         "Escalation-structureRef",
//         "A_structureRef_escalation-escalation",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_structureRef_escalation-escalation",
//                 name: "escalation",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Escalation",
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
//                 owning_association: "A_structureRef_escalation",
//                 association: Some(
//                     "A_structureRef_escalation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

