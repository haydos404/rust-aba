use std::fmt::{Display, Error, Formatter};

pub struct FooterRecord {
    pub net_total_in_cents: u32,
    pub credit_total_in_cents: u32,
    pub debit_total_in_cents: u32,
    pub len_detail_rows: usize,
}

impl Display for FooterRecord {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        return write!(
            f,
            "7\
             999-999\
             {front_blank:<12}\
             {net_total:0>10.10}\
             {credit_total:0>10.10}\
             {debit_total:0>10.10}\
             {middle_blank:<24}\
             {len_detail_rows:0>6.6}\
             {end_blank:<40}",
            front_blank = "",
            net_total = self.net_total_in_cents,
            credit_total = self.credit_total_in_cents,
            debit_total = self.debit_total_in_cents,
            middle_blank = "",
            len_detail_rows = self.len_detail_rows,
            end_blank = ""
        );
    }
}
