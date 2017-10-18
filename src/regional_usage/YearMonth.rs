// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// A naive structure representing a year and month
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct YearMonth
{
	year: u16,
	one_based_month: u8,
}

impl<'de> Deserialize<'de> for YearMonth
{
	/// Deserialize using Serde
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct YearMonthVisitor;
		
		impl<'de> Visitor<'de> for YearMonthVisitor
		{
			type Value = YearMonth;
			
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("a year-month string")
			}
			
			fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E>
			{
				let mut split = v.split('-');
				let year = match split.next()
				{
					None => return Err(E::custom("no year")),
					Some(value) => if value.len() != 4
					{
						return Err(E::custom("year wasn't 4 characters"));
					}
					else
					{
						match value.parse::<u16>()
						{
							Err(parseError) => return Err(E::custom(parseError)),
							Ok(year) => if year < 2000 || year > 2100
							{
								return Err(E::custom("year seems suspect"))
							}
							else
							{
								year
							}
						}
					}
				};
				
				let one_based_month = match split.next()
				{
					None => return Err(E::custom("no month")),
					Some(value) => if value.len() != 2
					{
						return Err(E::custom("month wasn't 2 characters"));
					}
					else
					{
						match value.parse::<u8>()
						{
							Err(parseError) => return Err(E::custom(parseError)),
							Ok(one_based_month) => if one_based_month < 1 || one_based_month > 12
							{
								return Err(E::custom("month seems suspect"))
							}
							else
							{
								one_based_month
							}
						}
					}
				};
				
				if split.next().is_some()
				{
					return Err(E::custom("additional data after month"));
				}
				
				Ok
				(
					YearMonth
					{
						year,
						one_based_month,
					}
				)
			}
		}
		
		deserializer.deserialize_str(YearMonthVisitor)
	}
}

impl YearMonth
{
	/// Year, eg 2017. Does not support BC, and note there is no such thing as AD 0...
	#[inline(always)]
	pub fn year(&self) -> u16
	{
		self.year
	}
	
	/// One-based month ordinal
	#[inline(always)]
	pub fn one_based_month(&self) -> u8
	{
		self.one_based_month
	}
	
	/// Zero-based month ordinal
	#[inline(always)]
	pub fn zero_based_month(&self) -> u8
	{
		self.one_based_month() - 1
	}
}
