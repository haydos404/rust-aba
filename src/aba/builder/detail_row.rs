use std::fmt::{Display, Error, Formatter};

pub enum TaxIndicator {
    None,
}

#[derive(PartialEq)]
pub enum TransactionCode {
    Credit = 50,
    Pay = 53,
    Debit = 13,
}

pub struct DetailRecord {
    /// Customer BSB Number
    pub customer_bsb: i32,
    /// Customer Account Number
    pub customer_account_number: i32,
    /// Customer last name initial with full first name
    /// 32 characters are valid
    /// example: J Peter
    pub customer_name: String,
    /// Transaction code that describes what kind of transaction this is
    pub transaction_code: TransactionCode,
    /// Amount to debit/credit the customer in cents
    pub amount_in_cents: u32,
    /// Reference for the customer that will show up in their records
    /// 18 characters are valid
    /// example: Woolworths Manuka
    pub payment_reference: String,
    /// Company BSB to allow retracing of entries to the source if necessary
    pub company_bsb: i32,
    /// Company account number to allow retracing of entries to the source if
    /// necessary
    pub company_account_number: i32,
    /// Name of the company or person remitting the entry
    /// 16 characters are valid
    pub company_name: String,
    /// Type of tax withheld
    pub tax_indicator: TaxIndicator,
    /// Amount in cents of tax withheld
    pub tax_amount_in_cents: u32,
}

impl Display for DetailRecord {
    /// Converts the detail record to a ABA description row
    ///
    /// The format of a detail row is as follows:
    /// 1(1): Record Type 1 - Must be 1
    /// 2-8(7): Customer BSB Number
    /// 9-17(9): Customer Account Number
    /// 18(1): Tax Indicator (N, W, X, Y, or an empty space)
    /// 19-20(2): Transaction Code. Typically 50 or 53
    /// 21-30(10): Amount to debit/credit in integer cents
    /// 31-62(32): Customer Name/Title
    /// 63-80(18): Payment Reference
    /// 81-87(7): Company BSB for trace reasons
    /// 88-96(9): Company Account number for trace reasons
    /// 97-112(16): Name of the Remitter/Company
    /// 113-120(8): Amount withheld for tax
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        return write!(
            f,
            "1\
             {customer_bsb:<3.3}-{customer_bsb:>3.3}\
             {customer_account:<9.9}\
             {tax_indicator:<1.1}\
             {transaction_code:<2.2}\
             {amount_in_cents:0>10.10}\
             {customer_name:<32.32}\
             {payment_reference:<18.18}\
             {company_bsb:<3.3}-{company_bsb:>3.3}\
             {company_account_number:<9.9}\
             {company_name:<16.16}\
             {tax_amount_in_cents:0>8}",
            customer_bsb = self.customer_bsb,
            customer_account = self.customer_account_number,
            tax_indicator = match self.tax_indicator {
                TaxIndicator::None => " ",
            },
            transaction_code = match self.transaction_code {
                TransactionCode::Debit => "13",
                TransactionCode::Credit => "50",
                TransactionCode::Pay => "53",
            },
            amount_in_cents = self.amount_in_cents,
            customer_name = self.customer_name,
            payment_reference = self.payment_reference,
            company_bsb = self.company_bsb,
            company_account_number = self.company_account_number,
            company_name = self.company_name,
            tax_amount_in_cents = self.tax_amount_in_cents
        );
    }
}
