//! data_store_reference

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of DataStoreReference (Class : DataStoreReference)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataStoreReference",
///     name: "DataStoreReference",
///     is_abstract: false,
///     super_class: Some(
///         "ItemAwareElement FlowElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataStoreReference-dataStoreRef",
///                 name: "dataStoreRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataStore",
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
///                 association: "A_dataStoreRef_dataStoreReference",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct DataStoreReference<'a> {
    #[builder(setter(into, strip_option), default)]
    pub data_store_ref: Option<&'a DataStore<'a>>,
}

