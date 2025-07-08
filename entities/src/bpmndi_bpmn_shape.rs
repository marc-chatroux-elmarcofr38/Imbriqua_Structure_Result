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
    /// COMPLEX FIELD : BPMNShape-bpmnElement
    pub bpmn_element: Option<i64>,
    /// COMPLEX FIELD : BPMNShape-label
    pub label: Option<i64>,
    /// COMPLEX FIELD : BPMNShape-choreographyActivityShape
    pub choreography_activity_shape: Option<i64>,
    /// SIMPLE FIELD : BPMNShape-isHorizontal
    pub is_horizontal: Option<std::primitive::bool>,
    /// SIMPLE FIELD : BPMNShape-isExpanded
    pub is_expanded: Option<std::primitive::bool>,
    /// SIMPLE FIELD : BPMNShape-isMarkerVisible
    pub is_marker_visible: Option<std::primitive::bool>,
    /// SIMPLE FIELD : BPMNShape-isMessageVisible
    pub is_message_visible: Option<std::primitive::bool>,
    /// SIMPLE FIELD : BPMNShape-participantBandKind
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

// RAW :
// CMOFClass {
//     xmi_id: "BPMNShape",
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
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "BPMNShape-bpmnElement",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "BPMNShape-isHorizontal",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "BPMNShape-isExpanded",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "BPMNShape-isMarkerVisible",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "BPMNShape-label",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "BPMNShape-isMessageVisible",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "BPMNShape-participantBandKind",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "BPMNShape-choreographyActivityShape",
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
//     ],
//     owned_rule: [],
// }

