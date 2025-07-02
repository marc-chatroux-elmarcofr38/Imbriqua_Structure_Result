//! bpmn_20_association_a_item_subject_ref_item_aware_element

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_itemSubjectRef_itemAwareElement",
//     name: "A_itemSubjectRef_itemAwareElement",
//     visibility: Private,
//     member_end: (
//         "ItemAwareElement-itemSubjectRef",
//         "A_itemSubjectRef_itemAwareElement-itemAwareElement",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_itemSubjectRef_itemAwareElement-itemAwareElement",
//                 name: "itemAwareElement",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ItemAwareElement",
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
//                 owning_association: "A_itemSubjectRef_itemAwareElement",
//                 association: Some(
//                     "A_itemSubjectRef_itemAwareElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

