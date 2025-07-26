//! bpmndi_class_bpmn_diagram

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmndi_bpmn_diagram")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperDiagram
    pub super_diagram: i64,
    /// COMPLEX FIELD : BPMNDI-BPMNDiagram-plane
    pub plane: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE BpmnDiagram need ONE Diagram
    #[sea_orm(
        belongs_to = "super::di_diagram::Entity",
        from = "Column::SuperDiagram",
        to = "super::di_diagram::Column::Id",
        on_delete = "Cascade"
    )]
    Diagram,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-BPMNDiagram',
//     name: "BPMNDiagram",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [
//         "Loaded XMIIdReference RefCell of 'DI-Diagram',
//     ],
//     owned_attribute: {
//         "BPMNDiagram-labelStyle": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-BPMNDiagram-labelStyle',
//                 name: "labelStyle",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMNDI-BPMNLabelStyle',
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMNDI-A_labelStyle_diagram',
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: Some(
//                     Property(
//                         HRefSubsettedProperty {
//                             href: "Loaded XMIIdReference RefCell of 'DI-Diagram-ownedStyle',
//                         },
//                     ),
//                 ),
//             },
//         ),
//         "BPMNDiagram-plane": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-BPMNDiagram-plane',
//                 name: "plane",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMNDI-BPMNPlane',
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMNDI-A_plane_diagram',
//                 ),
//                 redefined_property_link: Some(
//                     Property(
//                         HRefRedefinedProperty {
//                             href: "Loaded XMIIdReference RefCell of 'DI-Diagram-rootElement',
//                         },
//                     ),
//                 ),
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMNDI.cmof#BPMNDiagram",
//     table_name: "bpmndi_bpmn_diagram",
//     model_name: "BpmnDiagram",
//     full_name: "bpmndi_class_bpmn_diagram",
//     reverse_super: RefCell {
//         value: [],
//     },
// }

