// enum comparisonOperators{
//   gt,
//   eq,
//   lt,
//   lte,
//   gte,
// };

#[derive(Debug)]
pub struct TradePairs {
    pub stream_name: String,
    pub comparison_operator: String,
    pub value: u32,
}

pub fn create_trade_pairs() -> Vec<TradePairs> {
    let mut trade_pair_array: Vec<TradePairs> = Vec::new();

    trade_pair_array.push(TradePairs {
        stream_name: String::from("btcusdt"),
        comparison_operator: String::from(">="),
        value: 40000,
    });
    trade_pair_array.push(TradePairs {
        stream_name: String::from("ethbtc"),
        comparison_operator: String::from("<="),
        value: 40000,
    });
    trade_pair_array.push(TradePairs {
        stream_name: String::from("etcbtc"),
        comparison_operator: String::from("<"),
        value: 40000,
    });
    trade_pair_array.push(TradePairs {
        stream_name: String::from("ethusdt"),
        comparison_operator: String::from("<="),
        value: 40000,
    });
    trade_pair_array.push(TradePairs {
        stream_name: String::from("omgbtc"),
        comparison_operator: String::from("<"),
        value: 40000,
    });

    trade_pair_array
}

// fn create_map(streamName: String, comparisonOperator: String, value: u32) -> Vec<tradePairs> {
//     let mut trade_pair_array: Vec<tradePairs> = Vec::new();

//     let mut some = tradePairs {
//         streamName,
//         comparisonOperator,
//         value,
//     };

//     trade_pair_array.push(some);

//     trade_pair_array
// }
