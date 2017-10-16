// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FeatureName(String);

impl<I: Into<String>> From<I> for FeatureName
{
	#[inline(always)]
	fn from(value: I) -> Self
	{
		FeatureName(value.into())
	}
}

impl FromStr for FeatureName
{
	type Err = ();
	
	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		Ok(FeatureName(s.to_owned()))
	}
}

impl Deref for FeatureName
{
	type Target = str;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl FeatureName
{
	#[inline(always)]
	pub fn feature<'a>(&'a self, canIUse: &'a CanIUse) -> Option<Feature<'a>>
	{
		canIUse.feature(self)
	}
}
