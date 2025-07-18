//! bpmndi_class_bpmn_shape

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmndi_bpmn_shape")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : LabeledShape
    pub super_labeled_shape: i64,
    /// COMPLEX FIELD : BPMNDI-BPMNShape-bpmnElement
    pub bpmn_element: Option<i64>,
    /// COMPLEX FIELD : BPMNDI-BPMNShape-choreographyActivityShape
    pub choreography_activity_shape: Option<i64>,
    /// COMPLEX FIELD : BPMNDI-BPMNShape-label
    pub label: Option<i64>,
    /// SIMPLE FIELD : BPMNDI-BPMNShape-isExpanded
    pub is_expanded: Option<std::primitive::bool>,
    /// SIMPLE FIELD : BPMNDI-BPMNShape-isHorizontal
    pub is_horizontal: Option<std::primitive::bool>,
    /// SIMPLE FIELD : BPMNDI-BPMNShape-isMarkerVisible
    pub is_marker_visible: Option<std::primitive::bool>,
    /// SIMPLE FIELD : BPMNDI-BPMNShape-isMessageVisible
    pub is_message_visible: Option<std::primitive::bool>,
    /// SIMPLE FIELD : BPMNDI-BPMNShape-participantBandKind
    pub participant_band_kind: Option<ParticipantBandKind>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE BpmnShape need ONE LabeledShape
    #[sea_orm(
        belongs_to = "super::di_labeled_shape::Entity",
        from = "Column::SuperLabeledShape",
        to = "super::di_labeled_shape::Column::Id",
        on_delete = "Cascade"
    )]
    LabeledShape,
}

// SUPER : ONE BpmnShape need ONE LabeledShape
impl Related<super::di_labeled_shape::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LabeledShape.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "BpmnShape" (bpmndi_class_bpmn_shape)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __is_expanded__ (xmi_id : "BPMNDI-BPMNShape-isExpanded")
    ///   * type : __Option<std::primitive::bool>__
    /// * __is_horizontal__ (xmi_id : "BPMNDI-BPMNShape-isHorizontal")
    ///   * type : __Option<std::primitive::bool>__
    /// * __is_marker_visible__ (xmi_id : "BPMNDI-BPMNShape-isMarkerVisible")
    ///   * type : __Option<std::primitive::bool>__
    /// * __is_message_visible__ (xmi_id : "BPMNDI-BPMNShape-isMessageVisible")
    ///   * type : __Option<std::primitive::bool>__
    /// * __participant_band_kind__ (xmi_id : "BPMNDI-BPMNShape-participantBandKind")
    ///   * type : __Option<ParticipantBandKind>__
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __LabeledShape__ (__LabeledShapeModel__)
    ///   * one-to-one link : one __BpmnShape__ need one __LabeledShape__)
    ///   * callable using find_also_related(__LabeledShapeModel__) from __BpmnShape__
    ///   * saved in __super_labeled_shape__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "BpmnShape" (bpmndi_class_bpmn_shape)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __is_expanded__ (xmi_id : "BPMNDI-BPMNShape-isExpanded")
  * type : __Option<std::primitive::bool>__
* __is_horizontal__ (xmi_id : "BPMNDI-BPMNShape-isHorizontal")
  * type : __Option<std::primitive::bool>__
* __is_marker_visible__ (xmi_id : "BPMNDI-BPMNShape-isMarkerVisible")
  * type : __Option<std::primitive::bool>__
* __is_message_visible__ (xmi_id : "BPMNDI-BPMNShape-isMessageVisible")
  * type : __Option<std::primitive::bool>__
* __participant_band_kind__ (xmi_id : "BPMNDI-BPMNShape-participantBandKind")
  * type : __Option<ParticipantBandKind>__



## Direct Super :
* __LabeledShape__ (__LabeledShapeModel__)
  * one-to-one link : one __BpmnShape__ need one __LabeledShape__)
  * callable using find_also_related(__LabeledShapeModel__) from __BpmnShape__
  * saved in __super_labeled_shape__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "BPMNShape",
//         package_id: "BPMNDI",
//         is_set: true,
//     },
//     name: "BPMNShape",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [
//         Class(
//             SuperClass {
//                 href: "DI.cmof#LabeledShape",
//             },
//         ),
//     ],
//     owned_attribute: {
//         "-BPMNShape-bpmnElement": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "BPMNShape-bpmnElement",
//                     package_id: "BPMNDI",
//                     is_set: true,
//                 },
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
//                     "A_bpmnElement_shape",
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
//         "-BPMNShape-choreographyActivityShape": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "BPMNShape-choreographyActivityShape",
//                     package_id: "BPMNDI",
//                     is_set: true,
//                 },
//                 name: "choreographyActivityShape",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BPMNShape",
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
//                     "A_choreographyActivityShape_participantBandShape",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-BPMNShape-isExpanded": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "BPMNShape-isExpanded",
//                     package_id: "BPMNDI",
//                     is_set: true,
//                 },
//                 name: "isExpanded",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#Boolean",
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-BPMNShape-isHorizontal": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "BPMNShape-isHorizontal",
//                     package_id: "BPMNDI",
//                     is_set: true,
//                 },
//                 name: "isHorizontal",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#Boolean",
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-BPMNShape-isMarkerVisible": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "BPMNShape-isMarkerVisible",
//                     package_id: "BPMNDI",
//                     is_set: true,
//                 },
//                 name: "isMarkerVisible",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#Boolean",
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-BPMNShape-isMessageVisible": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "BPMNShape-isMessageVisible",
//                     package_id: "BPMNDI",
//                     is_set: true,
//                 },
//                 name: "isMessageVisible",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#Boolean",
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-BPMNShape-label": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "BPMNShape-label",
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
//                     "A_label_shape",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: Some(
//                     Property(
//                         SubsettedProperty {
//                             href: "DI.cmof#LabeledShape-ownedLabel",
//                         },
//                     ),
//                 ),
//             },
//         ),
//         "-BPMNShape-participantBandKind": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "BPMNShape-participantBandKind",
//                     package_id: "BPMNDI",
//                     is_set: true,
//                 },
//                 name: "participantBandKind",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ParticipantBandKind",
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
//     },
//     owned_rule: {},
//     technical_name: "BPMNDI.cmof#BPMNShape",
//     table_name: "bpmndi_bpmn_shape",
//     model_name: "BpmnShape",
//     full_name: "bpmndi_class_bpmn_shape",
// }

