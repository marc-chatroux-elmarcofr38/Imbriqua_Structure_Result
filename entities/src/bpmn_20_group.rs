//! bpmn_20_class_group

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_group")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Artifact
    pub super_artifact: i64,
    /// COMPLEX FIELD : BPMN20-Group-categoryValueRef
    pub category_value_ref: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Group need ONE Artifact
    #[sea_orm(
        belongs_to = "super::bpmn_20_artifact::Entity",
        from = "Column::SuperArtifact",
        to = "super::bpmn_20_artifact::Column::Id",
        on_delete = "Cascade"
    )]
    Artifact,
}

// SUPER : ONE Group need ONE Artifact
impl Related<super::bpmn_20_artifact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artifact.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Group" (bpmn_20_class_group)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __CategoryValue__ (__CategoryValueModel__) from A_categoryValueRef_categoryValueRef
    ///   * one-to-many link : (0-1) __Group__ need (0-inf) __CategoryValue__)
    ///   * callable using find_with_related(__CategoryValueModel__) from __Group__
    /// 
    /// ## Direct Super :
    /// * __Artifact__ (__ArtifactModel__)
    ///   * one-to-one link : one __Group__ need one __Artifact__)
    ///   * callable using find_also_related(__ArtifactModel__) from __Group__
    ///   * saved in __super_artifact__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Group" (bpmn_20_class_group)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Relation : One To Many :
* __CategoryValue__ (__CategoryValueModel__) from A_categoryValueRef_categoryValueRef
  * one-to-many link : (0-1) __Group__ need (0-inf) __CategoryValue__)
  * callable using find_with_related(__CategoryValueModel__) from __Group__

## Direct Super :
* __Artifact__ (__ArtifactModel__)
  * one-to-one link : one __Group__ need one __Artifact__)
  * callable using find_also_related(__ArtifactModel__) from __Group__
  * saved in __super_artifact__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "Group",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "Group",
//     is_abstract: false,
//     super_class: [
//         "Artifact",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-Group-categoryValueRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Group-categoryValueRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "categoryValueRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CategoryValue",
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
//                     "A_categoryValueRef_categoryValueRef",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Group",
//     table_name: "bpmn_20_group",
//     model_name: "Group",
//     full_name: "bpmn_20_class_group",
// }

