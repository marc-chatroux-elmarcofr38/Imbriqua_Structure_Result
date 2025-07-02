//! bpmn_20_association_a_extension_attribute_definition_extension_attribute_value

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_extensionAttributeDefinition_extensionAttributeValue",
//     name: "A_extensionAttributeDefinition_extensionAttributeValue",
//     visibility: Private,
//     member_end: (
//         "ExtensionAttributeValue-extensionAttributeDefinition",
//         "A_extensionAttributeDefinition_extensionAttributeValue-extensionAttributeValue",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_extensionAttributeDefinition_extensionAttributeValue-extensionAttributeValue",
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
//                 owning_association: "A_extensionAttributeDefinition_extensionAttributeValue",
//                 association: Some(
//                     "A_extensionAttributeDefinition_extensionAttributeValue",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

