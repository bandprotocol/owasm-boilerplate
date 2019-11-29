use owasm::decl_data;
use owasm::x::{coingecko, cryptocompare};

decl_data! {
  pub struct Data {
    pub coin_gecko: f32 = coingecko::Price::new("bitcoin"),
    pub crypto_compare: f32 = cryptocompare::Price::new(),
  }
}

pub fn execute(data: Vec<Data>) -> Box<[u8]> {
  let mut total = 0.0;
  for each in &data {
    total += each.coin_gecko;
    total += each.crypto_compare;
  }
  let average = total / (data.len() as f32) / 2.0;
  let average_times_100 = (average * 100.0) as u64;
  Box::new(average_times_100.to_be_bytes())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_data() {
    let data = execute(vec![Data::build_from_local_env().unwrap()]);
    println!("output: {:?}", data);
    // println!("{:?}", Data::build_from_local_env());
  }
}
