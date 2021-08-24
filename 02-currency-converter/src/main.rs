#[macro_use]
extern crate lazy_static;

#[derive(Clone, Debug)]
/// A currency, as a ratio of the almighty US dollar.
struct Currency {
    /// Name of currency
    name: String,
    shortcode: String,
    /// Ratio of USD.
    ratio_of_usd: f32,
}

impl Currency {
    fn new(shortcode: &str, name: &str, ratio_of_usd: f32) -> Self {
        Currency {
            shortcode: shortcode.to_string(),
            name: name.to_string(),
            ratio_of_usd,
        }
    }

    /// Return the currency as USD
    fn convert_to_usd(&self, currency_amount: f32) -> f32 {
        self.ratio_of_usd * currency_amount
    }

    /// Convert one currency into another
    fn convert_to_currency(&self, currency_amount: f32, currency: &Currency) -> f32 {
        let as_usd = self.convert_to_usd(currency_amount);
        as_usd / currency.ratio_of_usd
    }
}

lazy_static! {
    static ref USD: Currency = Currency::new("USD", "US Dollar", 1.0);
    static ref AUD: Currency = Currency::new("AUD", "Australian Dollar", 0.72);
    static ref JPY: Currency = Currency::new("JPY", "Japanese Yen", 0.0091);
}

fn choose_currency(message: &str) -> &Currency {
    let result: &Currency;
    let currencies: [&Currency; 3] = [&USD, &AUD, &JPY];

    let options: String = currencies
        .iter()
        .map(|x| format!("{} [{}]", x.name, x.shortcode))
        .collect::<Vec<String>>()
        .join(", ");

    loop {
        println!("{}: {}", message, options);

        let mut buffer = String::new();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Expected a value");

        let value = buffer.trim();

        match currencies
            .iter()
            .find(|x| x.shortcode.to_lowercase() == value.to_lowercase())
        {
            Some(val) => {
                result = val;
                break;
            }
            None => {
                continue;
            }
        }
    }

    println!();

    result
}

fn get_number_from_buffer() -> f32 {
    let result: f32;

    loop {
        println!("How much do you want to convert?");

        let mut buffer = String::new();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Expected a value");

        match buffer.trim().parse::<f32>() {
            Ok(num) => {
                result = num;
                break;
            }
            Err(_) => {
                println!("Error, not a number");
                continue;
            }
        }
    }

    println!();

    result
}

fn main() {
    let from = choose_currency("Choose a currency to convert from");
    let to = choose_currency("Choose a currency to convert to");
    let amount = get_number_from_buffer();

    println!(
        "\n{} {} is {} {}",
        amount,
        from.shortcode,
        from.convert_to_currency(amount, to),
        to.shortcode
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_usd() {
        assert_eq!((*AUD).convert_to_usd(200.0), 144.0);
    }

    #[test]
    fn it_converts_aud_to_jpy() {
        assert_eq!((*AUD).convert_to_currency(200.0, &(*JPY)), 15824.175);
    }
}
