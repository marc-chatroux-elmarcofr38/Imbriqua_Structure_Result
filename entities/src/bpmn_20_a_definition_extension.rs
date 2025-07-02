//! bpmn_20_association_a_definition_extension

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_definition_extension",
//     name: "A_definition_extension",
//     visibility: Private,
//     member_end: (
//         "Extension-definition",
//         "A_definition_extension-extension",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_definition_extension-extension",
//                 name: "extension",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Extension",
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
//                 owning_association: "A_definition_extension",
//                 association: Some(
//                     "A_definition_extension",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

