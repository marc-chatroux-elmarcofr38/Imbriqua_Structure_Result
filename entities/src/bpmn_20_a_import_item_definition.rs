//! bpmn_20_association_a_import_item_definition

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_import_itemDefinition",
//     name: "A_import_itemDefinition",
//     visibility: Private,
//     member_end: (
//         "ItemDefinition-import",
//         "A_import_itemDefinition-itemDefinition",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_import_itemDefinition-itemDefinition",
//                 name: "itemDefinition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ItemDefinition",
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
//                 owning_association: "A_import_itemDefinition",
//                 association: Some(
//                     "A_import_itemDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

