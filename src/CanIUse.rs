// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// A database of data relating to caniuse.com
/// Not used directly, but references should be passed to methods on AgentName, FeatureName, EraName, and, less usefully, Status and ParentCategory.
#[derive(Deserialize, Debug, Clone)]
pub struct CanIUse
{
	agents: HashMap<AgentName, AgentDetail>,
	statuses: HashMap<Status, String>,
	#[serde(rename = "cats")] child_categories: HashMap<ParentCategory, Vec<Category>>,
	#[serde(deserialize_with = "CanIUse::updated_deserialize")] updated: DateTime<Utc>,
	#[serde(rename = "data")] features: HashMap<FeatureName, FeatureDetail>,
}

impl Default for CanIUse
{
	/// Defaults to the up-to-date version of the caniuse.com database shipped embedded in this crate.
	#[inline(always)]
	fn default() -> Self
	{
		match include_str!("data-2.0.json").parse()
		{
			Err(error) => panic!("Invalid data embedded: {}", error),
			Ok(canIUse) => canIUse
		}
	}
}

impl FromStr for CanIUse
{
	type Err = ::serde_json::error::Error;
	
	/// Deserialize a CanIUse database from a UTF-8 string representing the contents of a `data-2.0.json` file (typically in `fulldata-json/`).
	#[inline(always)]
	fn from_str(can_i_use_database_json: &str) -> Result<Self, Self::Err>
	{
		::serde_json::from_str(can_i_use_database_json)
	}
}

impl CanIUse
{
	/// Deserialize a CanIUse database from a file path to a `data-2.0.json` file (typically in `fulldata-json/`).
	#[inline(always)]
	pub fn from_path<P: AsRef<Path>>(can_i_use_database_file_path: P) -> Result<Self, Box<::std::error::Error>>
	{
		Self::from_reader(File::open(can_i_use_database_file_path)?)
	}
	
	/// Deserialize a CanIUse database from a readable stream of raw JSON bytes.
	#[inline(always)]
	pub fn from_reader<R: Read>(reader_of_stream_of_can_i_use_json_bytes: R) -> Result<Self, Box<::std::error::Error>>
	{
		Ok(serde_json::from_reader(reader_of_stream_of_can_i_use_json_bytes)?)
	}
	
	/// Deserialize a CanIUse database from a slice of raw JSON bytes.
	#[inline(always)]
	pub fn from_slice(raw_can_i_use_json_bytes: &[u8]) -> Result<Self, ::serde_json::error::Error>
	{
		Ok(serde_json::from_slice(raw_can_i_use_json_bytes)?)
	}
	
	/// A timestamp of when this particular database was last updated.
	#[inline(always)]
	pub fn last_updated(&self) -> DateTime<Utc>
	{
		self.updated
	}
	
	/// An iterator over the AgentNames known in this caniuse.com database
	#[inline(always)]
	pub fn known_agent_names(&self) -> AgentNameIterator
	{
		AgentNameIterator(self.agents.keys())
	}
	
	/// An iterator over the AgentNames known in this caniuse.com database
	#[inline(always)]
	pub fn known_statuses(&self) -> StatusIterator
	{
		StatusIterator(self.statuses.keys())
	}
	
	/// An iterator over the AgentNames known in this caniuse.com database
	#[inline(always)]
	pub fn known_parent_categories(&self) -> ParentCategoryIterator
	{
		ParentCategoryIterator(self.child_categories.keys())
	}
	
	/// An iterator over the AgentNames known in this caniuse.com database
	#[inline(always)]
	pub fn known_feature_names(&self) -> FeatureNameIterator
	{
		FeatureNameIterator(self.features.keys())
	}
	
	#[inline(always)]
	fn agent<'a>(&'a self, agent_name: &'a AgentName) -> Option<Agent<'a>>
	{
		self.agents.get(agent_name).map(|agent_detail| Agent
		{
			agent_name,
			agent_detail,
		})
	}
	
	#[inline(always)]
	fn feature<'a>(&'a self, feature_name: &'a FeatureName) -> Option<Feature<'a>>
	{
		self.features.get(feature_name).map(|feature_detail| Feature
		{
			feature_name,
			feature_detail,
		})
	}
	
	#[inline(always)]
	fn status_description<'a>(&'a self, statusName: &Status) -> Option<&'a str>
	{
		self.statuses.get(statusName).map(|value| value.as_str())
	}
	
	#[inline(always)]
	fn child_categories<'a>(&'a self, parentCategory: &ParentCategory) -> Option<&'a [Category]>
	{
		self.child_categories.get(parentCategory).map(|value| &value[..])
	}
	
	#[inline(always)]
	fn updated_deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
	{
		struct OurVisitor;
		
		impl<'de> Visitor<'de> for OurVisitor
		{
			type Value = u64;
			
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("a positive integer")
			}
			
			fn visit_u8<E: de::Error>(self, value: u8) -> Result<Self::Value, E>
			{
				Ok(value as Self::Value)
			}
			
			fn visit_u16<E: de::Error>(self, value: u16) -> Result<Self::Value, E>
			{
				Ok(value as Self::Value)
			}
			
			fn visit_u32<E: de::Error>(self, value: u32) -> Result<Self::Value, E>
			{
				Ok(value as Self::Value)
			}
			
			fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E>
			{
				Ok(value)
			}
			
			fn visit_i8<E: de::Error>(self, value: i8) -> Result<Self::Value, E>
			{
				if value >= 0
				{
					Ok(value as Self::Value)
				}
				else
				{
					Err(E::custom(format!("i8 should not be negative '{}'", value)))
				}
			}
			
			fn visit_i16<E: de::Error>(self, value: i16) -> Result<Self::Value, E>
			{
				if value >= 0
				{
					Ok(value as Self::Value)
				}
				else
				{
					Err(E::custom(format!("i16 should not be negative '{}'", value)))
				}
			}
			
			fn visit_i32<E: de::Error>(self, value: i32) -> Result<Self::Value, E>
			{
				if value >= 0
				{
					Ok(value as Self::Value)
				}
				else
				{
					Err(E::custom(format!("i32 should not be negative '{}'", value)))
				}
			}
			
			fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E>
			{
				if value >= 0
				{
					Ok(value as Self::Value)
				}
				else
				{
					Err(E::custom(format!("i64 should not be negative '{}'", value)))
				}
			}
		}
		
		let time = deserializer.deserialize_u64(OurVisitor)?;
		// Cast here is deliberate; we deliberately parse expecting a non-negative timestamp
		let utc = Utc;
		Ok(utc.timestamp(time as i64, 0))
	}
}

lazy_static!
{
	/// The up-to-date version of the caniuse.com database shipped embedded in this crate.
	#[derive(Debug)] pub static ref EmbeddedCanIUseDatabase: CanIUse = CanIUse::default();
}
