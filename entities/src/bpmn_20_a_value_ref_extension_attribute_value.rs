//! bpmn_20_association_a_value_ref_extension_attribute_value

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_valueRef_extensionAttributeValue",
//     name: "A_valueRef_extensionAttributeValue",
//     visibility: Private,
//     member_end: (
//         "ExtensionAttributeValue-valueRef",
//         "A_valueRef_extensionAttributeValue-extensionAttributeValue",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_valueRef_extensionAttributeValue-extensionAttributeValue",
//                 name: "extensionAttributeValue",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ExtensionAttributeValue",
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
//                 owning_association: "A_valueRef_extensionAttributeValue",
//                 association: Some(
//                     "A_valueRef_extensionAttributeValue",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

