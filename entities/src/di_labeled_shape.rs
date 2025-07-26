//! di_class_labeled_shape

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_labeled_shape")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperShape
    pub super_shape: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE LabeledShape need ONE Shape
    #[sea_orm(
        belongs_to = "super::di_shape::Entity",
        from = "Column::SuperShape",
        to = "super::di_shape::Column::Id",
        on_delete = "Cascade"
    )]
    Shape,
    // SUPER : ONE BpmnShape need ONE LabeledShape
    #[sea_orm(has_one = "super::bpmndi_bpmn_shape::Entity")]
    BpmnShape,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'DI-LabeledShape',
//     name: "LabeledShape",
//     is_abstract: true,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'DI-Shape',
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "LabeledShape-ownedLabel": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'DI-LabeledShape-ownedLabel',
//                 name: "ownedLabel",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'DI-Label',
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
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'DI-A_ownedLabel_owningShape',
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "DI.cmof#LabeledShape",
//     table_name: "di_labeled_shape",
//     model_name: "LabeledShape",
//     full_name: "di_class_labeled_shape",
//     reverse_super: RefCell {
//         value: [
//             (Weak),
//         ],
//     },
// }

