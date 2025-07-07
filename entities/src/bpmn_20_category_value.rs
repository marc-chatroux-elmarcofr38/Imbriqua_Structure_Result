//! bpmn_20_class_category_value

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_category_value")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// SIMPLE FIELD : CategoryValue-value
    pub value: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE CategoryValue need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id"
    )]
    BaseElement,
}

// SUPER : ONE CategoryValue need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// ManyToMany : with FlowElement using A_categorizedFlowElements_categoryValueRef
impl Related<super::bpmn_20_a_categorized_flow_elements_category_value_ref::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_categorized_flow_elements_category_value_ref::Relation::FlowElement.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_categorized_flow_elements_category_value_ref::Relation::CategoryValue
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "CategoryValue",
//     name: "CategoryValue",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "CategoryValue-categorizedFlowElements",
//                 name: "categorizedFlowElements",
//                 visibility: Public,
//                 simple_type: Some(
//                     "FlowElement",
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
//                 is_derived: true,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_categorizedFlowElements_categoryValueRef",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "CategoryValue-value",
//                 name: "value",
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
//     ],
//     owned_rule: [],
// }

