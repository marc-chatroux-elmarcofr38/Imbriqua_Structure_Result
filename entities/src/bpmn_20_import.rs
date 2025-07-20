//! bpmn_20_class_import

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_import")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SIMPLE FIELD : BPMN20-Import-importType
    pub import_type: std::string::String,
    /// SIMPLE FIELD : BPMN20-Import-location
    pub location: std::string::String,
    /// SIMPLE FIELD : BPMN20-Import-namespace
    pub namespace: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Import" (bpmn_20_class_import)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __import_type__ (xmi_id : "BPMN20-Import-importType")
    ///   * type : __std::string::String__
    /// * __location__ (xmi_id : "BPMN20-Import-location")
    ///   * type : __std::string::String__
    /// * __namespace__ (xmi_id : "BPMN20-Import-namespace")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __Definitions__ (__DefinitionsModel__) from A_imports_definition
    ///   * one-to-many link : (1-1) __Import__ need (0-inf) __Definitions__)
    ///   * callable using find_with_related(__DefinitionsModel__) from __Import__
    ///   * named definition in BPMN
    /// 
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Import" (bpmn_20_class_import)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __import_type__ (xmi_id : "BPMN20-Import-importType")
  * type : __std::string::String__
* __location__ (xmi_id : "BPMN20-Import-location")
  * type : __std::string::String__
* __namespace__ (xmi_id : "BPMN20-Import-namespace")
  * type : __std::string::String__


## Relation : One To Many :
* __Definitions__ (__DefinitionsModel__) from A_imports_definition
  * one-to-many link : (1-1) __Import__ need (0-inf) __Definitions__)
  * callable using find_with_related(__DefinitionsModel__) from __Import__
  * named definition in BPMN



"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "Import",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "Import",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: {
//         "-Import-importType": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Import-importType",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "importType",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
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
//         "-Import-location": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Import-location",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "location",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
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
//         "-Import-namespace": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Import-namespace",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "namespace",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Import",
//     table_name: "bpmn_20_import",
//     model_name: "Import",
//     full_name: "bpmn_20_class_import",
// }

