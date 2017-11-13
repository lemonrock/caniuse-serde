// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// Represents the error that can occur when parsing a str to get a Regional Usages.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct RegionalUsagesFromStrError;

impl Error for RegionalUsagesFromStrError
{
	#[inline(always)]
	fn description(&self) -> &str
	{
		"unknown caniuse ISO-like code"
	}
	
	#[inline(always)]
	fn cause(&self) -> Option<&Error>
	{
		None
	}
}

impl Display for RegionalUsagesFromStrError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", self.description())
	}
}
