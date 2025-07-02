//! bpmn_20_association_a_assignment_data_association

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_assignment_dataAssociation",
//     name: "A_assignment_dataAssociation",
//     visibility: Private,
//     member_end: (
//         "DataAssociation-assignment",
//         "A_assignment_dataAssociation-dataAssociation",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_assignment_dataAssociation-dataAssociation",
//                 name: "dataAssociation",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataAssociation",
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
//                 owning_association: "A_assignment_dataAssociation",
//                 association: Some(
//                     "A_assignment_dataAssociation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

