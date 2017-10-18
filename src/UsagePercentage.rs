// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// A simple 'newtype' wrapper that represents a percentage
#[derive(Deserialize, Debug, Copy, Clone)]
pub struct UsagePercentage(f64);

impl<I: Into<f64>> From<I> for UsagePercentage
{
	/// Converts from anything that can be represented as a f64 into a percentage.
	/// Clamps values below zero (including negative zero and negative infinity) to positive zero.
	/// Clamps NaN as positive zero.
	/// Clamps values above one hundred (including positive infinity) to one hundred.
	#[inline(always)]
	fn from(value: I) -> Self
	{
		Self::clamp(value.into())
	}
}

impl UsagePercentage
{
	/// Represents 0%
	pub const Zero: Self = UsagePercentage(0.0);
	
	/// Represents the minimum, 0%; interchangeable with UsagePercentage::Zero
	pub const Minimum: Self = UsagePercentage::Zero;
	
	/// Represents 100%
	pub const OneHundred: Self = UsagePercentage(100.0);
	
	/// Represents the maximum, 100%; interchangeable with UsagePercentage::OneHundred
	pub const Maximum: Self = UsagePercentage::OneHundred;
	
	/// Converts from anything that can be represented as a f64 into a percentage.
	/// Clamps values below zero (including negative zero and negative infinity) to positive zero.
	/// Clamps NaN as positive zero.
	/// Clamps values above one hundred (including positive infinity) to one hundred.
	#[inline(always)]
	pub fn new(value: f64) -> Self
	{
		Self::clamp(value)
	}
	
	/// Converts to a scalar, ie a percentage divided by 100
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
	/// Partial Equality; total equality is also supported
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
	/// Partial comparison
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		self.0.partial_cmp(&other.0)
	}
}

impl Ord for UsagePercentage
{
	/// Total comparison; always succeeds
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.partial_cmp(other).unwrap_or(Ordering::Equal)
	}
}

impl Hash for UsagePercentage
{
	/// Hash
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.to_bits().hash(state)
	}
}

impl Display for UsagePercentage
{
	/// Displays as a floating point value followed by a '%'
	#[inline(always)]
	fn fmt(&self, fmt: &mut Formatter) -> fmt::Result
	{
		write!(fmt, "{}%", self.0)
	}
}

impl Default for UsagePercentage
{
	/// Defaults to zero
	#[inline(always)]
	fn default() -> Self
	{
		Self::Zero
	}
}

impl Add<UsagePercentage> for UsagePercentage
{
	type Output = Self;
	
	/// Add
	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output
	{
		Self::clamp(self.0 + rhs.0)
	}
}

impl AddAssign<UsagePercentage> for UsagePercentage
{
	/// Add in place
	#[inline(always)]
	fn add_assign(&mut self, rhs: Self)
	{
		*self = self.add(rhs)
	}
}

impl Sub<UsagePercentage> for UsagePercentage
{
	type Output = Self;
	
	/// Subtract
	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output
	{
		Self::clamp(self.0 - rhs.0)
	}
}

impl SubAssign<UsagePercentage> for UsagePercentage
{
	/// Subtract in place
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Self)
	{
		*self = self.sub(rhs)
	}
}

impl Deref for UsagePercentage
{
	type Target = f64;
	
	/// Dereferences to f64
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}
