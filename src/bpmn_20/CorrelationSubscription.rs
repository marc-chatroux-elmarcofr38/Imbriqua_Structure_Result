
/// Conversion of CorrelationSubscription (Class : CorrelationSubscription)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CorrelationSubscription",
///     name: "CorrelationSubscription",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationSubscription-correlationKeyRef",
///                 name: "correlationKeyRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CorrelationKey",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
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
///                 association: "A_correlationKeyRef_correlationSubscription",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationSubscription-correlationPropertyBinding",
///                 name: "correlationPropertyBinding",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CorrelationPropertyBinding",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
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
///                 association: "A_correlationPropertyBinding_correlationSubscription",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct CorrelationSubscription<'a> {
    #[builder(setter(into))]
    pub correlation_key_ref: &'a CorrelationKey<'a>,
    #[builder(setter(into, strip_option), default)]
    pub correlation_property_binding: Option<Vec<&'a CorrelationPropertyBinding<'a>>>,
}

