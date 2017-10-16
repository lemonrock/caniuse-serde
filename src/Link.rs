// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Link
{
	#[serde(with = "url_serde")] url: Url,
	title: String,
}

impl Link
{
	/// url
	#[inline(always)]
	pub fn url(&self) -> &Url
	{
		&self.url
	}
	
	/// title
	#[inline(always)]
	pub fn title(&self) -> &str
	{
		&self.title
	}
}
