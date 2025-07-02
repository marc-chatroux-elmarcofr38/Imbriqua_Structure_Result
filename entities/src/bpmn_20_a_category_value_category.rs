//! bpmn_20_association_a_category_value_category

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_categoryValue_category",
//     name: "A_categoryValue_category",
//     visibility: Private,
//     member_end: (
//         "Category-categoryValue",
//         "A_categoryValue_category-category",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_categoryValue_category-category",
//                 name: "category",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Category",
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
//                 owning_association: "A_categoryValue_category",
//                 association: Some(
//                     "A_categoryValue_category",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

