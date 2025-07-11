//! bpmn_20_class_text_annotation

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_text_annotation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Artifact
    pub super_artifact: i64,
    /// SIMPLE FIELD : TextAnnotation-text
    pub text: std::string::String,
    /// SIMPLE FIELD : TextAnnotation-textFormat
    #[sea_orm(default_value = "text/plain")]
    pub text_format: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE TextAnnotation need ONE Artifact
    #[sea_orm(
        belongs_to = "super::bpmn_20_artifact::Entity",
        from = "Column::SuperArtifact",
        to = "super::bpmn_20_artifact::Column::Id",
        on_delete = "Cascade"
    )]
    Artifact,
}

// SUPER : ONE TextAnnotation need ONE Artifact
impl Related<super::bpmn_20_artifact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artifact.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "TextAnnotation" (bpmn_20_class_text_annotation)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __text__ (xmi_id : "TextAnnotation-text")
    ///   * type : __std::string::String__
    /// * __text_format__ (xmi_id : "TextAnnotation-textFormat")
    ///   * type : __std::string::String__
    ///   * default : "text/plain"
    /// 
    /// 
    /// ## Direct Super :
    /// * __Artifact__ (__ArtifactModel__)
    ///   * one-to-one link : one __TextAnnotation__ need one __Artifact__)
    ///   * callable using find_also_related(__ArtifactModel__) from __TextAnnotation__
    ///   * saved in __super_artifact__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "TextAnnotation" (bpmn_20_class_text_annotation)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __text__ (xmi_id : "TextAnnotation-text")
  * type : __std::string::String__
* __text_format__ (xmi_id : "TextAnnotation-textFormat")
  * type : __std::string::String__
  * default : "text/plain"


## Direct Super :
* __Artifact__ (__ArtifactModel__)
  * one-to-one link : one __TextAnnotation__ need one __Artifact__)
  * callable using find_also_related(__ArtifactModel__) from __TextAnnotation__
  * saved in __super_artifact__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "TextAnnotation",
//     name: "TextAnnotation",
//     is_abstract: false,
//     super_class: [
//         "Artifact",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "TextAnnotation-text",
//                 name: "text",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#String",
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
//                 owning_association: "",
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "TextAnnotation-textFormat",
//                 name: "textFormat",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#String",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "text/plain",
//                 ),
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

