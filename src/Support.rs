// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// Represents details of support
pub struct Support<'a>
{
	support_detail: &'a SupportDetail,
	feature: &'a Feature<'a>,
}

impl<'a> Support<'a>
{
	/// How mature is support?
	#[inline(always)]
	pub fn maturity(&self) -> SupportMaturity
	{
		self.support_detail.maturity()
	}
	
	/// Does support require a prefix?
	#[inline(always)]
	pub fn requires_prefix(&self) -> bool
	{
		self.support_detail.requires_prefix()
	}
	
	/// Is support behind a flag or some other mechanism that isn't normally enabled in a default install?
	#[inline(always)]
	pub fn disabled_by_default(&self) -> bool
	{
		self.support_detail.disabled_by_default()
	}
	
	/// Returns a list of pairs of one-based note numbers (the list itself is zero-based) and note text
	/// Panics if the feature does not contain the note number; this is only possible if the caniuse.com database is invalid or the wrong Feature is passed
	#[inline(always)]
	pub fn notes(&'a self) -> Vec<(u8, &'a str)>
	{
		self.support_detail.notes(self.feature)
	}
}
