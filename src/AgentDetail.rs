// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Deserialize, Debug, Clone)]
struct AgentDetail
{
	#[serde(rename = "browser")] name: String,
	#[serde(rename = "abbr")] abbreviated_name: String,
	prefix: Prefix,
	#[serde(rename = "type")] agent_type: AgentType,
	usage_global: BTreeMap<Version, UsagePercentage>,
	#[serde(deserialize_with = "AgentDetail::deserialize_version_list")] version_list: BTreeMap<Version, VersionDetail>,
	current_version: Version,
	#[serde(default)] prefix_exceptions: BTreeMap<Version, Prefix>
}

impl AgentDetail
{
	fn deserialize_version_list<'de, D: Deserializer<'de>>(deserializer: D) -> Result<BTreeMap<Version, VersionDetail>, D::Error>
	{
		use de::Error as SerdeError;
		
		/*
			A typical JSON version list entry would be:-
		
			{
				"version": "9",
				"global_usage": 0.0082,
				"release_date": 1150761600,
				"era": -44,
				"prefix": "o"
			}
		*/
		
		enum Field
		{
			version,
			global_usage,
			release_date,
			era,
			prefix,
		}
		
		const FieldNames: &'static [&'static str] = &
		[
			"version",
			"global_usage",
			"release_date",
			"era",
			"prefix",
		];
		
		impl<'de> Deserialize<'de> for Field
		{
			#[inline(always)]
			fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Field, D::Error>
			{
				struct FieldVisitor;
				
				impl<'de> Visitor<'de> for FieldVisitor
				{
					type Value = Field;
					
					fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
					{
						formatter.write_str("one of version, global_usage, release_date, era or prefix")
					}
					
					fn visit_str<E: SerdeError>(self, value: &str) -> Result<Field, E>
					{
						match value
						{
							"version" => Ok(Field::version),
							"global_usage" => Ok(Field::global_usage),
							"release_date" => Ok(Field::release_date),
							"era" => Ok(Field::era),
							"prefix" => Ok(Field::prefix),
							_ => Err(SerdeError::unknown_field(value, FieldNames)),
						}
					}
				}
				
				deserializer.deserialize_identifier(FieldVisitor)
			}
		}
		
		struct VersionListEntryDeserializeSeed<'a>(&'a mut BTreeMap<Version, VersionDetail>);
		
		impl<'a, 'de> DeserializeSeed<'de> for VersionListEntryDeserializeSeed<'a>
		{
			type Value = ();
			
			#[inline(always)]
			fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error>
			{
				struct VersionListEntryVisitor<'a>(&'a mut BTreeMap<Version, VersionDetail>);
				
				impl<'a, 'de> Visitor<'de> for VersionListEntryVisitor<'a>
				{
					type Value = ();
					
					#[inline(always)]
					fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
					{
						formatter.write_str("a version list entry (Version, VersionDetail)")
					}
					
					fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error>
					{
						let mut version = None;
						let mut global_usage = None;
						let mut release_date = None;
						let mut era = None;
						let mut prefix = None;
						while let Some(field) = map.next_key()?
						{
							match field
							{
								Field::version =>
								{
									version = Some(map.next_value()?);
								}
								
								Field::global_usage =>
								{
									global_usage = Some(map.next_value()?);
								}
								
								Field::release_date =>
								{
									release_date = Some(map.next_value()?);
								}
								
								Field::era =>
								{
									era = Some(map.next_value()?);
								}
								
								Field::prefix =>
								{
									prefix = Some(map.next_value()?);
								}
							}
						}
						let version = version.ok_or_else(|| SerdeError::missing_field("version"))?;
						let global_usage = global_usage.ok_or_else(|| SerdeError::missing_field("global_usage"))?;
						let release_date_u64: u64 = release_date.ok_or_else(|| SerdeError::missing_field("release_date"))?;
						let era = era.ok_or_else(|| SerdeError::missing_field("era"))?;
						let prefix = prefix.ok_or_else(|| SerdeError::missing_field("prefix"))?;
						
						// Cast here is deliberate; we deliberately parse expecting a non-negative timestamp
						let release_date = Utc.timestamp(release_date_u64 as i64, 0);
						
						self.0.insert(version, VersionDetail
						{
							global_usage,
							release_date,
							era,
							prefix,
						});
						
						Ok(())
					}
				}
				
				deserializer.deserialize_map(VersionListEntryVisitor(self.0))
			}
		}
		
		struct VersionListVisitor;
		
		impl<'de> Visitor<'de> for VersionListVisitor
		{
			type Value = BTreeMap<Version, VersionDetail>;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("a version list (Version, VersionDetail)")
			}
			
			fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Self::Value, A::Error>
			{
				let mut map = BTreeMap::new();
				
				while let Some(()) = sequence.next_element_seed(VersionListEntryDeserializeSeed(&mut map))?
				{
					// Nothing to do; we have mutated the map using VersionListEntryDeserializeSeed
				}
				
				Ok(map)
			}
		}
		
		deserializer.deserialize_seq(VersionListVisitor)
	}
}
