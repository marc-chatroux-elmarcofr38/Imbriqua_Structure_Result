//! bpmndi_class_bpmn_label

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmndi_bpmn_label")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperLabel
    pub super_label: i64,
    /// COMPLEX FIELD : BPMNDI-BPMNLabel-labelStyle
    pub label_style: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE BpmnLabel need ONE Label
    #[sea_orm(
        belongs_to = "super::di_label::Entity",
        from = "Column::SuperLabel",
        to = "super::di_label::Column::Id",
        on_delete = "Cascade"
    )]
    Label,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-BPMNLabel',
//     name: "BPMNLabel",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [
//         "Loaded XMIIdReference RefCell of 'DI-Label',
//     ],
//     owned_attribute: {
//         "BPMNLabel-labelStyle": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-BPMNLabel-labelStyle',
//                 name: "labelStyle",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMNDI-BPMNLabelStyle',
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
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMNDI-A_labelStyle_label',
//                 ),
//                 redefined_property_link: Some(
//                     Property(
//                         HRefRedefinedProperty {
//                             href: "Loaded XMIIdReference RefCell of 'DI-DiagramElement-style',
//                         },
//                     ),
//                 ),
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMNDI.cmof#BPMNLabel",
//     table_name: "bpmndi_bpmn_label",
//     model_name: "BpmnLabel",
//     full_name: "bpmndi_class_bpmn_label",
//     reverse_super: RefCell {
//         value: [],
//     },
// }

