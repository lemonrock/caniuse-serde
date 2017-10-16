// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct Eras(LinkedHashMap<EraName, (usize, String)>);

impl<'de> Deserialize<'de> for Eras
{
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct ErasVisitor;
		
		impl<'de> Visitor<'de> for ErasVisitor
		{
			type Value = Eras;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("an eras ordered map of name to description")
			}
			
			#[inline(always)]
			fn visit_map<M: MapAccess<'de>>(self, mut access: M) -> Result<Self::Value, M::Error>
			{
				let mut map = LinkedHashMap::with_capacity(access.size_hint().unwrap_or(16));
				
				let mut index = 0;
				while let Some((key, value)) = access.next_entry()?
				{
					map.insert(key, (index, value));
					index += 1;
				}
				
				Ok(Eras(map))
			}
		}
		
		deserializer.deserialize_map(ErasVisitor)
	}
}

impl Eras
{
	#[inline(always)]
	fn index(&self, eraName: &EraName) -> Option<usize>
	{
		self.0.get(eraName).map(|era| era.0)
	}
	
	#[inline(always)]
	fn description(&self, eraName: &EraName) -> Option<&str>
	{
		self.0.get(eraName).map(|era| era.1.as_str())
	}
}
