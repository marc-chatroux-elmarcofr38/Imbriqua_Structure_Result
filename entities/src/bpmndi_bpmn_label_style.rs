//! bpmndi_class_bpmn_label_style

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmndi_bpmn_label_style")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperStyle
    pub super_style: i64,
    /// COMPLEX FIELD : BPMNDI-BPMNLabelStyle-font
    pub font: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE BpmnLabelStyle need ONE Style
    #[sea_orm(
        belongs_to = "super::di_style::Entity",
        from = "Column::SuperStyle",
        to = "super::di_style::Column::Id",
        on_delete = "Cascade"
    )]
    Style,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-BPMNLabelStyle',
//     name: "BPMNLabelStyle",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [
//         "Loaded XMIIdReference RefCell of 'DI-Style',
//     ],
//     owned_attribute: {
//         "BPMNLabelStyle-font": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-BPMNLabelStyle-font',
//                 name: "font",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefDataType(
//                         HRefDataType {
//                             href: "Loaded XMIIdReference RefCell of 'DC-Font',
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
//                 owning_association: None,
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMNDI.cmof#BPMNLabelStyle",
//     table_name: "bpmndi_bpmn_label_style",
//     model_name: "BpmnLabelStyle",
//     full_name: "bpmndi_class_bpmn_label_style",
//     reverse_super: RefCell {
//         value: [],
//     },
// }

