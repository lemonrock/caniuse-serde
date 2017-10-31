// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// Details about a particular version, of which only the release_date is particularly useful.
/// The era is a relative value which can change with releases of the caniuse.com database, and the global_usage can differ to that available in `RegionalUsage::WorldWide`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VersionDetail
{
	global_usage: UsagePercentage,
	release_date: DateTime<Utc>,
	era: i64,
	prefix: Prefix,
}

impl VersionDetail
{
	/// A global usage of this version; one of three measurements included in the caniuse.com database.
	/// It is recommended to use that in the `RegionalUsage::WorldWide` database instead as RegionalUsage data has greater consistency.
	#[inline(always)]
	pub fn global_usage(&self) -> UsagePercentage
	{
		self.global_usage
	}
	
	/// A timestamp of when this particular version was released.
	/// It is likely that the hours, minutes and seconds represent false precision.
	#[inline(always)]
	pub fn release_date(&self) -> DateTime<Utc>
	{
		self.release_date
	}
	
	/// Eras are the caniuse.com database's attempt to align different browsers by version.
	/// Negative values are for not current versions.
	/// Zero is for the current version.
	/// The era is a relative value which which can change with releases of the caniuse.com database.
	#[inline(always)]
	pub fn era(&self) -> i64
	{
		self.era
	}
	
	/// Prefix for CSS properties.
	#[inline(always)]
	pub fn prefix<'a>(&'a self) -> &'a Prefix
	{
		&self.prefix
	}
}
