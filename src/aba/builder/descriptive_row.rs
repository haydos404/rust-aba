use chrono;
use chrono::Utc;
use std::fmt::{Display, Error, Formatter};

/// Number issued by APCA authority
/// 57-62(6):  ID number issued by APCA
pub struct ApcaNumber(String);

impl ApcaNumber {
    pub fn new(number: &str) -> Result<Self, String> {
        if number.len() > 6 {
            return Err(format!(
                "APCA Number must have 6 characters. Found {:}",
                number.len()
            ));
        }

        Ok(Self(number.to_string()))
    }
}

impl Default for ApcaNumber {
    fn default() -> Self {
        Self::new("000000").unwrap()
    }
}

impl Clone for ApcaNumber {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Display for ApcaNumber {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

/// Reel Number
/// 2 digit numeric value starting at 1
/// Default behaviour sets this to 01
pub struct ReelSequenceNumber(i8);

impl ReelSequenceNumber {
    pub fn new(number: i8) -> Result<Self, String> {
        match number {
            number @ 1..=99 => Ok(Self(number)),
            _ => Err(
                "Invalid reel sequence number detected. Make sure it's between 1 and 99"
                    .to_string(),
            ),
        }
    }
}

impl Default for ReelSequenceNumber {
    fn default() -> Self {
        Self::new(1).unwrap()
    }
}

impl Clone for ReelSequenceNumber {
    fn clone(&self) -> Self {
        Self::new(self.0).unwrap()
    }
}

impl Display for ReelSequenceNumber {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:0>2.2}", self.0)
    }
}

pub struct DescriptiveRecord {
    /// 2 digit numeric value starting at 1
    /// Default behaviour sets this to 01
    pub reel_sequence_number: Option<ReelSequenceNumber>,

    /// Approved Financial Institution Abbreviation.
    /// 3 character string allowed only
    /// example: Bank of Queensland is BQL, Westpac is WBC
    pub financial_institution_code: String,

    /// Name of the company/entity performing this transaction
    /// 26 character string allowed only
    pub company_name: String,

    /// Number issued by APCA authority
    pub apca_number: Option<ApcaNumber>,

    /// Batch description
    pub description: String,

    /// Date to process the payments at
    pub process_at: chrono::Date<Utc>,
}

impl Display for DescriptiveRecord {
    /// Converts the descriptive record to a ABA description row
    ///
    /// The format of a descriptive row is as follows
    /// 1(1):      0 Always Zero
    /// 2-18(17):   Blank
    /// 19-20(2):  Reel Number
    /// 21-23(3):  Financial Institution Abbreviation
    /// 24-30(7):  Blank
    /// 31-56(26):  Company Name
    /// 57-62(6):  ID number issued by APCA
    /// 63-74(12):  Batch description
    /// 75-80(6):  Date to be processed in DDMMYY
    /// 81-120(40): Blank
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        // Ensure that the reel number is between 1 and 99
        let reel_sequence_number = self.reel_sequence_number.clone().unwrap_or_default();

        // Format the process date into DDMMYY or 2019-05-20 = 200519
        let process_at = self.process_at.format("%d%m%C");

        return write!(
            f,
            "0\
             {front_blank:<17}\
             {reel_sequence_number:0>2.2}\
             {financial_institution_code:<3.3}\
             {middle_blank:<7.7}\
             {company_name:<26.26}\
             {apca_number:0>6.6}\
             {description:<12.12}\
             {process_at:<6.6}\
             {end_blank:<40}",
            front_blank = "",
            reel_sequence_number = reel_sequence_number,
            financial_institution_code = self.financial_institution_code,
            middle_blank = "",
            company_name = self.company_name,
            apca_number = self.apca_number.clone().unwrap_or_default(),
            description = self.description,
            process_at = process_at,
            end_blank = ""
        );
    }
}
