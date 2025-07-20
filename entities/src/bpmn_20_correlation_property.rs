//! bpmn_20_class_correlation_property

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_correlation_property")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : RootElement
    pub super_root_element: i64,
    /// COMPLEX FIELD : BPMN20-CorrelationProperty-type
    pub r#type: Option<i64>,
    /// SIMPLE FIELD : BPMN20-CorrelationProperty-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE CorrelationProperty need ONE RootElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_root_element::Entity",
        from = "Column::SuperRootElement",
        to = "super::bpmn_20_root_element::Column::Id",
        on_delete = "Cascade"
    )]
    RootElement,
}

// SUPER : ONE CorrelationProperty need ONE RootElement
impl Related<super::bpmn_20_root_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RootElement.def()
    }
}

// ManyToMany : with CorrelationKey using A_correlationPropertyRef_correlationKey
impl Related<super::bpmn_20_a_correlation_property_ref_correlation_key::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_correlation_property_ref_correlation_key::Relation::CorrelationKey.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_correlation_property_ref_correlation_key::Relation::CorrelationProperty
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "CorrelationProperty" (bpmn_20_class_correlation_property)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "BPMN20-CorrelationProperty-name")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __ItemDefinition__ (__ItemDefinitionModel__) from A_type_correlationProperty
    ///   * one-to-many link : (0-1) __CorrelationProperty__ need (0-inf) __ItemDefinition__)
    ///   * callable using find_with_related(__ItemDefinitionModel__) from __CorrelationProperty__
    /// 
    /// ## Direct Super :
    /// * __RootElement__ (__RootElementModel__)
    ///   * one-to-one link : one __CorrelationProperty__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __CorrelationProperty__
    ///   * saved in __super_root_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "CorrelationProperty" (bpmn_20_class_correlation_property)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "BPMN20-CorrelationProperty-name")
  * type : __std::string::String__


## Relation : One To Many :
* __ItemDefinition__ (__ItemDefinitionModel__) from A_type_correlationProperty
  * one-to-many link : (0-1) __CorrelationProperty__ need (0-inf) __ItemDefinition__)
  * callable using find_with_related(__ItemDefinitionModel__) from __CorrelationProperty__

## Direct Super :
* __RootElement__ (__RootElementModel__)
  * one-to-one link : one __CorrelationProperty__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __CorrelationProperty__
  * saved in __super_root_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "CorrelationProperty",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "CorrelationProperty",
//     is_abstract: false,
//     super_class: [
//         "RootElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-CorrelationProperty-correlationPropertyRetrievalExpression": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "CorrelationProperty-correlationPropertyRetrievalExpression",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "correlationPropertyRetrievalExpression",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CorrelationPropertyRetrievalExpression",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
//                 upper: Infinity,
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
//                     "A_correlationPropertyRetrievalExpression_correlationproperty",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-CorrelationProperty-name": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "CorrelationProperty-name",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
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
//         "-CorrelationProperty-type": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "CorrelationProperty-type",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "r#type",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ItemDefinition",
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
//                     "A_type_correlationProperty",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#CorrelationProperty",
//     table_name: "bpmn_20_correlation_property",
//     model_name: "CorrelationProperty",
//     full_name: "bpmn_20_class_correlation_property",
// }

