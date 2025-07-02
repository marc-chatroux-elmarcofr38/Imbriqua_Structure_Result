//! bpmn_20_association_a_default_inclusive_gateway

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_default_inclusiveGateway",
//     name: "A_default_inclusiveGateway",
//     visibility: Private,
//     member_end: (
//         "InclusiveGateway-default",
//         "A_default_inclusiveGateway-inclusiveGateway",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_default_inclusiveGateway-inclusiveGateway",
//                 name: "inclusiveGateway",
//                 visibility: Public,
//                 simple_type: Some(
//                     "InclusiveGateway",
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
//                 owning_association: "A_default_inclusiveGateway",
//                 association: Some(
//                     "A_default_inclusiveGateway",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

