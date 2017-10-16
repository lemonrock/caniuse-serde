// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Deserialize, Debug, Copy, Clone)]
pub struct UsagePercentage(f64);

impl<I: Into<f64>> From<I> for UsagePercentage
{
	#[inline(always)]
	fn from(value: I) -> Self
	{
		Self::clamp(value.into())
	}
}

impl UsagePercentage
{
	pub const Zero: Self = UsagePercentage(0.0);
	
	pub const Minimum: Self = UsagePercentage::Zero;
	
	pub const OneHundred: Self = UsagePercentage(100.0);
	
	pub const Maximum: Self = UsagePercentage::OneHundred;
	
	#[inline(always)]
	pub fn new(value: f64) -> Self
	{
		Self::clamp(value)
	}
	
	#[inline(always)]
	pub fn to_scalar(self) -> f64
	{
		self.0 / 100.0
	}
	
	#[inline(always)]
	fn clamp(value: f64) -> Self
	{
		if value.is_sign_negative() || value.is_nan()
		{
			UsagePercentage(0.0)
		}
		else if value.is_sign_positive() && (value > 100.0 || value.is_infinite())
		{
			UsagePercentage(100.0)
		}
		else
		{
			UsagePercentage(value)
		}
	}
}

impl PartialEq for UsagePercentage
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.0.eq(&other.0)
	}
}

impl Eq for UsagePercentage
{
}

impl PartialOrd for UsagePercentage
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		self.0.partial_cmp(&other.0)
	}
}

impl Ord for UsagePercentage
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.partial_cmp(other).unwrap_or(Ordering::Equal)
	}
}

impl Hash for UsagePercentage
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.to_bits().hash(state)
	}
}

impl Display for UsagePercentage
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut Formatter) -> fmt::Result
	{
		<f64 as Display>::fmt(&self.0, fmt)
	}
}

impl LowerExp for UsagePercentage
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<f64 as LowerExp>::fmt(&self.0, f)
	}
}

impl UpperExp for UsagePercentage
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<f64 as UpperExp>::fmt(&self.0, f)
	}
}

impl Default for UsagePercentage
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Zero
	}
}

impl Add<UsagePercentage> for UsagePercentage
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output
	{
		Self::clamp(self.0 + rhs.0)
	}
}

impl AddAssign<UsagePercentage> for UsagePercentage
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: Self)
	{
		*self = self.add(rhs)
	}
}

impl Sub<UsagePercentage> for UsagePercentage
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output
	{
		Self::clamp(self.0 - rhs.0)
	}
}

impl SubAssign<UsagePercentage> for UsagePercentage
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Self)
	{
		*self = self.sub(rhs)
	}
}

impl Deref for UsagePercentage
{
	type Target = f64;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}
