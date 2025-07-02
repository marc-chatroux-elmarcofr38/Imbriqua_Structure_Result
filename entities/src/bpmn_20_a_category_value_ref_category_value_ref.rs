//! bpmn_20_association_a_category_value_ref_category_value_ref

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_categoryValueRef_categoryValueRef",
//     name: "A_categoryValueRef_categoryValueRef",
//     visibility: Private,
//     member_end: (
//         "Group-categoryValueRef",
//         "A_categoryValueRef_categoryValueRef-categoryValueRef",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_categoryValueRef_categoryValueRef-categoryValueRef",
//                 name: "categoryValueRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Group",
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
//                 owning_association: "A_categoryValueRef_categoryValueRef",
//                 association: Some(
//                     "A_categoryValueRef_categoryValueRef",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

