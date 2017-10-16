// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct Agent<'a>
{
	eras: &'a Eras,
	agentName: &'a AgentName,
	agentDetail: &'a AgentDetail,
}

impl<'a> Agent<'a>
{
	#[inline(always)]
	pub fn agentName(&self) -> &'a AgentName
	{
		self.agentName
	}
	
	#[inline(always)]
	pub fn agentDetailName(&self) -> &str
	{
		self.agentDetail.name()
	}
	
	#[inline(always)]
	pub fn abbreviatedName(&self) -> &str
	{
		self.agentDetail.abbreviatedName()
	}
	
	/// desktop or mobile agent
	#[inline(always)]
	pub fn agentType(&self) -> AgentType
	{
		self.agentDetail.agentType()
	}
	
	/// prefix to use for this particular version (lacks leading and trailing dash)
	/// varies per version only for legacy Opera using the Presto rendering engine (from -webkit- to -o-)
	#[inline(always)]
	pub fn prefix(&self, version: &VersionRange) -> &'a Prefix
	{
		self.agentDetail.prefix(version)
	}
	
	/// Multiply by 100 to get a percentage
	#[inline(always)]
	pub fn globalUsageFraction(&self, version: &VersionRange) -> Option<f64>
	{
		self.agentDetail.globalUsageFraction(version)
	}
	
	/// versions to eras; not super useful as eras aren't tied to dates, so to say 'e0' doesn't really define a point in time
	#[inline(always)]
	pub fn versionNearestToEra(&self, eraName: &EraName) -> Option<&'a VersionRange>
	{
		self.agentDetail.versionNearestToEra(self.eras, eraName)
	}
}
