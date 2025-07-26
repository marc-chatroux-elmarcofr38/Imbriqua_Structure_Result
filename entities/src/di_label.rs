//! di_class_label

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_label")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperNode
    pub super_node: i64,
    /// COMPLEX FIELD : DI-Label-bounds
    pub bounds: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Label need ONE Node
    #[sea_orm(
        belongs_to = "super::di_node::Entity",
        from = "Column::SuperNode",
        to = "super::di_node::Column::Id",
        on_delete = "Cascade"
    )]
    Node,
    // SUPER : ONE BpmnLabel need ONE Label
    #[sea_orm(has_one = "super::bpmndi_bpmn_label::Entity")]
    BpmnLabel,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'DI-Label',
//     name: "Label",
//     is_abstract: true,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'DI-Node',
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "Label-bounds": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'DI-Label-bounds',
//                 name: "bounds",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefDataType(
//                         HRefDataType {
//                             href: "Loaded XMIIdReference RefCell of 'DC-Bounds',
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
//                 owning_association: None,
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "DI.cmof#Label",
//     table_name: "di_label",
//     model_name: "Label",
//     full_name: "di_class_label",
//     reverse_super: RefCell {
//         value: [
//             (Weak),
//         ],
//     },
// }

