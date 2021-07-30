// Generated from definition io.k8s.api.core.v1.ContainerStatus

/// ContainerStatus contains details for the current status of this container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerStatus {
    /// Container's ID in the format 'docker://\<container_id\>'.
    pub container_id: Option<String>,

    /// The image the container is running. More info: https://kubernetes.io/docs/concepts/containers/images
    pub image: String,

    /// ImageID of the container's image.
    pub image_id: String,

    /// Details about the container's last termination condition.
    pub last_state: Option<crate::api::core::v1::ContainerState>,

    /// This must be a DNS_LABEL. Each container in a pod must have a unique name. Cannot be updated.
    pub name: String,

    /// Specifies whether the container has passed its readiness probe.
    pub ready: bool,

    /// The number of times the container has been restarted, currently based on the number of dead containers that have not yet been removed. Note that this is calculated from dead containers. But those containers are subject to garbage collection. This value will get capped at 5 by GC.
    pub restart_count: i32,

    /// Details about the container's current condition.
    pub state: Option<crate::api::core::v1::ContainerState>,
}

impl<'de> crate::serde::Deserialize<'de> for ContainerStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_container_id,
            Key_image,
            Key_image_id,
            Key_last_state,
            Key_name,
            Key_ready,
            Key_restart_count,
            Key_state,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "containerID" => Field::Key_container_id,
                            "image" => Field::Key_image,
                            "imageID" => Field::Key_image_id,
                            "lastState" => Field::Key_last_state,
                            "name" => Field::Key_name,
                            "ready" => Field::Key_ready,
                            "restartCount" => Field::Key_restart_count,
                            "state" => Field::Key_state,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ContainerStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_container_id: Option<String> = None;
                let mut value_image: Option<String> = None;
                let mut value_image_id: Option<String> = None;
                let mut value_last_state: Option<crate::api::core::v1::ContainerState> = None;
                let mut value_name: Option<String> = None;
                let mut value_ready: Option<bool> = None;
                let mut value_restart_count: Option<i32> = None;
                let mut value_state: Option<crate::api::core::v1::ContainerState> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_container_id => value_container_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image => value_image = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_image_id => value_image_id = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_last_state => value_last_state = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_ready => value_ready = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_restart_count => value_restart_count = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_state => value_state = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerStatus {
                    container_id: value_container_id,
                    image: value_image.ok_or_else(|| crate::serde::de::Error::missing_field("image"))?,
                    image_id: value_image_id.ok_or_else(|| crate::serde::de::Error::missing_field("imageID"))?,
                    last_state: value_last_state,
                    name: value_name.ok_or_else(|| crate::serde::de::Error::missing_field("name"))?,
                    ready: value_ready.ok_or_else(|| crate::serde::de::Error::missing_field("ready"))?,
                    restart_count: value_restart_count.ok_or_else(|| crate::serde::de::Error::missing_field("restartCount"))?,
                    state: value_state,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerStatus",
            &[
                "containerID",
                "image",
                "imageID",
                "lastState",
                "name",
                "ready",
                "restartCount",
                "state",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ContainerStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerStatus",
            5 +
            self.container_id.as_ref().map_or(0, |_| 1) +
            self.last_state.as_ref().map_or(0, |_| 1) +
            self.state.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.container_id {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "containerID", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "image", &self.image)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "imageID", &self.image_id)?;
        if let Some(value) = &self.last_state {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lastState", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ready", &self.ready)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "restartCount", &self.restart_count)?;
        if let Some(value) = &self.state {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "state", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ContainerStatus {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.ContainerStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ContainerStatus contains details for the current status of this container.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: std::array::IntoIter::new([
                    (
                        "containerID".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Container's ID in the format 'docker://<container_id>'.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "image".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The image the container is running. More info: https://kubernetes.io/docs/concepts/containers/images".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "imageID".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ImageID of the container's image.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "lastState".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ContainerState>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Details about the container's last termination condition.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "name".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("This must be a DNS_LABEL. Each container in a pod must have a unique name. Cannot be updated.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ready".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies whether the container has passed its readiness probe.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "restartCount".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of times the container has been restarted, currently based on the number of dead containers that have not yet been removed. Note that this is calculated from dead containers. But those containers are subject to garbage collection. This value will get capped at 5 by GC.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "state".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ContainerState>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Details about the container's current condition.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ]).collect(),
                required: std::array::IntoIter::new([
                    "image",
                    "imageID",
                    "name",
                    "ready",
                    "restartCount",
                ]).map(std::borrow::ToOwned::to_owned).collect(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
