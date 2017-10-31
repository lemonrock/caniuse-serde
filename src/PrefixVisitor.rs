// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


struct PrefixVisitor;

impl<'de> Visitor<'de> for PrefixVisitor
{
	type Value = Option<Prefix>;
	
	fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
	{
		formatter.write_str("a vendor prefix without leading or trailing hyphens")
	}
	
	fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E>
	{
		use self::Prefix::*;
		
		let result = match v
		{
			"" => None,
			
			"o" => Some(o),
			
			"moz" => Some(moz),
			
			"webkit" => Some(webkit),
			
			"ms" => Some(ms),
			
			_ => Some(Unknown(v.to_owned())),
		};
		
		Ok(result)
	}
	
	fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E>
	{
		use self::Prefix::*;
		
		let result = match &v[..]
		{
			"" => None,
			
			"o" => Some(o),
			
			"moz" => Some(moz),
			
			"webkit" => Some(webkit),
			
			"ms" => Some(ms),
			
			_ => Some(Unknown(v.to_owned())),
		};
		
		Ok(result)
	}
}
