//! bpmn_20_class_definitions

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_definitions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// SIMPLE FIELD : BPMN20-Definitions-exporter
    pub exporter: std::string::String,
    /// SIMPLE FIELD : BPMN20-Definitions-exporterVersion
    pub exporter_version: std::string::String,
    /// SIMPLE FIELD : BPMN20-Definitions-expressionLanguage
    #[sea_orm(default_value = "http://www.w3.org/1999/XPath")]
    pub expression_language: std::string::String,
    /// SIMPLE FIELD : BPMN20-Definitions-name
    pub name: std::string::String,
    /// SIMPLE FIELD : BPMN20-Definitions-targetNamespace
    pub target_namespace: std::string::String,
    /// SIMPLE FIELD : BPMN20-Definitions-typeLanguage
    #[sea_orm(default_value = "http://www.w3.org/2001/XMLSchema")]
    pub type_language: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Definitions need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE Definitions need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Definitions" (bpmn_20_class_definitions)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __exporter__ (xmi_id : "BPMN20-Definitions-exporter")
    ///   * type : __std::string::String__
    /// * __exporter_version__ (xmi_id : "BPMN20-Definitions-exporterVersion")
    ///   * type : __std::string::String__
    /// * __expression_language__ (xmi_id : "BPMN20-Definitions-expressionLanguage")
    ///   * type : __std::string::String__
    ///   * default : "http://www.w3.org/1999/XPath"
    /// * __name__ (xmi_id : "BPMN20-Definitions-name")
    ///   * type : __std::string::String__
    /// * __target_namespace__ (xmi_id : "BPMN20-Definitions-targetNamespace")
    ///   * type : __std::string::String__
    /// * __type_language__ (xmi_id : "BPMN20-Definitions-typeLanguage")
    ///   * type : __std::string::String__
    ///   * default : "http://www.w3.org/2001/XMLSchema"
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __Definitions__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Definitions__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Definitions" (bpmn_20_class_definitions)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __exporter__ (xmi_id : "BPMN20-Definitions-exporter")
  * type : __std::string::String__
* __exporter_version__ (xmi_id : "BPMN20-Definitions-exporterVersion")
  * type : __std::string::String__
* __expression_language__ (xmi_id : "BPMN20-Definitions-expressionLanguage")
  * type : __std::string::String__
  * default : "http://www.w3.org/1999/XPath"
* __name__ (xmi_id : "BPMN20-Definitions-name")
  * type : __std::string::String__
* __target_namespace__ (xmi_id : "BPMN20-Definitions-targetNamespace")
  * type : __std::string::String__
* __type_language__ (xmi_id : "BPMN20-Definitions-typeLanguage")
  * type : __std::string::String__
  * default : "http://www.w3.org/2001/XMLSchema"



## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __Definitions__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Definitions__
  * saved in __super_base_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "Definitions",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "Definitions",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-Definitions-diagrams": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Definitions-diagrams",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "diagrams",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefClass(
//                         HRefClass {
//                             href: "BPMNDI.cmof#BPMNDiagram",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
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
//                     "A_diagrams_definitions",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Definitions-exporter": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Definitions-exporter",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "exporter",
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
//         "-Definitions-exporterVersion": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Definitions-exporterVersion",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "exporterVersion",
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
//         "-Definitions-expressionLanguage": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Definitions-expressionLanguage",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "expressionLanguage",
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
//                 default: Some(
//                     "http://www.w3.org/1999/XPath",
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
//         "-Definitions-extensions": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Definitions-extensions",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "extensions",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Extension",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
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
//                     "A_extensions_definitions",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Definitions-imports": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Definitions-imports",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "imports",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Import",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
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
//                     "A_imports_definition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Definitions-name": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Definitions-name",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "name",
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
//         "-Definitions-relationships": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Definitions-relationships",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "relationships",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Relationship",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
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
//                     "A_relationships_definition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Definitions-rootElements": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Definitions-rootElements",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "rootElements",
//                 visibility: Public,
//                 simple_type: Some(
//                     "RootElement",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
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
//                     "A_rootElements_definition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Definitions-targetNamespace": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Definitions-targetNamespace",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "targetNamespace",
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
//         "-Definitions-typeLanguage": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Definitions-typeLanguage",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "typeLanguage",
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
//                 default: Some(
//                     "http://www.w3.org/2001/XMLSchema",
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
//     technical_name: "BPMN20.cmof#Definitions",
//     table_name: "bpmn_20_definitions",
//     model_name: "Definitions",
//     full_name: "bpmn_20_class_definitions",
// }

