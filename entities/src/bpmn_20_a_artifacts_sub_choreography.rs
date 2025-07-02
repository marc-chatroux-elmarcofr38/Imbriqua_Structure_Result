//! bpmn_20_association_a_artifacts_sub_choreography

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_artifacts_subChoreography",
//     name: "A_artifacts_subChoreography",
//     visibility: Private,
//     member_end: (
//         "SubChoreography-artifacts",
//         "A_artifacts_subChoreography-subChoreography",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_artifacts_subChoreography-subChoreography",
//                 name: "subChoreography",
//                 visibility: Public,
//                 simple_type: Some(
//                     "SubChoreography",
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
//                 owning_association: "A_artifacts_subChoreography",
//                 association: Some(
//                     "A_artifacts_subChoreography",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

