//! di_class_diagram_element

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_diagram_element")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// COMPLEX FIELD : DiagramElement-modelElement
    pub model_element: Option<i64>,
    /// COMPLEX FIELD : DiagramElement-owningDiagram
    pub owning_diagram: Option<i64>,
    /// COMPLEX FIELD : DiagramElement-owningElement
    pub owning_element: Option<i64>,
    /// COMPLEX FIELD : DiagramElement-style
    pub style: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Edge need ONE DiagramElement
    #[sea_orm(has_one = "super::di_edge::Entity")]
    Edge,
    // SUPER : ONE Node need ONE DiagramElement
    #[sea_orm(has_one = "super::di_node::Entity")]
    Node,
}

// SUPER : ONE Edge need ONE DiagramElement
impl Related<super::di_edge::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Edge.def()
    }
}

// SUPER : ONE Node need ONE DiagramElement
impl Related<super::di_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Node.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "DiagramElement" (di_class_diagram_element)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// ## Direct One To One :
    /// * __Diagram__ (__DiagramModel__) from A_rootElement_owningDiagram
    ///   * one-to-one link : (0-1) __DiagramElement__ need (1-1) __Diagram__)
    ///   * callable using find_also_related(__DiagramModel__) from __DiagramElement__
    ///   * saved in __owning_diagram__ field as foreing key
    /// 
    /// ## Relation : One To Many :
    /// * __Element__ (__ElementModel__) from A_modelElement_diagramElement
    ///   * one-to-many link : (0-1) __DiagramElement__ need (0-inf) __Element__)
    ///   * callable using find_with_related(__ElementModel__) from __DiagramElement__
    /// * __DiagramElement__ (__DiagramElementModel__) from A_ownedElement_owningElement
    ///   * one-to-many link : (0-1) __DiagramElement__ need (0-inf) __DiagramElement__)
    ///   * callable using find_with_related(__DiagramElementModel__) from __DiagramElement__
    ///   * named owning_element in BPMN
    /// * __Plane__ (__PlaneModel__) from A_planeElement_plane
    ///   * one-to-many link : (0-1) __DiagramElement__ need (0-inf) __Plane__)
    ///   * callable using find_with_related(__PlaneModel__) from __DiagramElement__
    ///   * named plane in BPMN
    /// * __Style__ (__StyleModel__) from A_style_diagramElement
    ///   * one-to-many link : (0-1) __DiagramElement__ need (0-inf) __Style__)
    ///   * callable using find_with_related(__StyleModel__) from __DiagramElement__
    /// 
    /// 
    /// ## Reverse Super :
    /// * __Edge__ (__EdgeModel__)
    ///   * one-to-one link (reverse) : one __Edge__ need one __DiagramElement__)
    ///   * callable using find_also_related(__DiagramElementModel__) from __Edge__
    ///   * saved in __super_diagram_element__ field as foreing key in __EdgeModel__
    /// * __Node__ (__NodeModel__)
    ///   * one-to-one link (reverse) : one __Node__ need one __DiagramElement__)
    ///   * callable using find_also_related(__DiagramElementModel__) from __Node__
    ///   * saved in __super_diagram_element__ field as foreing key in __NodeModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "DiagramElement" (di_class_diagram_element)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__


## Direct One To One :
* __Diagram__ (__DiagramModel__) from A_rootElement_owningDiagram
  * one-to-one link : (0-1) __DiagramElement__ need (1-1) __Diagram__)
  * callable using find_also_related(__DiagramModel__) from __DiagramElement__
  * saved in __owning_diagram__ field as foreing key

## Relation : One To Many :
* __Element__ (__ElementModel__) from A_modelElement_diagramElement
  * one-to-many link : (0-1) __DiagramElement__ need (0-inf) __Element__)
  * callable using find_with_related(__ElementModel__) from __DiagramElement__
* __DiagramElement__ (__DiagramElementModel__) from A_ownedElement_owningElement
  * one-to-many link : (0-1) __DiagramElement__ need (0-inf) __DiagramElement__)
  * callable using find_with_related(__DiagramElementModel__) from __DiagramElement__
  * named owning_element in BPMN
* __Plane__ (__PlaneModel__) from A_planeElement_plane
  * one-to-many link : (0-1) __DiagramElement__ need (0-inf) __Plane__)
  * callable using find_with_related(__PlaneModel__) from __DiagramElement__
  * named plane in BPMN
* __Style__ (__StyleModel__) from A_style_diagramElement
  * one-to-many link : (0-1) __DiagramElement__ need (0-inf) __Style__)
  * callable using find_with_related(__StyleModel__) from __DiagramElement__


## Reverse Super :
* __Edge__ (__EdgeModel__)
  * one-to-one link (reverse) : one __Edge__ need one __DiagramElement__)
  * callable using find_also_related(__DiagramElementModel__) from __Edge__
  * saved in __super_diagram_element__ field as foreing key in __EdgeModel__
* __Node__ (__NodeModel__)
  * one-to-one link (reverse) : one __Node__ need one __DiagramElement__)
  * callable using find_also_related(__DiagramElementModel__) from __Node__
  * saved in __super_diagram_element__ field as foreing key in __NodeModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "DiagramElement",
//     name: "DiagramElement",
//     is_abstract: true,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: {
//         "DiagramElement-modelElement": Property(
//             CMOFProperty {
//                 xmi_id: "DiagramElement-modelElement",
//                 name: "modelElement",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     ClassLink(
//                         ClassLink {
//                             href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: true,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: true,
//                 is_derived_union: true,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_modelElement_diagramElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "DiagramElement-ownedElement": Property(
//             CMOFProperty {
//                 xmi_id: "DiagramElement-ownedElement",
//                 name: "ownedElement",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DiagramElement",
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
//                     "A_ownedElement_owningElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "DiagramElement-owningDiagram": Property(
//             CMOFProperty {
//                 xmi_id: "DiagramElement-owningDiagram",
//                 name: "owningDiagram",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Diagram",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: true,
//                 is_composite: false,
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
//         "DiagramElement-owningElement": Property(
//             CMOFProperty {
//                 xmi_id: "DiagramElement-owningElement",
//                 name: "owningElement",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DiagramElement",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: true,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: true,
//                 is_derived_union: true,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_ownedElement_owningElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "DiagramElement-style": Property(
//             CMOFProperty {
//                 xmi_id: "DiagramElement-style",
//                 name: "style",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Style",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: true,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: true,
//                 is_derived_union: true,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_style_diagramElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
// }

