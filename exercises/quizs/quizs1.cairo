// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 3 cairobucks.
// - If Mary buys more than 40 apples, each apple only costs 2 cairobucks!
// Write a function that calculates the price of an order of apples given
// the quantity bought. No hints this time!

// I AM NOT DONE

// Put your function here!
fn calculate_price_of_apples(quantity: usize) -> usize {
    const APPLE_PRICE_NORMAL: usize = 3;
    const APPLE_PRICE_DISCOUNT: usize = 2;
    const DISCOUNT_THRESHOLD: usize = 40;

    if quantity > DISCOUNT_THRESHOLD {
        quantity * APPLE_PRICE_DISCOUNT
    } else {
        quantity * APPLE_PRICE_NORMAL
    }
}

// Do not change the tests!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(105, price1, "Incorrect price");
    assert_eq!(120, price2, "Incorrect price");
    assert_eq!(82, price3, "Incorrect price");
    assert_eq!(130, price4, "Incorrect price");
}
