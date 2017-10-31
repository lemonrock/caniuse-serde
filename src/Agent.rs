// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// An agent is effectively a browser. It is not a rendering engine, although it is closely related
#[derive(Debug, Clone)]
pub struct Agent<'a>
{
	agent_name: &'a AgentName,
	agent_detail: &'a AgentDetail,
}

impl<'a> Agent<'a>
{
	/// Agent name
	#[inline(always)]
	pub fn agent_name(&self) -> &'a AgentName
	{
		self.agent_name
	}
	
	/// Agent browser name
	#[inline(always)]
	pub fn browser_name(&self) -> &str
	{
		&self.agent_detail.name
	}
	
	/// Agent detail abbreviated name, eg 'Chr.' for chrome
	#[inline(always)]
	pub fn abbreviated_name(&self) -> &str
	{
		&self.agent_detail.abbreviated_name
	}
	
	/// prefix to use for this particular version (lacks leading and trailing dash)
	/// varies per version only for legacy Opera using the Presto rendering engine (from -webkit- to -o-)
	#[inline(always)]
	pub fn prefix(&self, version: &Version) -> &'a Prefix
	{
		match self.agent_detail.prefix_exceptions.get(version)
		{
			Some(prefix) => prefix,
			None => &self.agent_detail.prefix,
		}
	}
	
	/// Is this a desktop or mobile agent?
	#[inline(always)]
	pub fn agent_type(&self) -> AgentType
	{
		self.agent_detail.agent_type
	}
	
	/// Global usage; differs from `VersionDetail.global_usage()` **and** from `RegionalUsage::WorldWide`.
	/// It is recommended to use the values in `RegionalUsage::WorldWide` for consistency.
	#[inline(always)]
	pub fn global_usage(&self, version: &Version) -> Option<UsagePercentage>
	{
		self.agent_detail.usage_global.get(version).map(|value| *value)
	}
	
	/// Details of every known version.
	#[inline(always)]
	pub fn version_details(&'a self) -> &'a BTreeMap<Version, VersionDetail>
	{
		&self.agent_detail.version_list
	}
	
	/// Current version as of `CanIUse.last_updated()`.
	#[inline(always)]
	pub fn current_version(&'a self) -> &'a Version
	{
		&self.agent_detail.current_version
	}
	
	/// Historic prefixes.
	/// Only currently supplied for Opera Presto versions.
	/// Duplicated in `version_details()`
	#[inline(always)]
	pub fn prefix_exceptions(&'a self) -> &'a BTreeMap<Version, Prefix>
	{
		&self.agent_detail.prefix_exceptions
	}
}
