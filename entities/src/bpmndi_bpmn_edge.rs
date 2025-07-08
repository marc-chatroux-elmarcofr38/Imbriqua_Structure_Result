//! bpmndi_class_bpmn_edge

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmndi_bpmn_edge")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : LabeledEdge
    pub super_labeled_edge: i64,
    /// COMPLEX FIELD : BPMNEdge-label
    pub label: Option<i64>,
    /// COMPLEX FIELD : BPMNEdge-bpmnElement
    pub bpmn_element: Option<i64>,
    /// COMPLEX FIELD : BPMNEdge-sourceElement
    pub source_element: Option<i64>,
    /// COMPLEX FIELD : BPMNEdge-targetElement
    pub target_element: Option<i64>,
    /// SIMPLE FIELD : BPMNEdge-messageVisibleKind
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

// SUPER : ONE BpmnEdge need ONE LabeledEdge
impl Related<super::di_labeled_edge::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LabeledEdge.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "BPMNEdge",
//     name: "BPMNEdge",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [
//         Class(
//             SuperClass {
//                 href: "DI.cmof#LabeledEdge",
//             },
//         ),
//     ],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "BPMNEdge-label",
//                 name: "label",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BPMNLabel",
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
//                 owning_association: "",
//                 association: Some(
//                     "A_label_edge",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: Some(
//                     Property(
//                         SubsettedProperty {
//                             href: "DI.cmof#LabeledEdge-ownedLabel",
//                         },
//                     ),
//                 ),
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "BPMNEdge-bpmnElement",
//                 name: "bpmnElement",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     ClassLink(
//                         ClassLink {
//                             href: "BPMN20.cmof#BaseElement",
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
//                 owning_association: "",
//                 association: Some(
//                     "A_bpmnElement_edge",
//                 ),
//                 redefined_property_link: Some(
//                     Property(
//                         RedefinedProperty {
//                             href: "DI.cmof#DiagramElement-modelElement",
//                         },
//                     ),
//                 ),
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "BPMNEdge-sourceElement",
//                 name: "sourceElement",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     ClassLink(
//                         ClassLink {
//                             href: "DI.cmof#DiagramElement",
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
//                 owning_association: "",
//                 association: Some(
//                     "A_sourceElement_sourceEdge",
//                 ),
//                 redefined_property_link: Some(
//                     Property(
//                         RedefinedProperty {
//                             href: "DI.cmof#Edge-source",
//                         },
//                     ),
//                 ),
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "BPMNEdge-targetElement",
//                 name: "targetElement",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     ClassLink(
//                         ClassLink {
//                             href: "DI.cmof#DiagramElement",
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
//                 owning_association: "",
//                 association: Some(
//                     "A_targetElement_targetEdge",
//                 ),
//                 redefined_property_link: Some(
//                     Property(
//                         RedefinedProperty {
//                             href: "DI.cmof#Edge-target",
//                         },
//                     ),
//                 ),
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "BPMNEdge-messageVisibleKind",
//                 name: "messageVisibleKind",
//                 visibility: Public,
//                 simple_type: Some(
//                     "MessageVisibleKind",
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

