//! bpmndi_class_bpmn_edge

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmndi_bpmn_edge")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperLabeledEdge
    pub super_labeled_edge: i64,
    /// COMPLEX FIELD : BPMNDI-BPMNEdge-bpmnElement
    pub bpmn_element: Option<i64>,
    /// COMPLEX FIELD : BPMNDI-BPMNEdge-label
    pub label: Option<i64>,
    /// COMPLEX FIELD : BPMNDI-BPMNEdge-sourceElement
    pub source_element: Option<i64>,
    /// COMPLEX FIELD : BPMNDI-BPMNEdge-targetElement
    pub target_element: Option<i64>,
    /// SIMPLE FIELD : BPMNDI-BPMNEdge-messageVisibleKind
    pub message_visible_kind: Option<MessageVisibleKind>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE BpmnEdge need ONE LabeledEdge
    #[sea_orm(
        belongs_to = "super::di_labeled_edge::Entity",
        from = "Column::SuperLabeledEdge",
        to = "super::di_labeled_edge::Column::Id",
        on_delete = "Cascade"
    )]
    LabeledEdge,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-BPMNEdge',
//     name: "BPMNEdge",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [
//         "Loaded XMIIdReference RefCell of 'DI-LabeledEdge',
//     ],
//     owned_attribute: {
//         "BPMNEdge-bpmnElement": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-BPMNEdge-bpmnElement',
//                 name: "bpmnElement",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefClass(
//                         HRefClass {
//                             href: "Loaded XMIIdReference RefCell of 'BPMN20-BaseElement',
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
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMNDI-A_bpmnElement_edge',
//                 ),
//                 redefined_property_link: Some(
//                     Property(
//                         HRefRedefinedProperty {
//                             href: "Loaded XMIIdReference RefCell of 'DI-DiagramElement-modelElement',
//                         },
//                     ),
//                 ),
//                 subsetted_property_link: None,
//             },
//         ),
//         "BPMNEdge-label": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-BPMNEdge-label',
//                 name: "label",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMNDI-BPMNLabel',
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
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
//                     "Loaded XMIIdReference RefCell of 'BPMNDI-A_label_edge',
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: Some(
//                     Property(
//                         HRefSubsettedProperty {
//                             href: "Loaded XMIIdReference RefCell of 'DI-LabeledEdge-ownedLabel',
//                         },
//                     ),
//                 ),
//             },
//         ),
//         "BPMNEdge-messageVisibleKind": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-BPMNEdge-messageVisibleKind',
//                 name: "messageVisibleKind",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMNDI-MessageVisibleKind',
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "BPMNEdge-sourceElement": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-BPMNEdge-sourceElement',
//                 name: "sourceElement",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefClass(
//                         HRefClass {
//                             href: "Loaded XMIIdReference RefCell of 'DI-DiagramElement',
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
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMNDI-A_sourceElement_sourceEdge',
//                 ),
//                 redefined_property_link: Some(
//                     Property(
//                         HRefRedefinedProperty {
//                             href: "Loaded XMIIdReference RefCell of 'DI-Edge-source',
//                         },
//                     ),
//                 ),
//                 subsetted_property_link: None,
//             },
//         ),
//         "BPMNEdge-targetElement": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMNDI-BPMNEdge-targetElement',
//                 name: "targetElement",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefClass(
//                         HRefClass {
//                             href: "Loaded XMIIdReference RefCell of 'DI-DiagramElement',
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
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMNDI-A_targetElement_targetEdge',
//                 ),
//                 redefined_property_link: Some(
//                     Property(
//                         HRefRedefinedProperty {
//                             href: "Loaded XMIIdReference RefCell of 'DI-Edge-target',
//                         },
//                     ),
//                 ),
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMNDI.cmof#BPMNEdge",
//     table_name: "bpmndi_bpmn_edge",
//     model_name: "BpmnEdge",
//     full_name: "bpmndi_class_bpmn_edge",
//     reverse_super: RefCell {
//         value: [],
//     },
// }

