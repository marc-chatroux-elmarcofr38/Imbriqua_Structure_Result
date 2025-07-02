//! bpmndi_class_bpmn_label

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmndi_bpmn_label")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : Label
    pub super_label: i32,
    /// COMPLEX FIELD : BPMNLabel-labelStyle
    pub label_style: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::di_label::Entity",
        from = "Column::SuperLabel",
        to = "super::di_label::Column::Id"
    )]
    Label,
}

// `Related` trait has to be implemented by hand
impl Related<super::di_label::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Label.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "BPMNLabel",
//     name: "BPMNLabel",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [
//         Class(
//             SuperClass {
//                 href: "DI.cmof#Label",
//             },
//         ),
//     ],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "BPMNLabel-labelStyle",
//                 name: "labelStyle",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BPMNLabelStyle",
//                 ),
//                 complex_type: None,
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
//                     "A_labelStyle_label",
//                 ),
//                 redefined_property_link: Some(
//                     Property(
//                         RedefinedProperty {
//                             href: "DI.cmof#DiagramElement-style",
//                         },
//                     ),
//                 ),
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

