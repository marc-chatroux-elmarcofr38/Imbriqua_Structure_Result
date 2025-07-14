//! bpmn_20_class_end_point

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_end_point")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : RootElement
    pub super_root_element: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE EndPoint need ONE RootElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_root_element::Entity",
        from = "Column::SuperRootElement",
        to = "super::bpmn_20_root_element::Column::Id",
        on_delete = "Cascade"
    )]
    RootElement,
}

// SUPER : ONE EndPoint need ONE RootElement
impl Related<super::bpmn_20_root_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RootElement.def()
    }
}

// ManyToMany : with Participant using A_endPointRefs_participant
impl Related<super::bpmn_20_a_end_point_refs_participant::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_end_point_refs_participant::Relation::Participant.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_end_point_refs_participant::Relation::EndPoint
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "EndPoint" (bpmn_20_class_end_point)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __RootElement__ (__RootElementModel__)
    ///   * one-to-one link : one __EndPoint__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __EndPoint__
    ///   * saved in __super_root_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "EndPoint" (bpmn_20_class_end_point)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __RootElement__ (__RootElementModel__)
  * one-to-one link : one __EndPoint__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __EndPoint__
  * saved in __super_root_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "EndPoint",
//     name: "EndPoint",
//     is_abstract: false,
//     super_class: [
//         "RootElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }

