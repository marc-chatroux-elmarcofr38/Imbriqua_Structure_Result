//! bpmn_20_association_a_documentation_base_element

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_documentation_baseElement",
//     name: "A_documentation_baseElement",
//     visibility: Private,
//     member_end: (
//         "BaseElement-documentation",
//         "A_documentation_baseElement-baseElement",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_documentation_baseElement-baseElement",
//                 name: "baseElement",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BaseElement",
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
//                 owning_association: "A_documentation_baseElement",
//                 association: Some(
//                     "A_documentation_baseElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

