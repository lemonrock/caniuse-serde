// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// A struct that exists to workaround Rust's lack (yet) of 'impl Trait'
#[derive(Debug, Clone)]
pub struct ParentCategoryIterator<'a>(Keys<'a, ParentCategory, Vec<Category>>);

impl<'a> Iterator for ParentCategoryIterator<'a>
{
	type Item = &'a ParentCategory;
	
	/// Returns the next AgentName object.
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		self.0.next()
	}
}

impl<'a> ExactSizeIterator for ParentCategoryIterator<'a>
{
	/// Returns the exact number of times the iterator will iterate.
	#[inline(always)]
	fn len(&self) -> usize
	{
		self.0.len()
	}
}
