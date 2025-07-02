//! bpmn_20_association_a_artifacts_sub_process

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_artifacts_subProcess",
//     name: "A_artifacts_subProcess",
//     visibility: Private,
//     member_end: (
//         "SubProcess-artifacts",
//         "A_artifacts_subProcess-subProcess",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_artifacts_subProcess-subProcess",
//                 name: "subProcess",
//                 visibility: Public,
//                 simple_type: Some(
//                     "SubProcess",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
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
//                 owning_association: "A_artifacts_subProcess",
//                 association: Some(
//                     "A_artifacts_subProcess",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

