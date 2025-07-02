//! bpmn_20_association_a_io_specification_callable_element

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_ioSpecification_callableElement",
//     name: "A_ioSpecification_callableElement",
//     visibility: Private,
//     member_end: (
//         "CallableElement-ioSpecification",
//         "A_ioSpecification_callableElement-callableElement",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_ioSpecification_callableElement-callableElement",
//                 name: "callableElement",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CallableElement",
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
//                 owning_association: "A_ioSpecification_callableElement",
//                 association: Some(
//                     "A_ioSpecification_callableElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

