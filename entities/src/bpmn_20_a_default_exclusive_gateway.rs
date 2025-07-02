//! bpmn_20_association_a_default_exclusive_gateway

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_default_exclusiveGateway",
//     name: "A_default_exclusiveGateway",
//     visibility: Private,
//     member_end: (
//         "ExclusiveGateway-default",
//         "A_default_exclusiveGateway-exclusiveGateway",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_default_exclusiveGateway-exclusiveGateway",
//                 name: "exclusiveGateway",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ExclusiveGateway",
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
//                 owning_association: "A_default_exclusiveGateway",
//                 association: Some(
//                     "A_default_exclusiveGateway",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

