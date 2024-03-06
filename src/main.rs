use lambda_runtime::{Error, LambdaEvent, service_fn};
use serde::{Deserialize, Serialize};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing::{info, instrument};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[derive(Debug, Deserialize)]
struct CustomEvent {
    k: usize,
    prices: Vec<i32>,
}

#[derive(Serialize)]
struct CustomOutput {
    max_profit: i32,
}

#[instrument]
async fn func(event: LambdaEvent<CustomEvent>) -> Result<CustomOutput, Error> {
    let (event, _context) = event.into_parts();
    info!("Received event: {:?}", event);

    let max_profit = max_profit(event.k, &event.prices);

    Ok(CustomOutput { max_profit })
}

fn max_profit(k: usize, prices: &[i32]) -> i32 {
    if prices.is_empty() || k == 0 {
        return 0;
    }
    let mut dp = vec![-prices[0]; 2 * k];
    for &price in prices {
        dp[0] = dp[0].max(-price);
        for j in 1..2 * k {
            if j % 2 == 0 {
                dp[j] = dp[j].max(dp[j - 1] - price);
            } else {
                dp[j] = dp[j].max(dp[j - 1] + price);
            }
        }
    }
    *dp.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lambda_function() {
        let test_event = CustomEvent {
            k: 2,
            prices: vec![3,2,6,5,0,3],
        };
        let context = Context::default();

        let response = func(LambdaEvent::new(test_event, context)).await;
        assert!(response.is_ok());
        let output = response.unwrap();
        assert_eq!(output.max_profit, 7); // (6-2) + (3-0) = 7
    }
}
