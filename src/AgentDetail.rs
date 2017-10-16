// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Deserialize, Debug, Clone)]
pub struct AgentDetail
{
	#[serde(rename = "browser")] name: String,
	#[serde(rename = "abbr")] abbreviated_name: String,
	prefix: Prefix,
	#[serde(rename = "type")] agentType: AgentType,
	usage_global: HashMap<Version, f64>,
	#[serde(rename = "versions")] eras_to_versions: Vec<Option<Version>>,
	#[serde(default)] prefix_exceptions: HashMap<Version, Prefix>
}

impl AgentDetail
{
	#[inline(always)]
	pub fn name(&self) -> &str
	{
		&self.name
	}
	
	#[inline(always)]
	pub fn abbreviatedName(&self) -> &str
	{
		&self.abbreviated_name
	}
	
	#[inline(always)]
	pub fn agentType(&self) -> AgentType
	{
		self.agentType
	}
	
	#[inline(always)]
	pub fn prefix<'a>(&'a self, version: &Version) -> &'a Prefix
	{
		match self.prefix_exceptions.get(version)
		{
			Some(prefix) => prefix,
			None => &self.prefix,
		}
	}
	
	/// Multiply by 100 to get a percentage
	#[inline(always)]
	pub fn globalUsageFraction(&self, version: &Version) -> Option<f64>
	{
		self.usage_global.get(version).map(|value| *value)
	}
	
	/// versions to eras; not super useful as eras aren't tied to dates, so to say 'e0' doesn't really define a point in time
	#[inline(always)]
	pub fn versionNearestToEra<'a>(&'a self, eras: &Eras, eraName: &EraName) -> Option<&'a Version>
	{
		let mut index = match eras.index(eraName)
		{
			None => if eraName.is_negative()
			{
				0
			}
			else
			{
				self.eras_to_versions.len() - 1
			},
			Some(index) => index,
		};
		
		loop
		{
			match self.eras_to_versions.get(index).unwrap()
			{
				&Some(ref version) => return Some(version),
				&None => if index == 0
				{
					return None;
				}
				else
				{
					index -=1;
				}
			}
		}
	}
}
