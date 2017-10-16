// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct EraName(i64);

impl<'de> Deserialize<'de> for EraName
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct EraNameVisitor;
		
		impl<'de> Visitor<'de> for EraNameVisitor
		{
			type Value = EraName;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("an era name starting with 'e' followed by a signed integer")
			}
			
			#[inline(always)]
			fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E>
			{
				if !v.starts_with('e')
				{
					return Err(E::custom("should start with 'e'"))
				}
				match (&v[1..]).parse()
				{
					Ok(value) => Ok(EraName(value)),
					Err(parseError) => Err(E::custom(parseError))
				}
			}
		}
		
		deserializer.deserialize_str(EraNameVisitor)
	}
}

impl EraName
{
	pub const PreviousTwo: EraName = EraName(-2);
	
	pub const Previous: EraName = EraName(-1);
	
	pub const Current: EraName = EraName(0);
	
	pub const Next: EraName = EraName(1);
	
	pub const NextTwo: EraName = EraName(2);
	
	#[inline(always)]
	pub fn older(&self) -> Self
	{
		EraName(self.0 - 1)
	}
	
	#[inline(always)]
	pub fn description<'a>(&self, canIUse: &'a CanIUse) -> Option<&'a str>
	{
		canIUse.eras.description(self)
	}
	
	#[inline(always)]
	fn is_negative(&self) -> bool
	{
		self.0 < 0
	}
}
