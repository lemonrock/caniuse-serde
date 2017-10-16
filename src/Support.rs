// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct Support<'a>
{
	agentName: &'a AgentName,
	versionRange: &'a Version,
	supportDetail: &'a SupportDetail,
	notes_by_num: &'a BTreeMap<u8, String>,
}

impl<'a> Support<'a>
{
	#[inline(always)]
	pub fn maturity(&self) -> SupportMaturity
	{
		self.supportDetail.maturity()
	}
	
	#[inline(always)]
	pub fn requires_prefix(&self) -> bool
	{
		self.supportDetail.requiresPrefix()
	}
	
	#[inline(always)]
	pub fn disabled_by_default(&self) -> bool
	{
		self.supportDetail.disabledByDefault()
	}
	
	/// A list of note pairs of number (which is one-based; the returned Vec is zero-based) and string
	#[inline(always)]
	pub fn notes(&'a self) -> Vec<(u8, &'a str)>
	{
		let noteNumbers = self.supportDetail.oneBasedNoteNumbers();
		let mut notes = Vec::with_capacity(noteNumbers.len());
		
		for oneBasedNoteNumber in noteNumbers
		{
			let noteString = self.notes_by_num.get(oneBasedNoteNumber).expect("Invalid caniuse.com database - note numbers do not tally").as_str();
			notes.push((*oneBasedNoteNumber, noteString))
		}
		
		notes
	}
}
