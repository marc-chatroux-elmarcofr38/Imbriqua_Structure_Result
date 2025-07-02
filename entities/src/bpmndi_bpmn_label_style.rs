//! bpmndi_class_bpmn_label_style

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmndi_bpmn_label_style")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : Style
    pub super_style: i32,
    /// COMPLEX FIELD : BPMNLabelStyle-font
    pub font: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::di_style::Entity",
        from = "Column::SuperStyle",
        to = "super::di_style::Column::Id"
    )]
    Style,
}

// `Related` trait has to be implemented by hand
impl Related<super::di_style::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Style.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "BPMNLabelStyle",
//     name: "BPMNLabelStyle",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [
//         Class(
//             SuperClass {
//                 href: "DI.cmof#Style",
//             },
//         ),
//     ],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "BPMNLabelStyle-font",
//                 name: "font",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     DataTypeLink(
//                         DataTypeLink {
//                             href: "DC.cmof#Font",
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
//     ],
//     owned_rule: [],
// }

