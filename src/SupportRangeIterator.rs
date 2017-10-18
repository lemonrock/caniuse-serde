// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// A struct that exists to workaround Rust's lack (yet) of 'impl Trait'
#[derive(Debug, Clone)]
pub struct SupportRangeIterator<'a>
{
	feature: &'a Feature<'a>,
	range: Range<'a, Version, SupportDetail>,
}

impl<'a> Iterator for SupportRangeIterator<'a>
{
	type Item = (&'a Version, Support<'a>);
	
	/// Returns the next Support object
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let next = self.range.next();
		self.transform(next)
	}
}

impl<'a> DoubleEndedIterator for SupportRangeIterator<'a>
{
	/// Returns the previous Support object
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item>
	{
		let next = self.range.next_back();
		self.transform(next)
	}
}

impl<'a> SupportRangeIterator<'a>
{
	#[inline(always)]
	fn transform(&self, next: Option<(&'a Version, &'a SupportDetail)>) -> Option<(&'a Version, Support<'a>)>
	{
		next.map
		(
			|(version, support_detail)|
			{
				(
					version,
					Support
					{
						support_detail,
						feature: self.feature,
					}
				)
			}
		)
	}
}
