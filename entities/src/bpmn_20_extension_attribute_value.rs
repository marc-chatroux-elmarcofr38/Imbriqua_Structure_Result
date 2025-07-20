//! bpmn_20_class_extension_attribute_value

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_extension_attribute_value")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// COMPLEX FIELD : BPMN20-ExtensionAttributeValue-extensionAttributeDefinition
    pub extension_attribute_definition: i64,
    /// COMPLEX FIELD : BPMN20-ExtensionAttributeValue-value
    pub value: Option<i64>,
    /// COMPLEX FIELD : BPMN20-ExtensionAttributeValue-valueRef
    pub value_ref: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ExtensionAttributeValue" (bpmn_20_class_extension_attribute_value)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// ## Direct One To One :
    /// * __Element__ (__ElementModel__) from A_value_extensionAttributeValue
    ///   * one-to-one link : (0-1) __ExtensionAttributeValue__ need (1-1) __Element__)
    ///   * callable using find_also_related(__ElementModel__) from __ExtensionAttributeValue__
    ///   * saved in __value__ field as foreing key
    /// 
    /// ## Relation : One To Many :
    /// * __ExtensionAttributeDefinition__ (__ExtensionAttributeDefinitionModel__) from A_extensionAttributeDefinition_extensionAttributeValue
    ///   * one-to-many link : (1-1) __ExtensionAttributeValue__ need (0-inf) __ExtensionAttributeDefinition__)
    ///   * callable using find_with_related(__ExtensionAttributeDefinitionModel__) from __ExtensionAttributeValue__
    /// * __BaseElement__ (__BaseElementModel__) from A_extensionValues_baseElement
    ///   * one-to-many link : (1-1) __ExtensionAttributeValue__ need (0-inf) __BaseElement__)
    ///   * callable using find_with_related(__BaseElementModel__) from __ExtensionAttributeValue__
    ///   * named base_element in BPMN
    /// * __Element__ (__ElementModel__) from A_valueRef_extensionAttributeValue
    ///   * one-to-many link : (0-1) __ExtensionAttributeValue__ need (0-inf) __Element__)
    ///   * callable using find_with_related(__ElementModel__) from __ExtensionAttributeValue__
    /// 
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ExtensionAttributeValue" (bpmn_20_class_extension_attribute_value)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__


## Direct One To One :
* __Element__ (__ElementModel__) from A_value_extensionAttributeValue
  * one-to-one link : (0-1) __ExtensionAttributeValue__ need (1-1) __Element__)
  * callable using find_also_related(__ElementModel__) from __ExtensionAttributeValue__
  * saved in __value__ field as foreing key

## Relation : One To Many :
* __ExtensionAttributeDefinition__ (__ExtensionAttributeDefinitionModel__) from A_extensionAttributeDefinition_extensionAttributeValue
  * one-to-many link : (1-1) __ExtensionAttributeValue__ need (0-inf) __ExtensionAttributeDefinition__)
  * callable using find_with_related(__ExtensionAttributeDefinitionModel__) from __ExtensionAttributeValue__
* __BaseElement__ (__BaseElementModel__) from A_extensionValues_baseElement
  * one-to-many link : (1-1) __ExtensionAttributeValue__ need (0-inf) __BaseElement__)
  * callable using find_with_related(__BaseElementModel__) from __ExtensionAttributeValue__
  * named base_element in BPMN
* __Element__ (__ElementModel__) from A_valueRef_extensionAttributeValue
  * one-to-many link : (0-1) __ExtensionAttributeValue__ need (0-inf) __Element__)
  * callable using find_with_related(__ElementModel__) from __ExtensionAttributeValue__



"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Weak ref of "BPMN20-ExtensionAttributeValue" (loaded : false)",
//     name: "ExtensionAttributeValue",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: {
//         "-ExtensionAttributeValue-extensionAttributeDefinition": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-ExtensionAttributeValue-extensionAttributeDefinition" (loaded : false)",
//                 name: "extensionAttributeDefinition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ExtensionAttributeDefinition",
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
//                     "A_extensionAttributeDefinition_extensionAttributeValue",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-ExtensionAttributeValue-value": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-ExtensionAttributeValue-value" (loaded : false)",
//                 name: "value",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefClass(
//                         HRefClass {
//                             href: "Weak ref of "Extensibilty-Element" (loaded : false)",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_value_extensionAttributeValue",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-ExtensionAttributeValue-valueRef": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-ExtensionAttributeValue-valueRef" (loaded : false)",
//                 name: "valueRef",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefClass(
//                         HRefClass {
//                             href: "Weak ref of "Extensibilty-Element" (loaded : false)",
//                         },
//                     ),
//                 ),
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
//                 owning_association: "",
//                 association: Some(
//                     "A_valueRef_extensionAttributeValue",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#ExtensionAttributeValue",
//     table_name: "bpmn_20_extension_attribute_value",
//     model_name: "ExtensionAttributeValue",
//     full_name: "bpmn_20_class_extension_attribute_value",
// }

