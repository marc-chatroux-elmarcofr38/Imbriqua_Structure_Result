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
        to = "super::bpmn_20_artifact::Column::Id",
        on_delete = "Cascade"
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

impl ActiveModel {
    /// # Help document for "Association" (bpmn_20_class_association)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __association_direction__ (xmi_id : "Association-associationDirection")
    ///   * type : __AssociationDirection__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __BaseElement__ (__BaseElementModel__) from A_sourceRef_outgoing_association
    ///   * one-to-many link : (1-1) __Association__ need (0-inf) __BaseElement__)
    ///   * callable using find_with_related(__BaseElementModel__) from __Association__
    /// * __BaseElement__ (__BaseElementModel__) from A_targetRef_incoming_association
    ///   * one-to-many link : (1-1) __Association__ need (0-inf) __BaseElement__)
    ///   * callable using find_with_related(__BaseElementModel__) from __Association__
    /// 
    /// ## Direct Super :
    /// * __Artifact__ (__ArtifactModel__)
    ///   * one-to-one link : one __Association__ need one __Artifact__)
    ///   * callable using find_also_related(__ArtifactModel__) from __Association__
    ///   * saved in __super_artifact__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Association" (bpmn_20_class_association)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __association_direction__ (xmi_id : "Association-associationDirection")
  * type : __AssociationDirection__


## Relation : One To Many :
* __BaseElement__ (__BaseElementModel__) from A_sourceRef_outgoing_association
  * one-to-many link : (1-1) __Association__ need (0-inf) __BaseElement__)
  * callable using find_with_related(__BaseElementModel__) from __Association__
* __BaseElement__ (__BaseElementModel__) from A_targetRef_incoming_association
  * one-to-many link : (1-1) __Association__ need (0-inf) __BaseElement__)
  * callable using find_with_related(__BaseElementModel__) from __Association__

## Direct Super :
* __Artifact__ (__ArtifactModel__)
  * one-to-one link : one __Association__ need one __Artifact__)
  * callable using find_also_related(__ArtifactModel__) from __Association__
  * saved in __super_artifact__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Association",
//     name: "Association",
//     is_abstract: false,
//     super_class: [
//         "Artifact",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "Association-associationDirection": Property(
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
//         "Association-sourceRef": Property(
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
//         "Association-targetRef": Property(
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Association",
//     table_name: "bpmn_20_association",
//     model_name: "Association",
//     full_name: "bpmn_20_class_association",
// }

