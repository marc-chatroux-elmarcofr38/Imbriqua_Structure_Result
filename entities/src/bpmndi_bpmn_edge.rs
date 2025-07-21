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

// SUPER : ONE BpmnEdge need ONE LabeledEdge
impl Related<super::di_labeled_edge::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LabeledEdge.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "BpmnEdge" (bpmndi_class_bpmn_edge)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __message_visible_kind__ (xmi_id : "BPMNDI-BPMNEdge-messageVisibleKind")
    ///   * type : __Option<MessageVisibleKind>__
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __LabeledEdge__ (__LabeledEdgeModel__)
    ///   * one-to-one link : one __BpmnEdge__ need one __LabeledEdge__)
    ///   * callable using find_also_related(__LabeledEdgeModel__) from __BpmnEdge__
    ///   * saved in __super_labeled_edge__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "BpmnEdge" (bpmndi_class_bpmn_edge)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __message_visible_kind__ (xmi_id : "BPMNDI-BPMNEdge-messageVisibleKind")
  * type : __Option<MessageVisibleKind>__



## Direct Super :
* __LabeledEdge__ (__LabeledEdgeModel__)
  * one-to-one link : one __BpmnEdge__ need one __LabeledEdge__)
  * callable using find_also_related(__LabeledEdgeModel__) from __BpmnEdge__
  * saved in __super_labeled_edge__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdLocalReference {
//         object_id: "BPMNEdge",
//         package_id: "BPMNDI",
//         is_set: true,
//     },
//     name: "BPMNEdge",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [
//         HRefClass(
//             HRefClass {
//                 href: "RefCell of 'DI-LabeledEdge' (loaded : true)",
//             },
//         ),
//     ],
//     owned_attribute: {
//         "BPMNEdge-bpmnElement": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "BPMNEdge-bpmnElement",
//                     package_id: "BPMNDI",
//                     is_set: true,
//                 },
//                 name: "bpmnElement",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefClass(
//                         HRefClass {
//                             href: "RefCell of 'BPMN20-BaseElement' (loaded : true)",
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
//                         HRefRedefinedProperty {
//                             href: "RefCell of 'DI-DiagramElement-modelElement' (loaded : true)",
//                         },
//                     ),
//                 ),
//                 subsetted_property_link: None,
//             },
//         ),
//         "BPMNEdge-label": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "BPMNEdge-label",
//                     package_id: "BPMNDI",
//                     is_set: true,
//                 },
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
//                         HRefSubsettedProperty {
//                             href: "RefCell of 'DI-LabeledEdge-ownedLabel' (loaded : true)",
//                         },
//                     ),
//                 ),
//             },
//         ),
//         "BPMNEdge-messageVisibleKind": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "BPMNEdge-messageVisibleKind",
//                     package_id: "BPMNDI",
//                     is_set: true,
//                 },
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
//         "BPMNEdge-sourceElement": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "BPMNEdge-sourceElement",
//                     package_id: "BPMNDI",
//                     is_set: true,
//                 },
//                 name: "sourceElement",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefClass(
//                         HRefClass {
//                             href: "RefCell of 'DI-DiagramElement' (loaded : true)",
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
//                         HRefRedefinedProperty {
//                             href: "RefCell of 'DI-Edge-source' (loaded : true)",
//                         },
//                     ),
//                 ),
//                 subsetted_property_link: None,
//             },
//         ),
//         "BPMNEdge-targetElement": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "BPMNEdge-targetElement",
//                     package_id: "BPMNDI",
//                     is_set: true,
//                 },
//                 name: "targetElement",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefClass(
//                         HRefClass {
//                             href: "RefCell of 'DI-DiagramElement' (loaded : true)",
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
//                         HRefRedefinedProperty {
//                             href: "RefCell of 'DI-Edge-target' (loaded : true)",
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
// }

