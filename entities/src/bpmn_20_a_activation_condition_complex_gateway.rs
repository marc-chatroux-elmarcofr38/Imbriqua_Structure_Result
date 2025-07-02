//! bpmn_20_association_a_activation_condition_complex_gateway

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_activationCondition_complexGateway",
//     name: "A_activationCondition_complexGateway",
//     visibility: Private,
//     member_end: (
//         "ComplexGateway-activationCondition",
//         "A_activationCondition_complexGateway-complexGateway",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_activationCondition_complexGateway-complexGateway",
//                 name: "complexGateway",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ComplexGateway",
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
//                 owning_association: "A_activationCondition_complexGateway",
//                 association: Some(
//                     "A_activationCondition_complexGateway",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

