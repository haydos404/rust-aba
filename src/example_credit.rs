use rust_aba::{ApcaBuilder, DescriptiveRecord, DetailRecord, TaxIndicator, TransactionCode};

fn main() {
    let mut builder = ApcaBuilder::new(DescriptiveRecord {
        reel_sequence_number: Option::None,
        financial_institution_code: String::from("CBA"),
        company_name: String::from("MY COMPANY"),
        apca_number: Option::None,
        description: String::from("CREDIT"),
        process_at: chrono::Utc::today(),
    });

    let builder = builder
        .add_detail_record(DetailRecord {
            customer_bsb: 333_123,
            customer_account_number: 193_122_400,
            customer_name: String::from("J Peter"),
            transaction_code: TransactionCode::Credit,
            amount_in_cents: 465,
            payment_reference: String::from("J Development 4935"),
            company_bsb: 162_050,
            company_account_number: 17_200_900,
            company_name: String::from("MY_COMPANY"),
            tax_indicator: TaxIndicator::None,
            tax_amount_in_cents: 0,
        })
        .add_detail_record(DetailRecord {
            customer_bsb: 938_324,
            customer_account_number: 300_219_600,
            customer_name: String::from("M Jonathon"),
            transaction_code: TransactionCode::Credit,
            amount_in_cents: 1200,
            payment_reference: String::from("J Development 4936"),
            company_bsb: 162_050,
            company_account_number: 17_200_900,
            company_name: String::from("MY_COMPANY"),
            tax_indicator: TaxIndicator::None,
            tax_amount_in_cents: 0,
        });

    println!("{:}", builder);
}
