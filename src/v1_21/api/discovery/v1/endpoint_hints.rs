// Generated from definition io.k8s.api.discovery.v1.EndpointHints

/// EndpointHints provides hints describing how an endpoint should be consumed.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EndpointHints {
    /// forZones indicates the zone(s) this endpoint should be consumed by to enable topology aware routing.
    pub for_zones: Option<Vec<crate::api::discovery::v1::ForZone>>,
}

impl<'de> serde::Deserialize<'de> for EndpointHints {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_for_zones,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "forZones" => Field::Key_for_zones,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EndpointHints;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EndpointHints")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_for_zones: Option<Vec<crate::api::discovery::v1::ForZone>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_for_zones => value_for_zones = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EndpointHints {
                    for_zones: value_for_zones,
                })
            }
        }

        deserializer.deserialize_struct(
            "EndpointHints",
            &[
                "forZones",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for EndpointHints {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EndpointHints",
            self.for_zones.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.for_zones {
            serde::ser::SerializeStruct::serialize_field(&mut state, "forZones", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
