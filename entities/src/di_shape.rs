//! di_class_shape

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_shape")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Node
    pub super_node: i64,
    /// COMPLEX FIELD : DI-Shape-bounds
    pub bounds: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Shape need ONE Node
    #[sea_orm(
        belongs_to = "super::di_node::Entity",
        from = "Column::SuperNode",
        to = "super::di_node::Column::Id",
        on_delete = "Cascade"
    )]
    Node,
    // SUPER : ONE LabeledShape need ONE Shape
    #[sea_orm(has_one = "super::di_labeled_shape::Entity")]
    LabeledShape,
}

// SUPER : ONE Shape need ONE Node
impl Related<super::di_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Node.def()
    }
}

// SUPER : ONE LabeledShape need ONE Shape
impl Related<super::di_labeled_shape::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LabeledShape.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Shape" (di_class_shape)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __Node__ (__NodeModel__)
    ///   * one-to-one link : one __Shape__ need one __Node__)
    ///   * callable using find_also_related(__NodeModel__) from __Shape__
    ///   * saved in __super_node__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __LabeledShape__ (__LabeledShapeModel__)
    ///   * one-to-one link (reverse) : one __LabeledShape__ need one __Shape__)
    ///   * callable using find_also_related(__ShapeModel__) from __LabeledShape__
    ///   * saved in __super_shape__ field as foreing key in __LabeledShapeModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Shape" (di_class_shape)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __Node__ (__NodeModel__)
  * one-to-one link : one __Shape__ need one __Node__)
  * callable using find_also_related(__NodeModel__) from __Shape__
  * saved in __super_node__ field as foreing key

## Reverse Super :
* __LabeledShape__ (__LabeledShapeModel__)
  * one-to-one link (reverse) : one __LabeledShape__ need one __Shape__)
  * callable using find_also_related(__ShapeModel__) from __LabeledShape__
  * saved in __super_shape__ field as foreing key in __LabeledShapeModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdLocalReference {
//         object_id: "Shape",
//         package_id: "DI",
//         is_set: true,
//     },
//     name: "Shape",
//     is_abstract: true,
//     super_class: [
//         "Node",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "Shape-bounds": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "Shape-bounds",
//                     package_id: "DI",
//                     is_set: true,
//                 },
//                 name: "bounds",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefDataType(
//                         HRefDataType {
//                             href: "RefCell of 'DC-Bounds' (loaded : true)",
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
//     technical_name: "DI.cmof#Shape",
//     table_name: "di_shape",
//     model_name: "Shape",
//     full_name: "di_class_shape",
// }

