// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Deserialize, Debug, Clone)]
pub struct RegionalUsage
{
	id: String,
	name: String,
	//month: "2017-09",
	//access_date: "2017-10-05",
	data: HashMap<AgentName, BTreeMap<Version, Option<UsagePercentage>>>,
	total: UsagePercentage,
}

impl FromStr for RegionalUsage
{
	type Err = ::serde_json::error::Error;
	
	#[inline(always)]
	fn from_str(canIUseRegionalUsageDatabase: &str) -> Result<Self, Self::Err>
	{
		::serde_json::from_str(canIUseRegionalUsageDatabase)
	}
}

impl RegionalUsage
{
	/// Deserialize regional usage data from a file path to a `data.json` file.
	#[inline(always)]
	pub fn from_path<P: AsRef<Path>>(canIUseRegionalUsageDatabaseFilePath: P) -> Result<Self, Box<::std::error::Error>>
	{
		Self::from_reader(File::open(canIUseRegionalUsageDatabaseFilePath)?)
	}
	
	/// Deserialize regional usage data from a readable stream of raw JSON bytes.
	#[inline(always)]
	pub fn from_reader<R: Read>(readerOfStreamOfCanIUseJsonBytes: R) -> Result<Self, Box<::std::error::Error>>
	{
		Ok(serde_json::from_reader(readerOfStreamOfCanIUseJsonBytes)?)
	}
	
	/// Deserialize regional usage data from a slice of raw JSON bytes.
	#[inline(always)]
	pub fn from_slice(rawCanIUseJsonBytes: &[u8]) -> Result<Self, ::serde_json::error::Error>
	{
		Ok(serde_json::from_slice(rawCanIUseJsonBytes)?)
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
	
	/// Total usage; may not add up to 100% (eg for Andorra, adds up to about 95%).
	#[inline(always)]
	pub fn total(&self) -> UsagePercentage
	{
		self.total
	}
	
	/// Usage; returns None if agentName has no known usages.
	#[inline(always)]
	pub fn usage(&self, agentName: &AgentName, lowerBound: Bound<&Version>, upperBound: Bound<&Version>) -> Option<Range<Version, Option<UsagePercentage>>>
	{
		match self.data.get(agentName)
		{
			None => None,
			Some(entry) => Some(entry.range((lowerBound, upperBound)))
		}
	}
}

lazy_static!
{
	/// Embedded world-wide agent usage database.
	pub static ref WorldWide: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/alt-ww.json")).unwrap();
	
	/// Embedded continental agent usage database for Africa.
	pub static ref Africa: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/alt-af.json")).unwrap();
	
	/// Embedded continental agent usage database for Antarctica.
	pub static ref Antartica: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/alt-an.json")).unwrap();
	
	/// Embedded continental agent usage database for Asia.
	pub static ref Asia: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/alt-as.json")).unwrap();
	
	/// Embedded continental agent usage database for Europe.
	pub static ref Europe: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/alt-eu.json")).unwrap();
	
	/// Embedded continental agent usage database for North America.
	pub static ref NorthAmerica: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/alt-na.json")).unwrap();
	
	/// Embedded continental agent usage database for Oceania.
	pub static ref Oceania: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/alt-oc.json")).unwrap();
	
	/// Embedded continental agent usage database for South America.
	pub static ref SouthAmerica: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/alt-sa.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AD.
	pub static ref AD: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AD.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AE.
	pub static ref AE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AF.
	pub static ref AF: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AF.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AG.
	pub static ref AG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AI.
	pub static ref AI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AL.
	pub static ref AL: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AL.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AM.
	pub static ref AM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AN.
	pub static ref AN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AO.
	pub static ref AO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AR.
	pub static ref AR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AS.
	pub static ref AS: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AS.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AT.
	pub static ref AT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AU.
	pub static ref AU: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AU.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AW.
	pub static ref AW: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AW.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AX.
	pub static ref AX: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AX.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code AZ.
	pub static ref AZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/AZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BA.
	pub static ref BA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BB.
	pub static ref BB: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BB.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BD.
	pub static ref BD: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BD.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BE.
	pub static ref BE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BF.
	pub static ref BF: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BF.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BG.
	pub static ref BG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BH.
	pub static ref BH: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BH.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BI.
	pub static ref BI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BJ.
	pub static ref BJ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BJ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BM.
	pub static ref BM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BN.
	pub static ref BN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BO.
	pub static ref BO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BR.
	pub static ref BR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BS.
	pub static ref BS: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BS.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BT.
	pub static ref BT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BW.
	pub static ref BW: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BW.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BY.
	pub static ref BY: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BY.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code BZ.
	pub static ref BZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/BZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CA.
	pub static ref CA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CD.
	pub static ref CD: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CD.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CF.
	pub static ref CF: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CF.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CG.
	pub static ref CG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CH.
	pub static ref CH: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CH.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CI.
	pub static ref CI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CK.
	pub static ref CK: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CK.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CL.
	pub static ref CL: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CL.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CM.
	pub static ref CM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CN.
	pub static ref CN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CO.
	pub static ref CO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CR.
	pub static ref CR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CU.
	pub static ref CU: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CU.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CV.
	pub static ref CV: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CV.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CX.
	pub static ref CX: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CX.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CY.
	pub static ref CY: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CY.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code CZ.
	pub static ref CZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/CZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code DE.
	pub static ref DE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/DE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code DJ.
	pub static ref DJ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/DJ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code DK.
	pub static ref DK: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/DK.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code DM.
	pub static ref DM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/DM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code DO.
	pub static ref DO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/DO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code DZ.
	pub static ref DZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/DZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code EC.
	pub static ref EC: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/EC.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code EE.
	pub static ref EE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/EE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code EG.
	pub static ref EG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/EG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ER.
	pub static ref ER: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ER.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ES.
	pub static ref ES: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ES.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ET.
	pub static ref ET: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ET.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code FI.
	pub static ref FI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/FI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code FJ.
	pub static ref FJ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/FJ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code FK.
	pub static ref FK: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/FK.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code FM.
	pub static ref FM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/FM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code FO.
	pub static ref FO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/FO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code FR.
	pub static ref FR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/FR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GA.
	pub static ref GA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GB.
	pub static ref GB: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GB.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GD.
	pub static ref GD: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GD.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GE.
	pub static ref GE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GF.
	pub static ref GF: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GF.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GG.
	pub static ref GG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GH.
	pub static ref GH: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GH.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GI.
	pub static ref GI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GL.
	pub static ref GL: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GL.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GM.
	pub static ref GM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GN.
	pub static ref GN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GP.
	pub static ref GP: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GP.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GQ.
	pub static ref GQ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GQ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GR.
	pub static ref GR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GT.
	pub static ref GT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GU.
	pub static ref GU: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GU.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GW.
	pub static ref GW: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GW.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code GY.
	pub static ref GY: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/GY.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code HK.
	pub static ref HK: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/HK.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code HN.
	pub static ref HN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/HN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code HR.
	pub static ref HR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/HR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code HT.
	pub static ref HT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/HT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code HU.
	pub static ref HU: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/HU.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ID.
	pub static ref ID: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ID.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code IE.
	pub static ref IE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/IE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code IL.
	pub static ref IL: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/IL.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code IM.
	pub static ref IM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/IM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code IN.
	pub static ref IN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/IN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code IQ.
	pub static ref IQ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/IQ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code IR.
	pub static ref IR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/IR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code IS.
	pub static ref IS: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/IS.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code IT.
	pub static ref IT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/IT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code JE.
	pub static ref JE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/JE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code JM.
	pub static ref JM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/JM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code JO.
	pub static ref JO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/JO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code JP.
	pub static ref JP: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/JP.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KE.
	pub static ref KE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KG.
	pub static ref KG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KH.
	pub static ref KH: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KH.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KI.
	pub static ref KI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KM.
	pub static ref KM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KN.
	pub static ref KN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KP.
	pub static ref KP: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KP.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KR.
	pub static ref KR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KW.
	pub static ref KW: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KW.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KY.
	pub static ref KY: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KY.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code KZ.
	pub static ref KZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/KZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LA.
	pub static ref LA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LB.
	pub static ref LB: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LB.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LC.
	pub static ref LC: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LC.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LI.
	pub static ref LI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LK.
	pub static ref LK: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LK.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LR.
	pub static ref LR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LS.
	pub static ref LS: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LS.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LT.
	pub static ref LT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LU.
	pub static ref LU: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LU.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LV.
	pub static ref LV: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LV.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code LY.
	pub static ref LY: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/LY.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MA.
	pub static ref MA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MC.
	pub static ref MC: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MC.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MD.
	pub static ref MD: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MD.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ME.
	pub static ref ME: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ME.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MG.
	pub static ref MG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MH.
	pub static ref MH: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MH.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MK.
	pub static ref MK: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MK.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ML.
	pub static ref ML: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ML.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MM.
	pub static ref MM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MN.
	pub static ref MN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MO.
	pub static ref MO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MP.
	pub static ref MP: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MP.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MQ.
	pub static ref MQ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MQ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MR.
	pub static ref MR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MS.
	pub static ref MS: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MS.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MT.
	pub static ref MT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MU.
	pub static ref MU: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MU.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MV.
	pub static ref MV: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MV.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MW.
	pub static ref MW: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MW.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MX.
	pub static ref MX: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MX.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MY.
	pub static ref MY: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MY.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code MZ.
	pub static ref MZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/MZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NA.
	pub static ref NA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NC.
	pub static ref NC: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NC.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NE.
	pub static ref NE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NF.
	pub static ref NF: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NF.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NG.
	pub static ref NG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NI.
	pub static ref NI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NL.
	pub static ref NL: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NL.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NO.
	pub static ref NO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NP.
	pub static ref NP: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NP.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NR.
	pub static ref NR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NU.
	pub static ref NU: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NU.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code NZ.
	pub static ref NZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/NZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code OM.
	pub static ref OM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/OM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PA.
	pub static ref PA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PE.
	pub static ref PE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PF.
	pub static ref PF: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PF.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PG.
	pub static ref PG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PH.
	pub static ref PH: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PH.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PK.
	pub static ref PK: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PK.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PL.
	pub static ref PL: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PL.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PM.
	pub static ref PM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PN.
	pub static ref PN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PR.
	pub static ref PR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PS.
	pub static ref PS: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PS.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PT.
	pub static ref PT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PW.
	pub static ref PW: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PW.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code PY.
	pub static ref PY: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/PY.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code QA.
	pub static ref QA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/QA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code RE.
	pub static ref RE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/RE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code RO.
	pub static ref RO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/RO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code RS.
	pub static ref RS: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/RS.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code RU.
	pub static ref RU: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/RU.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code RW.
	pub static ref RW: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/RW.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SA.
	pub static ref SA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SB.
	pub static ref SB: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SB.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SC.
	pub static ref SC: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SC.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SD.
	pub static ref SD: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SD.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SE.
	pub static ref SE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SG.
	pub static ref SG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SH.
	pub static ref SH: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SH.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SI.
	pub static ref SI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SK.
	pub static ref SK: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SK.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SL.
	pub static ref SL: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SL.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SM.
	pub static ref SM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SN.
	pub static ref SN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SO.
	pub static ref SO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SR.
	pub static ref SR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ST.
	pub static ref ST: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ST.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SV.
	pub static ref SV: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SV.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SY.
	pub static ref SY: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SY.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code SZ.
	pub static ref SZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/SZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TC.
	pub static ref TC: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TC.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TD.
	pub static ref TD: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TD.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TG.
	pub static ref TG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TH.
	pub static ref TH: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TH.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TJ.
	pub static ref TJ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TJ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TK.
	pub static ref TK: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TK.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TL.
	pub static ref TL: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TL.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TM.
	pub static ref TM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TN.
	pub static ref TN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TO.
	pub static ref TO: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TO.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TR.
	pub static ref TR: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TR.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TT.
	pub static ref TT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TV.
	pub static ref TV: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TV.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TW.
	pub static ref TW: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TW.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code TZ.
	pub static ref TZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/TZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code UA.
	pub static ref UA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/UA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code UG.
	pub static ref UG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/UG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code US.
	pub static ref US: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/US.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code UY.
	pub static ref UY: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/UY.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code UZ.
	pub static ref UZ: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/UZ.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code VA.
	pub static ref VA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/VA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code VC.
	pub static ref VC: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/VC.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code VE.
	pub static ref VE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/VE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code VG.
	pub static ref VG: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/VG.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code VI.
	pub static ref VI: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/VI.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code VN.
	pub static ref VN: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/VN.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code VU.
	pub static ref VU: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/VU.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code WF.
	pub static ref WF: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/WF.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code WS.
	pub static ref WS: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/WS.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code YE.
	pub static ref YE: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/YE.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code YT.
	pub static ref YT: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/YT.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ZA.
	pub static ref ZA: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ZA.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ZM.
	pub static ref ZM: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ZM.json")).unwrap();

	/// Embedded regional agent usage database for ISO 3166 country code ZW.
	pub static ref ZW: RegionalUsage = RegionalUsage::from_str(include_str!("../region-usage-json/ZW.json")).unwrap();
}
