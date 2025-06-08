//! item_aware_element

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of ItemAwareElement (Class : ItemAwareElement)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ItemAwareElement",
///     name: "ItemAwareElement",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ItemAwareElement-itemSubjectRef",
///                 name: "itemSubjectRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ItemDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_itemSubjectRef_itemAwareElement",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ItemAwareElement-dataState",
///                 name: "dataState",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataState",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: true,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataState_itemAwareElement",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct ItemAwareElement<'a> {
    #[builder(setter(into, strip_option), default)]
    pub item_subject_ref: Option<&'a ItemDefinition<'a>>,
    #[builder(setter(into, strip_option), default)]
    pub data_state: Option<&'a DataState<'a>>,
}

