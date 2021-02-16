use crate::account::*;
use crate::market::*;
use crate::general::*;
use crate::futures::general::*;
use crate::futures::market::*;
use crate::userstream::*;
use crate::client::*;

static API_HOST: &str = "https://api.binance.com";
static API_TEST_HOST: &str = "https://testnet.binance.vision";
static FAPI_HOST: &str = "https://fapi.binance.com";

pub enum MarketType {
    Live,
    Test
}

//#[derive(Clone)]
pub trait Binance {
    fn new(api_key: Option<String>, secret_key: Option<String>, market_type: MarketType) -> Self;
}

impl Binance for General {
    fn new(api_key: Option<String>, secret_key: Option<String>, market_type: MarketType) -> General {
        let api_host: &str = match market_type{
            MarketType::Live => API_HOST,
            MarketType::Test => API_TEST_HOST,
        };
        General {
            client: Client::new(api_key, secret_key, api_host.to_string()),
        }
    }
}

impl Binance for Account {
    fn new(api_key: Option<String>, secret_key: Option<String>, market_type: MarketType) -> Account {
        let api_host: &str = match market_type{
            MarketType::Live => API_HOST,
            MarketType::Test => API_TEST_HOST,
        };
        Account {
            client: Client::new(api_key, secret_key, api_host.to_string()),
            recv_window: 5000,
        }
    }
}

impl Binance for Market {
    fn new(api_key: Option<String>, secret_key: Option<String>, market_type: MarketType) -> Market {
        let api_host: &str = match market_type{
            MarketType::Live => API_HOST,
            MarketType::Test => API_TEST_HOST,
        };
        Market {
            client: Client::new(api_key, secret_key, api_host.to_string()),
            recv_window: 5000,
        }
    }
}

impl Binance for UserStream {
    fn new(api_key: Option<String>, secret_key: Option<String>, market_type: MarketType) -> UserStream {
        let api_host: &str = match market_type{
            MarketType::Live => API_HOST,
            MarketType::Test => API_TEST_HOST,
        };
        UserStream {
            client: Client::new(api_key, secret_key, api_host.to_string()),
            recv_window: 5000,
        }
    }
}

// *****************************************************
//              Binance Futures API
// *****************************************************

impl Binance for FuturesGeneral {
    fn new(api_key: Option<String>, secret_key: Option<String>, _market_type: MarketType) -> FuturesGeneral {
        FuturesGeneral {
            client: Client::new(api_key, secret_key, FAPI_HOST.to_string()),
        }
    }
}

impl Binance for FuturesMarket {
    fn new(api_key: Option<String>, secret_key: Option<String>, _market_type: MarketType) -> FuturesMarket {
        FuturesMarket {
            client: Client::new(api_key, secret_key, FAPI_HOST.to_string()),
            recv_window: 5000,
        }
    }
}
