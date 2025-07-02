//! bpmn_20_association_a_resource_parameters_resource

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_resourceParameters_resource",
//     name: "A_resourceParameters_resource",
//     visibility: Private,
//     member_end: (
//         "Resource-resourceParameters",
//         "A_resourceParameters_resource-resource",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_resourceParameters_resource-resource",
//                 name: "resource",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Resource",
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
//                 owning_association: "A_resourceParameters_resource",
//                 association: Some(
//                     "A_resourceParameters_resource",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

