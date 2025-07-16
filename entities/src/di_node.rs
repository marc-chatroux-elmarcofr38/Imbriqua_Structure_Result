//! di_class_node

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_node")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : DiagramElement
    pub super_diagram_element: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Node need ONE DiagramElement
    #[sea_orm(
        belongs_to = "super::di_diagram_element::Entity",
        from = "Column::SuperDiagramElement",
        to = "super::di_diagram_element::Column::Id",
        on_delete = "Cascade"
    )]
    DiagramElement,
    // SUPER : ONE Label need ONE Node
    #[sea_orm(has_one = "super::di_label::Entity")]
    Label,
    // SUPER : ONE Plane need ONE Node
    #[sea_orm(has_one = "super::di_plane::Entity")]
    Plane,
    // SUPER : ONE Shape need ONE Node
    #[sea_orm(has_one = "super::di_shape::Entity")]
    Shape,
}

// SUPER : ONE Node need ONE DiagramElement
impl Related<super::di_diagram_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DiagramElement.def()
    }
}

// SUPER : ONE Label need ONE Node
impl Related<super::di_label::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Label.def()
    }
}

// SUPER : ONE Plane need ONE Node
impl Related<super::di_plane::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Plane.def()
    }
}

// SUPER : ONE Shape need ONE Node
impl Related<super::di_shape::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Shape.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Node" (di_class_node)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __DiagramElement__ (__DiagramElementModel__)
    ///   * one-to-one link : one __Node__ need one __DiagramElement__)
    ///   * callable using find_also_related(__DiagramElementModel__) from __Node__
    ///   * saved in __super_diagram_element__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __Label__ (__LabelModel__)
    ///   * one-to-one link (reverse) : one __Label__ need one __Node__)
    ///   * callable using find_also_related(__NodeModel__) from __Label__
    ///   * saved in __super_node__ field as foreing key in __LabelModel__
    /// * __Plane__ (__PlaneModel__)
    ///   * one-to-one link (reverse) : one __Plane__ need one __Node__)
    ///   * callable using find_also_related(__NodeModel__) from __Plane__
    ///   * saved in __super_node__ field as foreing key in __PlaneModel__
    /// * __Shape__ (__ShapeModel__)
    ///   * one-to-one link (reverse) : one __Shape__ need one __Node__)
    ///   * callable using find_also_related(__NodeModel__) from __Shape__
    ///   * saved in __super_node__ field as foreing key in __ShapeModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Node" (di_class_node)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __DiagramElement__ (__DiagramElementModel__)
  * one-to-one link : one __Node__ need one __DiagramElement__)
  * callable using find_also_related(__DiagramElementModel__) from __Node__
  * saved in __super_diagram_element__ field as foreing key

## Reverse Super :
* __Label__ (__LabelModel__)
  * one-to-one link (reverse) : one __Label__ need one __Node__)
  * callable using find_also_related(__NodeModel__) from __Label__
  * saved in __super_node__ field as foreing key in __LabelModel__
* __Plane__ (__PlaneModel__)
  * one-to-one link (reverse) : one __Plane__ need one __Node__)
  * callable using find_also_related(__NodeModel__) from __Plane__
  * saved in __super_node__ field as foreing key in __PlaneModel__
* __Shape__ (__ShapeModel__)
  * one-to-one link (reverse) : one __Shape__ need one __Node__)
  * callable using find_also_related(__NodeModel__) from __Shape__
  * saved in __super_node__ field as foreing key in __ShapeModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Node",
//     name: "Node",
//     is_abstract: true,
//     super_class: [
//         "DiagramElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "DI.cmof#Node",
//     table_name: "di_node",
//     model_name: "Node",
//     full_name: "di_class_node",
// }

