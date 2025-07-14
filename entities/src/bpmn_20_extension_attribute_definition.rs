//! bpmn_20_class_extension_attribute_definition

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_extension_attribute_definition")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// COMPLEX FIELD : ExtensionAttributeDefinition-extensionDefinition
    pub extension_definition: i64,
    /// SIMPLE FIELD : ExtensionAttributeDefinition-name
    pub name: std::string::String,
    /// SIMPLE FIELD : ExtensionAttributeDefinition-type
    pub r#type: std::string::String,
    /// SIMPLE FIELD : ExtensionAttributeDefinition-isReference
    #[sea_orm(default_value = "false")]
    pub is_reference: std::primitive::bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ExtensionAttributeDefinition" (bpmn_20_class_extension_attribute_definition)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "ExtensionAttributeDefinition-name")
    ///   * type : __std::string::String__
    /// * __r#type__ (xmi_id : "ExtensionAttributeDefinition-type")
    ///   * type : __std::string::String__
    /// * __is_reference__ (xmi_id : "ExtensionAttributeDefinition-isReference")
    ///   * type : __std::primitive::bool__
    ///   * default : "false"
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __ExtensionDefinition__ (__ExtensionDefinitionModel__) from A_extensionAttributeDefinitions_extensionDefinition
    ///   * one-to-many link : (1-1) __ExtensionAttributeDefinition__ need (0-inf) __ExtensionDefinition__)
    ///   * callable using find_with_related(__ExtensionDefinitionModel__) from __ExtensionAttributeDefinition__
    ///   * named extension_definition in BPMN
    /// 
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ExtensionAttributeDefinition" (bpmn_20_class_extension_attribute_definition)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "ExtensionAttributeDefinition-name")
  * type : __std::string::String__
* __r#type__ (xmi_id : "ExtensionAttributeDefinition-type")
  * type : __std::string::String__
* __is_reference__ (xmi_id : "ExtensionAttributeDefinition-isReference")
  * type : __std::primitive::bool__
  * default : "false"


## Relation : One To Many :
* __ExtensionDefinition__ (__ExtensionDefinitionModel__) from A_extensionAttributeDefinitions_extensionDefinition
  * one-to-many link : (1-1) __ExtensionAttributeDefinition__ need (0-inf) __ExtensionDefinition__)
  * callable using find_with_related(__ExtensionDefinitionModel__) from __ExtensionAttributeDefinition__
  * named extension_definition in BPMN



"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "ExtensionAttributeDefinition",
//     name: "ExtensionAttributeDefinition",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "ExtensionAttributeDefinition-name",
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#String",
//                         },
//                     ),
//                 ),
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
//                 owning_association: "",
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ExtensionAttributeDefinition-type",
//                 name: "r#type",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#String",
//                         },
//                     ),
//                 ),
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
//                 owning_association: "",
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ExtensionAttributeDefinition-isReference",
//                 name: "isReference",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#Boolean",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "false",
//                 ),
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ExtensionAttributeDefinition-extensionDefinition",
//                 name: "extensionDefinition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ExtensionDefinition",
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
//                 owning_association: "",
//                 association: Some(
//                     "A_extensionAttributeDefinitions_extensionDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

