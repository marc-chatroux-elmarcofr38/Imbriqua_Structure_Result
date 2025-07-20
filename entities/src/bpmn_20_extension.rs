//! bpmn_20_class_extension

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_extension")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// COMPLEX FIELD : BPMN20-Extension-definition
    pub definition: i64,
    /// SIMPLE FIELD : BPMN20-Extension-mustUnderstand
    #[sea_orm(default_value = "false")]
    pub must_understand: std::primitive::bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Extension" (bpmn_20_class_extension)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __must_understand__ (xmi_id : "BPMN20-Extension-mustUnderstand")
    ///   * type : __std::primitive::bool__
    ///   * default : "false"
    /// 
    /// ## Direct One To One :
    /// * __ExtensionDefinition__ (__ExtensionDefinitionModel__) from A_definition_extension
    ///   * one-to-one link : (1-1) __Extension__ need (1-1) __ExtensionDefinition__)
    ///   * callable using find_also_related(__ExtensionDefinitionModel__) from __Extension__
    ///   * saved in __definition__ field as foreing key
    /// 
    /// ## Relation : One To Many :
    /// * __Definitions__ (__DefinitionsModel__) from A_extensions_definitions
    ///   * one-to-many link : (1-1) __Extension__ need (0-inf) __Definitions__)
    ///   * callable using find_with_related(__DefinitionsModel__) from __Extension__
    ///   * named definitions in BPMN
    /// 
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Extension" (bpmn_20_class_extension)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __must_understand__ (xmi_id : "BPMN20-Extension-mustUnderstand")
  * type : __std::primitive::bool__
  * default : "false"

## Direct One To One :
* __ExtensionDefinition__ (__ExtensionDefinitionModel__) from A_definition_extension
  * one-to-one link : (1-1) __Extension__ need (1-1) __ExtensionDefinition__)
  * callable using find_also_related(__ExtensionDefinitionModel__) from __Extension__
  * saved in __definition__ field as foreing key

## Relation : One To Many :
* __Definitions__ (__DefinitionsModel__) from A_extensions_definitions
  * one-to-many link : (1-1) __Extension__ need (0-inf) __Definitions__)
  * callable using find_with_related(__DefinitionsModel__) from __Extension__
  * named definitions in BPMN



"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         object_id: "Extension",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "Extension",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: {
//         "-Extension-definition": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Extension-definition",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "definition",
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
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_definition_extension",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Extension-mustUnderstand": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Extension-mustUnderstand",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "mustUnderstand",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Extension",
//     table_name: "bpmn_20_extension",
//     model_name: "Extension",
//     full_name: "bpmn_20_class_extension",
// }

