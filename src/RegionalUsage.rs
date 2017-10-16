// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Deserialize, Debug, Clone)]
pub struct RegionalUsage
{
	id: String,
	name: String,
	//month: "2017-09",
	//access_date: "2017-10-05",
	data: HashMap<AgentName, BTreeMap<Version, Option<UsagePercentage>>>,
	total: UsagePercentage,
}

impl RegionalUsage
{
	/// ISO-like code
	#[inline(always)]
	pub fn identifier(&self) -> &str
	{
		&self.id
	}
	
	/// Country name or similar
	#[inline(always)]
	pub fn country_or_region_name(&self) -> &str
	{
		&self.name
	}
	
	/// Total usage; may not add up to 100% (eg for Andorra, adds up to about 95%)
	#[inline(always)]
	pub fn total(&self) -> UsagePercentage
	{
		self.total
	}
	
	/// Usage; returns None if agentName has no known usages
	#[inline(always)]
	pub fn usage(&self, agentName: &AgentName, lowerBound: Bound<&Version>, upperBound: Bound<&Version>) -> Option<Range<Version, Option<UsagePercentage>>>
	{
		match self.data.get(agentName)
		{
			None => None,
			Some(entry) => Some(entry.range((lowerBound, upperBound)))
		}
	}
}
