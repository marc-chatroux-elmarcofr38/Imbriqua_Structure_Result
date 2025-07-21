//! di_class_diagram

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_diagram")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// COMPLEX FIELD : DI-Diagram-rootElement
    pub root_element: i64,
    /// SIMPLE FIELD : DI-Diagram-documentation
    pub documentation: Option<std::string::String>,
    /// SIMPLE FIELD : DI-Diagram-name
    pub name: Option<std::string::String>,
    /// SIMPLE FIELD : DI-Diagram-resolution
    pub resolution: Option<std::primitive::f64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE BpmnDiagram need ONE Diagram
    #[sea_orm(has_one = "super::bpmndi_bpmn_diagram::Entity")]
    BpmnDiagram,
}

// SUPER : ONE BpmnDiagram need ONE Diagram
impl Related<super::bpmndi_bpmn_diagram::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BpmnDiagram.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Diagram" (di_class_diagram)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __documentation__ (xmi_id : "DI-Diagram-documentation")
    ///   * type : __Option<std::string::String>__
    /// * __name__ (xmi_id : "DI-Diagram-name")
    ///   * type : __Option<std::string::String>__
    /// * __resolution__ (xmi_id : "DI-Diagram-resolution")
    ///   * type : __Option<std::primitive::f64>__
    /// 
    /// 
    /// 
    /// ## Reverse One To One :
    /// * __DiagramElement__ (__DiagramElementModel__) from A_rootElement_owningDiagram
    ///   * one-to-one link : (0-1) __DiagramElement__ need (1-1) __Diagram__)
    ///   * callable using find_also_related(__DiagramModel__) from __DiagramElement__
    ///   * saved in __owning_diagram__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __BpmnDiagram__ (__BpmnDiagramModel__)
    ///   * one-to-one link (reverse) : one __BpmnDiagram__ need one __Diagram__)
    ///   * callable using find_also_related(__DiagramModel__) from __BpmnDiagram__
    ///   * saved in __super_diagram__ field as foreing key in __BpmnDiagramModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Diagram" (di_class_diagram)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __documentation__ (xmi_id : "DI-Diagram-documentation")
  * type : __Option<std::string::String>__
* __name__ (xmi_id : "DI-Diagram-name")
  * type : __Option<std::string::String>__
* __resolution__ (xmi_id : "DI-Diagram-resolution")
  * type : __Option<std::primitive::f64>__



## Reverse One To One :
* __DiagramElement__ (__DiagramElementModel__) from A_rootElement_owningDiagram
  * one-to-one link : (0-1) __DiagramElement__ need (1-1) __Diagram__)
  * callable using find_also_related(__DiagramModel__) from __DiagramElement__
  * saved in __owning_diagram__ field as foreing key

## Reverse Super :
* __BpmnDiagram__ (__BpmnDiagramModel__)
  * one-to-one link (reverse) : one __BpmnDiagram__ need one __Diagram__)
  * callable using find_also_related(__DiagramModel__) from __BpmnDiagram__
  * saved in __super_diagram__ field as foreing key in __BpmnDiagramModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdLocalReference {
//         object_id: "Diagram",
//         package_id: "DI",
//         is_set: true,
//     },
//     name: "Diagram",
//     is_abstract: true,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: {
//         "Diagram-documentation": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "Diagram-documentation",
//                     package_id: "DI",
//                     is_set: true,
//                 },
//                 name: "documentation",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "RefCell of 'DC-String' (loaded : true)",
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "Diagram-name": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "Diagram-name",
//                     package_id: "DI",
//                     is_set: true,
//                 },
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "RefCell of 'DC-String' (loaded : true)",
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "Diagram-ownedStyle": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "Diagram-ownedStyle",
//                     package_id: "DI",
//                     is_set: true,
//                 },
//                 name: "ownedStyle",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Style",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: true,
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: true,
//                 is_derived_union: true,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_ownedStyle_owningDiagram",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "Diagram-resolution": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "Diagram-resolution",
//                     package_id: "DI",
//                     is_set: true,
//                 },
//                 name: "resolution",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "RefCell of 'DC-Real' (loaded : true)",
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "Diagram-rootElement": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "Diagram-rootElement",
//                     package_id: "DI",
//                     is_set: true,
//                 },
//                 name: "rootElement",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DiagramElement",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: true,
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: true,
//                 is_derived_union: true,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_rootElement_owningDiagram",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "DI.cmof#Diagram",
//     table_name: "di_diagram",
//     model_name: "Diagram",
//     full_name: "di_class_diagram",
// }

