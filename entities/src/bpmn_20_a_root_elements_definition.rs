//! bpmn_20_association_a_root_elements_definition

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_rootElements_definition",
//     name: "A_rootElements_definition",
//     visibility: Private,
//     member_end: (
//         "Definitions-rootElements",
//         "A_rootElements_definition-definition",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_rootElements_definition-definition",
//                 name: "definition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Definitions",
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
//                 owning_association: "A_rootElements_definition",
//                 association: Some(
//                     "A_rootElements_definition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

