//! bpmn_20_class_correlation_key

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_correlation_key")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// SIMPLE FIELD : CorrelationKey-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE CorrelationKey need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE CorrelationKey need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// ManyToMany : with CorrelationProperty using A_correlationPropertyRef_correlationKey
impl Related<super::bpmn_20_a_correlation_property_ref_correlation_key::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_correlation_property_ref_correlation_key::Relation::CorrelationProperty.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_correlation_property_ref_correlation_key::Relation::CorrelationKey
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "CorrelationKey" (bpmn_20_class_correlation_key)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "CorrelationKey-name")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __ChoreographyActivity__ (__ChoreographyActivityModel__) from A_correlationKeys_choreographyActivity
    ///   * one-to-many link : (0-1) __CorrelationKey__ need (0-inf) __ChoreographyActivity__)
    ///   * callable using find_with_related(__ChoreographyActivityModel__) from __CorrelationKey__
    ///   * named choreography_activity in BPMN
    /// * __Collaboration__ (__CollaborationModel__) from A_correlationKeys_collaboration
    ///   * one-to-many link : (0-1) __CorrelationKey__ need (0-inf) __Collaboration__)
    ///   * callable using find_with_related(__CollaborationModel__) from __CorrelationKey__
    ///   * named collaboration in BPMN
    /// * __ConversationNode__ (__ConversationNodeModel__) from A_correlationKeys_conversationNode
    ///   * one-to-many link : (0-1) __CorrelationKey__ need (0-inf) __ConversationNode__)
    ///   * callable using find_with_related(__ConversationNodeModel__) from __CorrelationKey__
    ///   * named conversation_node in BPMN
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __CorrelationKey__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __CorrelationKey__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "CorrelationKey" (bpmn_20_class_correlation_key)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "CorrelationKey-name")
  * type : __std::string::String__


## Relation : One To Many :
* __ChoreographyActivity__ (__ChoreographyActivityModel__) from A_correlationKeys_choreographyActivity
  * one-to-many link : (0-1) __CorrelationKey__ need (0-inf) __ChoreographyActivity__)
  * callable using find_with_related(__ChoreographyActivityModel__) from __CorrelationKey__
  * named choreography_activity in BPMN
* __Collaboration__ (__CollaborationModel__) from A_correlationKeys_collaboration
  * one-to-many link : (0-1) __CorrelationKey__ need (0-inf) __Collaboration__)
  * callable using find_with_related(__CollaborationModel__) from __CorrelationKey__
  * named collaboration in BPMN
* __ConversationNode__ (__ConversationNodeModel__) from A_correlationKeys_conversationNode
  * one-to-many link : (0-1) __CorrelationKey__ need (0-inf) __ConversationNode__)
  * callable using find_with_related(__ConversationNodeModel__) from __CorrelationKey__
  * named conversation_node in BPMN

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __CorrelationKey__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __CorrelationKey__
  * saved in __super_base_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "CorrelationKey",
//     name: "CorrelationKey",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "CorrelationKey-correlationPropertyRef": Property(
//             CMOFProperty {
//                 xmi_id: "CorrelationKey-correlationPropertyRef",
//                 name: "correlationPropertyRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CorrelationProperty",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
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
//                     "A_correlationPropertyRef_correlationKey",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "CorrelationKey-name": Property(
//             CMOFProperty {
//                 xmi_id: "CorrelationKey-name",
//                 name: "name",
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#CorrelationKey",
//     table_name: "bpmn_20_correlation_key",
//     model_name: "CorrelationKey",
//     full_name: "bpmn_20_class_correlation_key",
// }

