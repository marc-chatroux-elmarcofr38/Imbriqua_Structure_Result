
/// Conversion of ChoreographyActivity (Class : ChoreographyActivity)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ChoreographyActivity",
///     name: "ChoreographyActivity",
///     is_abstract: true,
///     super_class: Some(
///         "FlowNode",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ChoreographyActivity-participantRefs",
///                 name: "participantRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Participant",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 2,
///                 upper: Infinity,
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
///                 association: "A_participantRefs_choreographyActivity",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ChoreographyActivity-initiatingParticipantRef",
///                 name: "initiatingParticipantRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Participant",
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
///                 association: "A_initiatingParticipantRef_choreographyActivity",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ChoreographyActivity-correlationKeys",
///                 name: "correlationKeys",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CorrelationKey",
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
///                 association: "A_correlationKeys_choreographyActivity",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ChoreographyActivity-loopType",
///                 name: "loopType",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ChoreographyLoopType",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "None",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct ChoreographyActivity<'a> {
    #[builder(setter(into))]
    pub participant_refs: Vec<&'a Participant<'a>>,
    #[builder(setter(into))]
    pub initiating_participant_ref: &'a Participant<'a>,
    #[builder(setter(into, strip_option), default)]
    pub correlation_keys: Option<Vec<&'a CorrelationKey<'a>>>,
    #[builder(setter(into), default = "ChoreographyLoopType::None")]
    pub loop_type: &'a ChoreographyLoopType<'a>,
}

