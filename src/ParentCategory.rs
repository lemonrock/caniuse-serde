// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ParentCategory
{
	CSS,
	HTML5,
	JS,
	JS_API,
	Other,
	Security,
	SVG,
	
	Unknown(String),
}

impl Default for ParentCategory
{
	#[inline(always)]
	fn default() -> Self
	{
		ParentCategory::Other
	}
}

impl<'de> Deserialize<'de> for ParentCategory
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct CategoryVisitor;
		
		impl<'de> Visitor<'de> for CategoryVisitor
		{
			type Value = ParentCategory;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("an era name starting with 'e' followed by a signed integer")
			}
			
			#[inline(always)]
			fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E>
			{
				use self::ParentCategory::*;
				
				let result = match v
				{
					"CSS" => CSS,
					"HTML5" => HTML5,
					"JS" => JS,
					"JS API" => JS_API,
					"Other" => Other,
					"Security" => Security,
					"SVG" => SVG,
					
					_ => Unknown(v.to_owned()),
				};
				Ok(result)
			}
			
			#[inline(always)]
			fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E>
			{
				use self::ParentCategory::*;
				
				let result = match &v[..]
				{
					"CSS" => CSS,
					"HTML5" => HTML5,
					"JS" => JS,
					"JS API" => JS_API,
					"Other" => Other,
					"Security" => Security,
					"SVG" => SVG,
					
					_ => Unknown(v),
				};
				Ok(result)
			}
		}
		
		deserializer.deserialize_str(CategoryVisitor)
	}
}

impl ParentCategory
{
	#[inline(always)]
	pub fn children<'a>(&self, canIUse: &'a CanIUse) -> Option<&'a [Category]>
	{
		canIUse.childCategories(self)
	}
}
