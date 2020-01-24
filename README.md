# Rust ABA

![](https://github.com/haydos404/rust-aba/workflows/CI/badge.svg)

Generates an Australian Bank Payment File based on the standards set by the Australian Bankers Association.

For details regarding the standard, see https://www.cemtexaba.com/aba-format/cemtex-aba-file-format-details

## Usage

Create the `ApcaBuilder` with the appropriate options to describe your batch payment.

```rust
let mut builder = ApcaBuilder::new(DescriptiveRecord {
    reel_sequence_number: Option::None,
    financial_institution_code: String::from("CBA"),
    company_name: String::from("MY COMPANY"),
    apca_number: Option::None,
    description: String::from("CREDIT"),
    process_at: chrono::Utc::today(),
});
```

Next add the detail records for each transaction are to be processed.

```rust
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
```

Finally invoke `to_string` to generate the ABA file.

```rust
format!("{:}", builder)
```

To see an example output in terminal run `cargo run` or view the example source code specific here `src/example_credit.rs`.

## Example Output

The example provided above produces the following output to the terminal.

```
0                 01CBA       MY COMPANY                000000CREDIT      240120
1333123-333123193122400 500000000465J Peter                         J Development 4935162050-16205017200900 MY_COMPANY      00000000
7999-999            000000046500000004650000000000                        000001
```
