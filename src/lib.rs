use std::fmt::Display;

use dec_utils::dec_to_string_or_empty;
use rust_decimal::prelude::*;
//use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use serde_utc_time_ms::{de_string_to_utc_time_ms, se_time_ms_to_utc_z_string};
use taxbitrec::TaxBitRecType;
use time_ms_conversions::time_ms_to_utc_string;

#[derive(Debug, Deserialize, Serialize, Clone)]
// CSV Header
// Date,Transaction Type,Received Quantity,Received Currency,
// Sent Quantity,Sent Currency,Fee Currency,Fee Amount,
// Market Value,Source,Internal Transfer,External ID
pub struct TaxBitExportRec {
    #[serde(rename = "Date")]
    #[serde(deserialize_with = "de_string_to_utc_time_ms")]
    #[serde(serialize_with = "se_time_ms_to_utc_z_string")]
    pub time: i64,

    #[serde(rename = "Transaction Type")]
    pub type_txs: TaxBitRecType,

    #[serde(rename = "Received Quantity")]
    pub received_quantity: Option<Decimal>,

    #[serde(rename = "Received Currency")]
    pub received_currency: String,

    #[serde(rename = "Sent Quantity")]
    pub sent_quantity: Option<Decimal>,

    #[serde(rename = "Sent Currency")]
    pub sent_currency: String,

    #[serde(rename = "Fee Currency")]
    pub fee_currency: String,

    #[serde(rename = "Fee Amount")]
    pub fee_amount: Option<Decimal>,

    #[serde(rename = "Market Value")]
    pub market_value: Option<Decimal>,

    #[serde(rename = "Source")]
    pub source: String,

    #[serde(rename = "Internal Transfer")]
    pub internal_transfer: bool,

    #[serde(rename = "External ID")]
    pub external_id: String,
}

impl Display for TaxBitExportRec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{:?},{},{},{},{},{},{},{},{},{},{}",
            time_ms_to_utc_string(self.time),
            self.type_txs,
            dec_to_string_or_empty(self.sent_quantity),
            self.sent_currency,
            dec_to_string_or_empty(self.received_quantity),
            self.received_currency,
            self.fee_currency,
            dec_to_string_or_empty(self.fee_amount),
            dec_to_string_or_empty(self.market_value),
            self.source,
            self.internal_transfer,
            self.external_id,
        )
    }
}

impl TaxBitExportRec {
    pub fn new() -> TaxBitExportRec {
        TaxBitExportRec {
            time: 0i64,
            type_txs: TaxBitRecType::Unknown,
            received_quantity: None,
            received_currency: "".to_owned(),
            sent_quantity: None,
            sent_currency: "".to_owned(),
            fee_currency: "".to_owned(),
            fee_amount: None,
            market_value: None,
            source: "".to_owned(),
            internal_transfer: false,
            external_id: "".to_owned(),
        }
    }

    pub fn get_asset(&self) -> &str {
        match self.type_txs {
            TaxBitRecType::Expense
            | TaxBitRecType::TransferOut
            | TaxBitRecType::GiftSent
            | TaxBitRecType::Sale => self.sent_currency.as_str(),
            TaxBitRecType::Buy
            | TaxBitRecType::TransferIn
            | TaxBitRecType::Income
            | TaxBitRecType::GiftReceived
            | TaxBitRecType::Trade => self.received_currency.as_str(),
            TaxBitRecType::Unknown => panic!("SNH"),
        }
    }
}

impl Default for TaxBitExportRec {
    fn default() -> Self {
        Self::new()
    }
}

impl Eq for TaxBitExportRec {}

impl PartialEq for TaxBitExportRec {
    fn eq(&self, other: &Self) -> bool {
        println!("eq");
        self.time == other.time
            && self.type_txs == other.type_txs
            && self.received_currency == other.received_currency
            && self.sent_currency == other.sent_currency
            && self.fee_currency == other.fee_currency
            && self.received_quantity == other.received_quantity
            && self.sent_quantity == other.sent_quantity
            && self.fee_amount == other.fee_amount
            && self.market_value == other.market_value
            && self.source == other.source
            && self.internal_transfer == other.internal_transfer
            && self.external_id == other.external_id
    }
}

impl PartialOrd for TaxBitExportRec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        println!("partial_cmp");
        match self.time.partial_cmp(&other.time) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.type_txs.partial_cmp(&other.type_txs) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.received_currency.partial_cmp(&other.received_currency) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.sent_currency.partial_cmp(&other.sent_currency) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.fee_currency.partial_cmp(&other.fee_currency) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.received_quantity.partial_cmp(&other.received_quantity) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.sent_quantity.partial_cmp(&other.sent_quantity) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.fee_amount.partial_cmp(&other.fee_amount) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.market_value.partial_cmp(&other.market_value) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.source.partial_cmp(&other.source) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.internal_transfer.partial_cmp(&other.internal_transfer) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.external_id.partial_cmp(&other.external_id)
    }
}

impl Ord for TaxBitExportRec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            None => panic!("SNH"),
        }
    }
}

#[cfg(test)]
mod test {

    //use time_ms_conversions::{dt_str_to_utc_time_ms, TzMassaging};

    //use super::*;
    //use rust_decimal::prelude::*;
    use rust_decimal_macros::dec;

    use crate::{TaxBitExportRec, TaxBitRecType};

    #[test]
    fn test_new() {
        let tbr = TaxBitExportRec::new();
        assert_eq!(tbr.time, 0);
        assert_eq!(tbr.type_txs, TaxBitRecType::Unknown);
        assert_eq!(tbr.sent_quantity, None);
        assert_eq!(tbr.sent_currency, "".to_owned());
        assert_eq!(tbr.received_quantity, None);
        assert_eq!(tbr.received_currency, "".to_owned());
        assert_eq!(tbr.fee_amount, None);
        assert_eq!(tbr.fee_currency, "".to_owned());
        assert_eq!(tbr.market_value, None);
        assert_eq!(tbr.source, "".to_owned());
        assert_eq!(tbr.internal_transfer, false);
        assert_eq!(tbr.external_id, "".to_owned());
    }

    #[test]
    fn test_default() {
        let tbr_default = TaxBitExportRec::default();
        let tbr_new = TaxBitExportRec::new();
        assert_eq!(tbr_default, tbr_new);
    }

    #[test]
    fn test_eqne() {
        let mut tbr = TaxBitExportRec::default();
        let mut tbr_other = TaxBitExportRec::default();
        assert!(tbr == tbr_other);

        // The order is important so we go though all the paths,
        // so we modifiy the last test first
        tbr.external_id = "a".to_owned();
        assert!(tbr != tbr_other);

        tbr_other.internal_transfer = true;
        assert!(tbr != tbr_other);

        tbr.source = "a".to_owned();
        assert!(tbr != tbr_other);

        tbr.market_value = Some(dec!(0));
        tbr_other.market_value = Some(dec!(1));
        assert!(tbr != tbr_other);

        tbr.fee_amount = Some(dec!(0));
        tbr_other.fee_amount = Some(dec!(1));
        assert!(tbr != tbr_other);

        tbr.sent_quantity = Some(dec!(0));
        tbr_other.sent_quantity = Some(dec!(1));
        assert!(tbr != tbr_other);

        tbr.received_quantity = Some(dec!(0));
        tbr_other.received_quantity = Some(dec!(1));
        assert!(tbr != tbr_other);

        tbr.fee_currency = "a".to_owned();
        tbr_other.fee_currency = "b".to_owned();
        assert!(tbr != tbr_other);

        tbr.sent_currency = "a".to_owned();
        tbr_other.sent_currency = "b".to_owned();
        assert!(tbr != tbr_other);

        tbr.received_currency = "a".to_owned();
        tbr_other.received_currency = "b".to_owned();
        assert!(tbr != tbr_other);

        tbr.type_txs = TaxBitRecType::Expense;
        tbr_other.type_txs = TaxBitRecType::Buy;
        assert!(tbr != tbr_other);

        tbr.time = 0;
        tbr_other.time = 1;
        assert!(tbr != tbr_other);
    }

    #[test]
    fn test_partial_ord() {
        let mut tbr = TaxBitExportRec::default();
        let mut tbr_other = TaxBitExportRec::default();

        assert!(tbr <= tbr_other);

        // The order is important so we go though all the paths,
        // so we modifiy the last test first
        tbr.external_id = "a".to_owned();
        tbr_other.external_id = "b".to_owned();
        assert!(tbr < tbr_other);

        tbr_other.internal_transfer = true;
        assert!(tbr < tbr_other);

        tbr.source = "a".to_owned();
        tbr_other.source = "b".to_owned();
        assert!(tbr < tbr_other);

        tbr.market_value = Some(dec!(0));
        tbr_other.market_value = Some(dec!(1));
        assert!(tbr < tbr_other);

        tbr.fee_amount = Some(dec!(0));
        tbr_other.fee_amount = Some(dec!(1));
        assert!(tbr < tbr_other);

        tbr.sent_quantity = Some(dec!(0));
        tbr_other.sent_quantity = Some(dec!(1));
        assert!(tbr < tbr_other);

        tbr.received_quantity = Some(dec!(0));
        tbr_other.received_quantity = Some(dec!(1));
        assert!(tbr < tbr_other);

        tbr.fee_currency = "a".to_owned();
        tbr_other.fee_currency = "b".to_owned();
        assert!(tbr < tbr_other);

        tbr.sent_currency = "a".to_owned();
        tbr_other.sent_currency = "b".to_owned();
        assert!(tbr < tbr_other);

        tbr.received_currency = "a".to_owned();
        tbr_other.received_currency = "b".to_owned();
        assert!(tbr < tbr_other);

        tbr.type_txs = TaxBitRecType::Buy;
        tbr_other.type_txs = TaxBitRecType::Expense;
        assert!(tbr < tbr_other);

        tbr.time = 0;
        tbr_other.time = 1;
        assert!(tbr < tbr_other);
    }

    #[test]
    fn test_ord() {
        let tbr = TaxBitExportRec::default();
        let tbr_other = TaxBitExportRec::default();
        assert_eq!(tbr.cmp(&tbr_other), core::cmp::Ordering::Equal);
    }

    #[test]
    #[should_panic]
    fn test_ord_panic() {
        let mut tbr = TaxBitExportRec::default();
        let mut tbr_other = TaxBitExportRec::default();

        // Panic when a field is None and the same field in other is Some
        tbr.received_quantity = None;
        tbr_other.received_quantity = Some(dec!(1));
        assert_eq!(tbr.cmp(&tbr_other), core::cmp::Ordering::Equal);
    }

    #[test]
    #[should_panic]
    fn test_get_asset_panic() {
        let tbr = TaxBitExportRec::new();

        assert_eq!(tbr.type_txs, TaxBitRecType::Unknown);
        tbr.get_asset();
    }

    #[test]
    fn test_get_asset() {
        let mut tbr = TaxBitExportRec::new();

        tbr.type_txs = TaxBitRecType::Expense;
        tbr.sent_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TaxBitRecType::TransferOut;
        tbr.sent_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TaxBitRecType::GiftSent;
        tbr.sent_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TaxBitRecType::Sale;
        tbr.sent_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TaxBitRecType::Buy;
        tbr.received_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TaxBitRecType::TransferIn;
        tbr.received_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TaxBitRecType::Income;
        tbr.received_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TaxBitRecType::GiftReceived;
        tbr.received_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TaxBitRecType::Trade;
        tbr.received_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");
    }
}
