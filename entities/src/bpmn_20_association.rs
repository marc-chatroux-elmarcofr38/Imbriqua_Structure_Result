//! bpmn_20_class_association

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_association")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Artifact
    pub super_artifact: i64,
    /// COMPLEX FIELD : Association-sourceRef
    pub source_ref: i64,
    /// COMPLEX FIELD : Association-targetRef
    pub target_ref: i64,
    /// SIMPLE FIELD : Association-associationDirection
    pub association_direction: AssociationDirection,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Association need ONE Artifact
    #[sea_orm(
        belongs_to = "super::bpmn_20_artifact::Entity",
        from = "Column::SuperArtifact",
        to = "super::bpmn_20_artifact::Column::Id"
    )]
    Artifact,
}

// SUPER : ONE Association need ONE Artifact
impl Related<super::bpmn_20_artifact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artifact.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "Association",
//     name: "Association",
//     is_abstract: false,
//     super_class: [
//         "Artifact",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "Association-associationDirection",
//                 name: "associationDirection",
//                 visibility: Public,
//                 simple_type: Some(
//                     "AssociationDirection",
//                 ),
//                 complex_type: None,
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "Association-sourceRef",
//                 name: "sourceRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BaseElement",
//                 ),
//                 complex_type: None,
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
//                 association: Some(
//                     "A_sourceRef_outgoing_association",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "Association-targetRef",
//                 name: "targetRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BaseElement",
//                 ),
//                 complex_type: None,
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
//                 association: Some(
//                     "A_targetRef_incoming_association",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

