//! bpmn_20_class_extension_attribute_definition

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_extension_attribute_definition")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// COMPLEX FIELD : BPMN20-ExtensionAttributeDefinition-extensionDefinition
    pub extension_definition: i64,
    /// SIMPLE FIELD : BPMN20-ExtensionAttributeDefinition-isReference
    #[sea_orm(default_value = "false")]
    pub is_reference: std::primitive::bool,
    /// SIMPLE FIELD : BPMN20-ExtensionAttributeDefinition-name
    pub name: std::string::String,
    /// SIMPLE FIELD : BPMN20-ExtensionAttributeDefinition-type
    pub r#type: std::string::String,
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
    /// * __is_reference__ (xmi_id : "BPMN20-ExtensionAttributeDefinition-isReference")
    ///   * type : __std::primitive::bool__
    ///   * default : "false"
    /// * __name__ (xmi_id : "BPMN20-ExtensionAttributeDefinition-name")
    ///   * type : __std::string::String__
    /// * __r#type__ (xmi_id : "BPMN20-ExtensionAttributeDefinition-type")
    ///   * type : __std::string::String__
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
* __is_reference__ (xmi_id : "BPMN20-ExtensionAttributeDefinition-isReference")
  * type : __std::primitive::bool__
  * default : "false"
* __name__ (xmi_id : "BPMN20-ExtensionAttributeDefinition-name")
  * type : __std::string::String__
* __r#type__ (xmi_id : "BPMN20-ExtensionAttributeDefinition-type")
  * type : __std::string::String__


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
//     xmi_id: XMIIdLocalReference {
//         object_id: "ExtensionAttributeDefinition",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "ExtensionAttributeDefinition",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: {
//         "ExtensionAttributeDefinition-extensionDefinition": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "ExtensionAttributeDefinition-extensionDefinition",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
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
//         "ExtensionAttributeDefinition-isReference": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "ExtensionAttributeDefinition-isReference",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "isReference",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "Weak ref of 'DC-Boolean' (loaded : true)",
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
//         "ExtensionAttributeDefinition-name": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "ExtensionAttributeDefinition-name",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "Weak ref of 'DC-String' (loaded : true)",
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
//         "ExtensionAttributeDefinition-type": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "ExtensionAttributeDefinition-type",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "r#type",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "Weak ref of 'DC-String' (loaded : true)",
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#ExtensionAttributeDefinition",
//     table_name: "bpmn_20_extension_attribute_definition",
//     model_name: "ExtensionAttributeDefinition",
//     full_name: "bpmn_20_class_extension_attribute_definition",
// }

