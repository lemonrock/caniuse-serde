// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// A structure representing regional, continental or world-wide usage of an agent (browser) by version.
#[derive(Deserialize, Debug, Clone)]
pub struct RegionalUsage
{
	id: String,
	name: String,
	month: YearMonth,
	access_date: NaiveDate,
	data: HashMap<AgentName, BTreeMap<Version, Option<UsagePercentage>>>,
	total: UsagePercentage,
}

impl Default for RegionalUsage
{
	// Defaults to World-Wide usage
	#[inline(always)]
	fn default() -> Self
	{
		RegionalUsage::from_str(include_str!("../region-usage-json/alt-ww.json")).unwrap()
	}
}

impl FromStr for RegionalUsage
{
	type Err = ::serde_json::error::Error;
	
	#[inline(always)]
	fn from_str(regional_usage_database_json: &str) -> Result<Self, Self::Err>
	{
		::serde_json::from_str(regional_usage_database_json)
	}
}

impl RegionalUsage
{
	/// Deserialize regional usage data from a file path to a `data.json` file.
	#[inline(always)]
	pub fn from_path<P: AsRef<Path>>(regional_usage_database_file_path: P) -> Result<Self, Box<::std::error::Error>>
	{
		Self::from_reader(File::open(regional_usage_database_file_path)?)
	}
	
	/// Deserialize regional usage data from a readable stream of raw JSON bytes.
	#[inline(always)]
	pub fn from_reader<R: Read>(reader_of_stream_of_regional_usage_database_json_bytes: R) -> Result<Self, Box<::std::error::Error>>
	{
		Ok(serde_json::from_reader(reader_of_stream_of_regional_usage_database_json_bytes)?)
	}
	
	/// Deserialize regional usage data from a slice of raw JSON bytes.
	#[inline(always)]
	pub fn from_slice(regional_usage_database_json_byte: &[u8]) -> Result<Self, ::serde_json::error::Error>
	{
		Ok(serde_json::from_slice(regional_usage_database_json_byte)?)
	}
	
	/// ISO-like code.
	#[inline(always)]
	pub fn identifier(&self) -> &str
	{
		&self.id
	}
	
	/// Country name or similar.
	#[inline(always)]
	pub fn country_or_region_name(&self) -> &str
	{
		&self.name
	}
	
	/// Total usage; may not add up to 100% (eg for Andorra, AD, adds up to about 95%).
	#[inline(always)]
	pub fn total(&self) -> UsagePercentage
	{
		self.total
	}
	
	/// Usage; returns None if agent_name has no known usages.
	#[inline(always)]
	pub fn usage<'a>(&'a self, agent_name: &AgentName, lower_bound: Bound<&Version>, upper_bound: Bound<&Version>) -> Option<Range<'a, Version, Option<UsagePercentage>>>
	{
		match self.data.get(agent_name)
		{
			None => None,
			Some(entry) => Some(entry.range((lower_bound, upper_bound)))
		}
	}
	
	/// Usage for a specific version; returns None if agent_name does not exist
	/// Returns Some(None) if agent_name has no entry for version
	/// Returns Some(Some(None)) if agent_name has an entry, but the data in the caniuse.com regional database is 'null'
	/// Returns Some(Some(&Some(usage_percentage)) if agent_name has an entry with valid data
	#[inline(always)]
	pub fn usage_of_version<'a>(&'a self, agent_name: &AgentName, version: &Version) -> Option<Option<&'a Option<UsagePercentage>>>
	{
		match self.data.get(agent_name)
		{
			None => None,
			Some(entry) => Some(entry.get(version))
		}
	}
	
	/// Find matching values.
	/// For example the closure `|_agentName, _version, usagePercentage, _agentType| usagePercentage > UsagePercentage::new(0.05)` will match all versions of all user agents which are used by more than 5% of this region's users.
	/// Internal values with an invalid or unknown version, or a missing usage percentage (ie bad data), are never matched.
	/// A HashSet is returned to allow combination of multiple queries as unions (ie 'or').
	#[inline(always)]
	pub fn query<'a, Matcher: Fn(&'a AgentName, &'a Version, UsagePercentage, AgentType) -> bool>(&'a self, can_i_use: &'a CanIUse, matcher: Matcher) -> HashSet<(&'a AgentName, &'a Version)>
	{
		let mut result = HashSet::new();
		for agentName in can_i_use.known_agent_names()
		{
			let agent = can_i_use.agent(agentName).expect("is from known_agent_names() so must exist");
			let agent_type = agent.agent_type();
			
			if let Some(entry) = self.data.get(agentName)
			{
				for (version, optionalUsagePercentage) in entry.iter()
				{
					if !version.is_invalid_or_unknown()
					{
						if let &Some(versionUsagePercentage) = optionalUsagePercentage
						{
							if matcher(agentName, version, versionUsagePercentage, agent_type)
							{
								result.insert((agentName, version));
							}
						}
					}
				}
			}
		}
		result
	}
}

lazy_static!
{
	/// Embedded world-wide agent usage database.
	#[derive(Debug)] static ref WorldWide: RegionalUsage = RegionalUsage::default();

	/// Embedded continental agent usage database for Africa.
	#[derive(Debug)] static ref Africa: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/alt-af.json")).unwrap();

	/// Embedded continental agent usage database for Antarctica.
	#[derive(Debug)] static ref Antarctica: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/alt-an.json")).unwrap();

	/// Embedded continental agent usage database for Asia.
	#[derive(Debug)] static ref Asia: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/alt-as.json")).unwrap();

	/// Embedded continental agent usage database for Europe.
	#[derive(Debug)] static ref Europe: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/alt-eu.json")).unwrap();

	/// Embedded continental agent usage database for North America.
	#[derive(Debug)] static ref NorthAmerica: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/alt-na.json")).unwrap();

	/// Embedded continental agent usage database for Oceania.
	#[derive(Debug)] static ref Oceania: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/alt-oc.json")).unwrap();

	/// Embedded continental agent usage database for South America.
	#[derive(Debug)] static ref SouthAmerica: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/alt-sa.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AD.
	#[derive(Debug)] static ref AD: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AD.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AE.
	#[derive(Debug)] static ref AE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AF.
	#[derive(Debug)] static ref AF: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AF.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AG.
	#[derive(Debug)] static ref AG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AI.
	#[derive(Debug)] static ref AI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AL.
	#[derive(Debug)] static ref AL: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AL.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AM.
	#[derive(Debug)] static ref AM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AN.
	#[derive(Debug)] static ref AN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AO.
	#[derive(Debug)] static ref AO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AR.
	#[derive(Debug)] static ref AR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AS.
	#[derive(Debug)] static ref AS: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AS.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AT.
	#[derive(Debug)] static ref AT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AU.
	#[derive(Debug)] static ref AU: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AU.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AW.
	#[derive(Debug)] static ref AW: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AW.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AX.
	#[derive(Debug)] static ref AX: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AX.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AZ.
	#[derive(Debug)] static ref AZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BA.
	#[derive(Debug)] static ref BA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BB.
	#[derive(Debug)] static ref BB: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BB.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BD.
	#[derive(Debug)] static ref BD: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BD.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BE.
	#[derive(Debug)] static ref BE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BF.
	#[derive(Debug)] static ref BF: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BF.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BG.
	#[derive(Debug)] static ref BG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BH.
	#[derive(Debug)] static ref BH: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BH.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BI.
	#[derive(Debug)] static ref BI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BJ.
	#[derive(Debug)] static ref BJ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BJ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BM.
	#[derive(Debug)] static ref BM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BN.
	#[derive(Debug)] static ref BN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BO.
	#[derive(Debug)] static ref BO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BR.
	#[derive(Debug)] static ref BR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BS.
	#[derive(Debug)] static ref BS: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BS.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BT.
	#[derive(Debug)] static ref BT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BW.
	#[derive(Debug)] static ref BW: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BW.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BY.
	#[derive(Debug)] static ref BY: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BY.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BZ.
	#[derive(Debug)] static ref BZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CA.
	#[derive(Debug)] static ref CA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CD.
	#[derive(Debug)] static ref CD: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CD.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CF.
	#[derive(Debug)] static ref CF: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CF.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CG.
	#[derive(Debug)] static ref CG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CH.
	#[derive(Debug)] static ref CH: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CH.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CI.
	#[derive(Debug)] static ref CI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CK.
	#[derive(Debug)] static ref CK: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CK.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CL.
	#[derive(Debug)] static ref CL: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CL.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CM.
	#[derive(Debug)] static ref CM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CN.
	#[derive(Debug)] static ref CN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CO.
	#[derive(Debug)] static ref CO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CR.
	#[derive(Debug)] static ref CR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CU.
	#[derive(Debug)] static ref CU: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CU.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CV.
	#[derive(Debug)] static ref CV: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CV.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CX.
	#[derive(Debug)] static ref CX: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CX.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CY.
	#[derive(Debug)] static ref CY: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CY.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CZ.
	#[derive(Debug)] static ref CZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code DE.
	#[derive(Debug)] static ref DE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/DE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code DJ.
	#[derive(Debug)] static ref DJ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/DJ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code DK.
	#[derive(Debug)] static ref DK: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/DK.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code DM.
	#[derive(Debug)] static ref DM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/DM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code DO.
	#[derive(Debug)] static ref DO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/DO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code DZ.
	#[derive(Debug)] static ref DZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/DZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code EC.
	#[derive(Debug)] static ref EC: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/EC.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code EE.
	#[derive(Debug)] static ref EE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/EE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code EG.
	#[derive(Debug)] static ref EG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/EG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ER.
	#[derive(Debug)] static ref ER: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ER.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ES.
	#[derive(Debug)] static ref ES: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ES.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ET.
	#[derive(Debug)] static ref ET: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ET.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code FI.
	#[derive(Debug)] static ref FI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/FI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code FJ.
	#[derive(Debug)] static ref FJ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/FJ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code FK.
	#[derive(Debug)] static ref FK: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/FK.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code FM.
	#[derive(Debug)] static ref FM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/FM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code FO.
	#[derive(Debug)] static ref FO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/FO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code FR.
	#[derive(Debug)] static ref FR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/FR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GA.
	#[derive(Debug)] static ref GA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GB.
	#[derive(Debug)] static ref GB: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GB.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GD.
	#[derive(Debug)] static ref GD: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GD.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GE.
	#[derive(Debug)] static ref GE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GF.
	#[derive(Debug)] static ref GF: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GF.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GG.
	#[derive(Debug)] static ref GG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GH.
	#[derive(Debug)] static ref GH: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GH.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GI.
	#[derive(Debug)] static ref GI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GL.
	#[derive(Debug)] static ref GL: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GL.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GM.
	#[derive(Debug)] static ref GM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GN.
	#[derive(Debug)] static ref GN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GP.
	#[derive(Debug)] static ref GP: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GP.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GQ.
	#[derive(Debug)] static ref GQ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GQ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GR.
	#[derive(Debug)] static ref GR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GT.
	#[derive(Debug)] static ref GT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GU.
	#[derive(Debug)] static ref GU: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GU.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GW.
	#[derive(Debug)] static ref GW: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GW.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GY.
	#[derive(Debug)] static ref GY: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GY.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code HK.
	#[derive(Debug)] static ref HK: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/HK.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code HN.
	#[derive(Debug)] static ref HN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/HN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code HR.
	#[derive(Debug)] static ref HR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/HR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code HT.
	#[derive(Debug)] static ref HT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/HT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code HU.
	#[derive(Debug)] static ref HU: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/HU.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ID.
	#[derive(Debug)] static ref ID: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ID.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code IE.
	#[derive(Debug)] static ref IE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/IE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code IL.
	#[derive(Debug)] static ref IL: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/IL.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code IM.
	#[derive(Debug)] static ref IM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/IM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code IN.
	#[derive(Debug)] static ref IN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/IN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code IQ.
	#[derive(Debug)] static ref IQ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/IQ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code IR.
	#[derive(Debug)] static ref IR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/IR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code IS.
	#[derive(Debug)] static ref IS: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/IS.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code IT.
	#[derive(Debug)] static ref IT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/IT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code JE.
	#[derive(Debug)] static ref JE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/JE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code JM.
	#[derive(Debug)] static ref JM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/JM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code JO.
	#[derive(Debug)] static ref JO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/JO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code JP.
	#[derive(Debug)] static ref JP: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/JP.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KE.
	#[derive(Debug)] static ref KE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KG.
	#[derive(Debug)] static ref KG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KH.
	#[derive(Debug)] static ref KH: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KH.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KI.
	#[derive(Debug)] static ref KI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KM.
	#[derive(Debug)] static ref KM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KN.
	#[derive(Debug)] static ref KN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KP.
	#[derive(Debug)] static ref KP: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KP.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KR.
	#[derive(Debug)] static ref KR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KW.
	#[derive(Debug)] static ref KW: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KW.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KY.
	#[derive(Debug)] static ref KY: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KY.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KZ.
	#[derive(Debug)] static ref KZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LA.
	#[derive(Debug)] static ref LA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LB.
	#[derive(Debug)] static ref LB: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LB.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LC.
	#[derive(Debug)] static ref LC: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LC.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LI.
	#[derive(Debug)] static ref LI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LK.
	#[derive(Debug)] static ref LK: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LK.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LR.
	#[derive(Debug)] static ref LR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LS.
	#[derive(Debug)] static ref LS: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LS.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LT.
	#[derive(Debug)] static ref LT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LU.
	#[derive(Debug)] static ref LU: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LU.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LV.
	#[derive(Debug)] static ref LV: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LV.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LY.
	#[derive(Debug)] static ref LY: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LY.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MA.
	#[derive(Debug)] static ref MA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MC.
	#[derive(Debug)] static ref MC: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MC.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MD.
	#[derive(Debug)] static ref MD: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MD.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ME.
	#[derive(Debug)] static ref ME: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ME.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MG.
	#[derive(Debug)] static ref MG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MH.
	#[derive(Debug)] static ref MH: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MH.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MK.
	#[derive(Debug)] static ref MK: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MK.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ML.
	#[derive(Debug)] static ref ML: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ML.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MM.
	#[derive(Debug)] static ref MM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MN.
	#[derive(Debug)] static ref MN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MO.
	#[derive(Debug)] static ref MO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MP.
	#[derive(Debug)] static ref MP: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MP.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MQ.
	#[derive(Debug)] static ref MQ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MQ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MR.
	#[derive(Debug)] static ref MR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MS.
	#[derive(Debug)] static ref MS: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MS.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MT.
	#[derive(Debug)] static ref MT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MU.
	#[derive(Debug)] static ref MU: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MU.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MV.
	#[derive(Debug)] static ref MV: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MV.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MW.
	#[derive(Debug)] static ref MW: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MW.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MX.
	#[derive(Debug)] static ref MX: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MX.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MY.
	#[derive(Debug)] static ref MY: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MY.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MZ.
	#[derive(Debug)] static ref MZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NA.
	#[derive(Debug)] static ref NA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NC.
	#[derive(Debug)] static ref NC: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NC.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NE.
	#[derive(Debug)] static ref NE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NF.
	#[derive(Debug)] static ref NF: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NF.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NG.
	#[derive(Debug)] static ref NG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NI.
	#[derive(Debug)] static ref NI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NL.
	#[derive(Debug)] static ref NL: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NL.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NO.
	#[derive(Debug)] static ref NO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NP.
	#[derive(Debug)] static ref NP: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NP.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NR.
	#[derive(Debug)] static ref NR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NU.
	#[derive(Debug)] static ref NU: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NU.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NZ.
	#[derive(Debug)] static ref NZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code OM.
	#[derive(Debug)] static ref OM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/OM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PA.
	#[derive(Debug)] static ref PA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PE.
	#[derive(Debug)] static ref PE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PF.
	#[derive(Debug)] static ref PF: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PF.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PG.
	#[derive(Debug)] static ref PG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PH.
	#[derive(Debug)] static ref PH: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PH.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PK.
	#[derive(Debug)] static ref PK: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PK.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PL.
	#[derive(Debug)] static ref PL: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PL.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PM.
	#[derive(Debug)] static ref PM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PN.
	#[derive(Debug)] static ref PN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PR.
	#[derive(Debug)] static ref PR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PS.
	#[derive(Debug)] static ref PS: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PS.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PT.
	#[derive(Debug)] static ref PT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PW.
	#[derive(Debug)] static ref PW: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PW.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PY.
	#[derive(Debug)] static ref PY: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PY.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code QA.
	#[derive(Debug)] static ref QA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/QA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code RE.
	#[derive(Debug)] static ref RE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/RE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code RO.
	#[derive(Debug)] static ref RO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/RO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code RS.
	#[derive(Debug)] static ref RS: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/RS.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code RU.
	#[derive(Debug)] static ref RU: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/RU.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code RW.
	#[derive(Debug)] static ref RW: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/RW.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SA.
	#[derive(Debug)] static ref SA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SB.
	#[derive(Debug)] static ref SB: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SB.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SC.
	#[derive(Debug)] static ref SC: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SC.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SD.
	#[derive(Debug)] static ref SD: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SD.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SE.
	#[derive(Debug)] static ref SE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SG.
	#[derive(Debug)] static ref SG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SH.
	#[derive(Debug)] static ref SH: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SH.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SI.
	#[derive(Debug)] static ref SI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SK.
	#[derive(Debug)] static ref SK: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SK.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SL.
	#[derive(Debug)] static ref SL: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SL.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SM.
	#[derive(Debug)] static ref SM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SN.
	#[derive(Debug)] static ref SN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SO.
	#[derive(Debug)] static ref SO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SR.
	#[derive(Debug)] static ref SR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ST.
	#[derive(Debug)] static ref ST: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ST.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SV.
	#[derive(Debug)] static ref SV: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SV.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SY.
	#[derive(Debug)] static ref SY: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SY.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SZ.
	#[derive(Debug)] static ref SZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TC.
	#[derive(Debug)] static ref TC: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TC.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TD.
	#[derive(Debug)] static ref TD: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TD.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TG.
	#[derive(Debug)] static ref TG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TH.
	#[derive(Debug)] static ref TH: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TH.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TJ.
	#[derive(Debug)] static ref TJ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TJ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TK.
	#[derive(Debug)] static ref TK: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TK.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TL.
	#[derive(Debug)] static ref TL: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TL.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TM.
	#[derive(Debug)] static ref TM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TN.
	#[derive(Debug)] static ref TN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TO.
	#[derive(Debug)] static ref TO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TR.
	#[derive(Debug)] static ref TR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TT.
	#[derive(Debug)] static ref TT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TV.
	#[derive(Debug)] static ref TV: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TV.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TW.
	#[derive(Debug)] static ref TW: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TW.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TZ.
	#[derive(Debug)] static ref TZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code UA.
	#[derive(Debug)] static ref UA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/UA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code UG.
	#[derive(Debug)] static ref UG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/UG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code US.
	#[derive(Debug)] static ref US: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/US.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code UY.
	#[derive(Debug)] static ref UY: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/UY.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code UZ.
	#[derive(Debug)] static ref UZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/UZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code VA.
	#[derive(Debug)] static ref VA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/VA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code VC.
	#[derive(Debug)] static ref VC: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/VC.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code VE.
	#[derive(Debug)] static ref VE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/VE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code VG.
	#[derive(Debug)] static ref VG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/VG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code VI.
	#[derive(Debug)] static ref VI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/VI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code VN.
	#[derive(Debug)] static ref VN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/VN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code VU.
	#[derive(Debug)] static ref VU: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/VU.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code WF.
	#[derive(Debug)] static ref WF: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/WF.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code WS.
	#[derive(Debug)] static ref WS: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/WS.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code YE.
	#[derive(Debug)] static ref YE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/YE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code YT.
	#[derive(Debug)] static ref YT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/YT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ZA.
	#[derive(Debug)] static ref ZA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ZA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ZM.
	#[derive(Debug)] static ref ZM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ZM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ZW.
	#[derive(Debug)] static ref ZW: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ZW.json")).unwrap();
}
