//! di_class_labeled_shape

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_labeled_shape")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Shape
    pub super_shape: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE LabeledShape need ONE Shape
    #[sea_orm(
        belongs_to = "super::di_shape::Entity",
        from = "Column::SuperShape",
        to = "super::di_shape::Column::Id"
    )]
    Shape,
    // SUPER : ONE BpmnShape need ONE LabeledShape
    #[sea_orm(has_one = "super::bpmndi_bpmn_shape::Entity")]
    BpmnShape,
}

// SUPER : ONE LabeledShape need ONE Shape
impl Related<super::di_shape::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Shape.def()
    }
}

// SUPER : ONE BpmnShape need ONE LabeledShape
impl Related<super::bpmndi_bpmn_shape::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BpmnShape.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "LabeledShape",
//     name: "LabeledShape",
//     is_abstract: true,
//     super_class: [
//         "Shape",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "LabeledShape-ownedLabel",
//                 name: "ownedLabel",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Label",
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
//                 subsetted_property: Some(
//                     "DiagramElement-ownedElement",
//                 ),
//                 owning_association: "",
//                 association: Some(
//                     "A_ownedLabel_owningShape",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

