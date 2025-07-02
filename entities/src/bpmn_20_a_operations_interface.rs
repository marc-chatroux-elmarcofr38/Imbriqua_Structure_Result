//! bpmn_20_association_a_operations_interface

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_operations_interface",
//     name: "A_operations_interface",
//     visibility: Private,
//     member_end: (
//         "Interface-operations",
//         "A_operations_interface-interface",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_operations_interface-interface",
//                 name: "interface",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Interface",
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
//                 owning_association: "A_operations_interface",
//                 association: Some(
//                     "A_operations_interface",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

