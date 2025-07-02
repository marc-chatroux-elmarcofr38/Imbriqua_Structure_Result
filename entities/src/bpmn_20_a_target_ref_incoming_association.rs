//! bpmn_20_association_a_target_ref_incoming_association

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_targetRef_incoming_association",
//     name: "A_targetRef_incoming_association",
//     visibility: Private,
//     member_end: (
//         "Association-targetRef",
//         "A_targetRef_incoming_association-incoming",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_targetRef_incoming_association-incoming",
//                 name: "incoming",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Association",
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
//                 owning_association: "A_targetRef_incoming_association",
//                 association: Some(
//                     "A_targetRef_incoming_association",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

