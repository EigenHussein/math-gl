use serde::{self, de::Visitor, ser::SerializeStruct, Deserialize, Serialize};
use serde_json;
use std::fmt;

use serde::de::{self, Deserializer, MapAccess, SeqAccess};

#[derive(Serialize,Debug,Default)]
struct Student {
    name: String,
    age: u32,
    stage: u32,
}
impl Student {
    fn new(name: &str, age: u32, stage: u32) -> Self {
        Self {
            name: name.to_string(),
            age: age,
            stage: stage,
        }
    }
}

impl<'de> Deserialize<'de> for Student {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Name,
            Age,
            Stage,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`secs` or `nanos`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "name" => Ok(Field::Name),
                            "age" => Ok(Field::Age),
                            "stage" => Ok(Field::Stage),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct StudentVisitor;

        impl<'de> Visitor<'de> for StudentVisitor {
            type Value = Student;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Duration")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Student, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let name = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let age = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let stage = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                Ok(Student {
                    name: name,
                    age: age,
                    stage: stage,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Student, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                let mut age = None;
                let mut stage = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Age => {
                            if age.is_some() {
                                return Err(de::Error::duplicate_field("age"));
                            }
                            age = Some(map.next_value()?);
                        }
                        Field::Stage => {
                            if stage.is_some() {
                                return Err(de::Error::duplicate_field("stage"));
                            }
                            stage = Some(map.next_value()?);
                        }
                    }
                }
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let age = age.ok_or_else(|| de::Error::missing_field("age"))?;
                let stage = stage.ok_or_else(|| de::Error::missing_field("stage"))?;
                Ok(Student {
                    name: name,
                    age: age,
                    stage: stage,
                })
            }
        }

        const FIELDS: &[&str] = &["name", "age", "stage"];
        deserializer.deserialize_struct("Student", FIELDS, StudentVisitor)
    }
}

fn main() {
    let ser = serde_json::to_string(&Student {
        name: "Naming".to_string(),
        age: 21,
        stage: 34,
    })
    .unwrap();

    let deser: Student = serde_json::from_str(ser.as_str()).unwrap();
    println!("{:?}", deser);
}
