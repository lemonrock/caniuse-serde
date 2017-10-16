// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Deserialize, Debug, Clone)]
pub struct CanIUse
{
	eras: Eras,
	agents: HashMap<AgentName, AgentDetail>,
	statuses: HashMap<Status, String>,
	#[serde(rename = "cats")] childCategories: HashMap<ParentCategory, Vec<Category>>,
	updated: DateTime<Utc>,
	#[serde(rename = "data")] features: HashMap<FeatureName, FeatureDetail>,
}

impl Default for CanIUse
{
	#[inline(always)]
	fn default() -> Self
	{
		match CanIUse::RawStringData.parse()
		{
			Err(error) => panic!("Invalid data embedded: {}", error),
			Ok(canIUse) => canIUse
		}
	}
}

impl FromStr for CanIUse
{
	type Err = ::serde_json::error::Error;
	
	#[inline(always)]
	fn from_str(canIUseDatabaseJson: &str) -> Result<Self, Self::Err>
	{
		::serde_json::from_str(canIUseDatabaseJson)
	}
}

lazy_static!
{
	/// The up-to-date version of the caniuse.com database shipped embedded in this crate
   	pub static ref EmbeddedCanIUseDatabase: CanIUse = CanIUse::default();
}

impl CanIUse
{
	const RawStringData: &'static str = include_str!("data.json");
	
	#[inline(always)]
	pub fn from_path<P: AsRef<Path>>(canIUseJsonDatabaseFilePath: P) -> Result<Self, Box<::std::error::Error>>
	{
		Self::from_reader(File::open(canIUseJsonDatabaseFilePath)?)
	}
	
	#[inline(always)]
	pub fn from_reader<R: Read>(readerOfStreamOfCanIUseJsonBytes: R) -> Result<Self, Box<::std::error::Error>>
	{
		Ok(serde_json::from_reader(readerOfStreamOfCanIUseJsonBytes)?)
	}
	
	#[inline(always)]
	pub fn from_slice(rawCanIUseJsonBytes: &[u8]) -> Result<Self, ::serde_json::error::Error>
	{
		Ok(serde_json::from_slice(rawCanIUseJsonBytes)?)
	}
	
	#[inline(always)]
	pub fn lastUpdated(&self) -> DateTime<Utc>
	{
		self.updated
	}
	
	#[inline(always)]
	fn agent<'a>(&'a self, agentName: &'a AgentName) -> Option<Agent<'a>>
	{
		self.agents.get(agentName).map(|agentDetail| Agent
		{
			eras: &self.eras,
			agentName,
			agentDetail,
		})
	}
	
	#[inline(always)]
	fn feature<'a>(&'a self, featureName: &'a FeatureName) -> Option<Feature<'a>>
	{
		self.features.get(featureName).map(|featureDetail| Feature
		{
			featureName,
			featureDetail,
		})
	}
	
	#[inline(always)]
	fn statusDescription<'a>(&'a self, statusName: &Status) -> Option<&'a str>
	{
		self.statuses.get(statusName).map(|value| value.as_str())
	}
	
	#[inline(always)]
	fn childCategories<'a>(&'a self, parentCategory: &ParentCategory) -> Option<&'a [Category]>
	{
		self.childCategories.get(parentCategory).map(|value| &value[..])
	}
}
