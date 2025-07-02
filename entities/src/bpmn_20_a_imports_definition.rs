//! bpmn_20_association_a_imports_definition

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_imports_definition",
//     name: "A_imports_definition",
//     visibility: Private,
//     member_end: (
//         "Definitions-imports",
//         "A_imports_definition-definition",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_imports_definition-definition",
//                 name: "definition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Definitions",
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
//                 owning_association: "A_imports_definition",
//                 association: Some(
//                     "A_imports_definition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

