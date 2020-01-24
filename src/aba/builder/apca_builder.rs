use super::{footer_row::FooterRecord, DescriptiveRecord, DetailRecord, TransactionCode};
use std::fmt::{Display, Error, Formatter};

/// Builds ABA files
pub struct ApcaBuilder {
    /// Header file for the ABA transaction
    descriptive_record: DescriptiveRecord,
    /// Collection of rows outlining debits and credits to execute
    detail_records: Vec<DetailRecord>,
}

impl ApcaBuilder {
    #[must_use]
    pub fn new(options: DescriptiveRecord) -> Self {
        Self {
            descriptive_record: options,
            detail_records: vec![],
        }
    }

    /// Adds a new detail record to the aba file
    pub fn add_detail_record(&mut self, record: DetailRecord) -> &mut Self {
        self.detail_records.push(record);
        self
    }
}

impl Display for ApcaBuilder {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let credits = self
            .detail_records
            .iter()
            .filter(|x| {
                [TransactionCode::Credit, TransactionCode::Pay].contains(&x.transaction_code)
            })
            .fold(0, |acc, x| acc + x.amount_in_cents);

        let debits = self
            .detail_records
            .iter()
            .filter(|x| [TransactionCode::Debit].contains(&x.transaction_code))
            .fold(0, |acc, x| acc + x.amount_in_cents);

        let footer = FooterRecord {
            net_total_in_cents: credits - debits,
            credit_total_in_cents: credits,
            debit_total_in_cents: debits,
            len_detail_rows: self.detail_records.len(),
        };

        write!(
            f,
            "{:}\n{:}\n{:}",
            self.descriptive_record,
            self.detail_records
                .iter()
                .fold(String::default(), |acc, x| if acc.is_empty() {
                    x.to_string()
                } else {
                    format!("{:}\n{:}", acc, x)
                },),
            footer
        )
    }
}
