//! Demand Forecasting Engine
//! 
//! Uses time series analysis (ETS/ARIMA) to predict future inventory demand.

use serde::{Deserialize, Serialize};
use wms_core::error::{WmsError, Result};

/// Forecast result with predictions and confidence intervals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastResult {
    /// Predicted demand values
    pub predictions: Vec<f64>,
    /// Lower confidence bound (95%)
    pub lower_bound: Vec<f64>,
    /// Upper confidence bound (95%)
    pub upper_bound: Vec<f64>,
    /// Suggested reorder point based on forecast
    pub suggested_reorder_point: f64,
    /// Suggested reorder quantity
    pub suggested_reorder_quantity: f64,
    /// Model used for forecasting
    pub model_type: ForecastModel,
    /// Model fit metrics
    pub metrics: ForecastMetrics,
}

/// Available forecast models
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ForecastModel {
    /// Exponential Smoothing (Error, Trend, Seasonality)
    Ets,
    /// Simple Moving Average
    Sma,
    /// Weighted Moving Average  
    Wma,
    /// Naive forecast (last value)
    Naive,
}

impl Default for ForecastModel {
    fn default() -> Self {
        Self::Ets
    }
}

/// Forecast model fit metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastMetrics {
    /// Mean Absolute Error
    pub mae: f64,
    /// Mean Squared Error
    pub mse: f64,
    /// Root Mean Squared Error
    pub rmse: f64,
    /// Mean Absolute Percentage Error
    pub mape: f64,
}

/// Forecast engine for demand prediction
pub struct ForecastEngine {
    /// Default number of periods for moving average
    default_window: usize,
    /// Safety stock multiplier
    safety_stock_multiplier: f64,
}

impl ForecastEngine {
    /// Create a new forecast engine
    pub fn new() -> Self {
        Self {
            default_window: 7,
            safety_stock_multiplier: 1.65, // 95% service level
        }
    }
    
    /// Run forecast on historical data
    pub fn forecast(&self, history: &[f64], days_ahead: u32) -> Result<ForecastResult> {
        if history.is_empty() {
            return Err(WmsError::Forecast("No historical data provided".to_string()));
        }
        
        let days = days_ahead as usize;
        
        // Try ETS first, fall back to simpler methods if needed
        let (predictions, model) = if history.len() >= 30 {
            (self.exponential_smoothing(history, days), ForecastModel::Ets)
        } else if history.len() >= 7 {
            (self.simple_moving_average(history, days), ForecastModel::Sma)
        } else {
            (self.naive_forecast(history, days), ForecastModel::Naive)
        };
        
        // Calculate confidence intervals
        let std_dev = self.calculate_std_dev(history);
        let margin = self.safety_stock_multiplier * std_dev;
        
        let lower_bound: Vec<f64> = predictions.iter()
            .map(|&p| (p - margin).max(0.0))
            .collect();
        let upper_bound: Vec<f64> = predictions.iter()
            .map(|&p| p + margin)
            .collect();
        
        // Calculate reorder suggestions
        let avg_daily_demand = history.iter().sum::<f64>() / history.len() as f64;
        let lead_time_demand = avg_daily_demand * 7.0; // Assume 7 day lead time
        let safety_stock = margin * (7.0_f64).sqrt();
        
        let suggested_reorder_point = lead_time_demand + safety_stock;
        let suggested_reorder_quantity = avg_daily_demand * 30.0; // 30 day supply
        
        // Calculate fit metrics using hold-out validation
        let metrics = self.calculate_metrics(history);
        
        Ok(ForecastResult {
            predictions,
            lower_bound,
            upper_bound,
            suggested_reorder_point,
            suggested_reorder_quantity,
            model_type: model,
            metrics,
        })
    }
    
    /// Exponential Smoothing (Simple ETS)
    fn exponential_smoothing(&self, history: &[f64], periods: usize) -> Vec<f64> {
        // Optimize alpha using grid search
        let alpha = self.optimize_alpha(history);
        
        // Initialize with first value
        let mut level = history[0];
        
        // Fit the model
        for &value in &history[1..] {
            level = alpha * value + (1.0 - alpha) * level;
        }
        
        // Forecast (flat for simple ETS)
        vec![level; periods]
    }
    
    /// Optimize smoothing parameter alpha
    fn optimize_alpha(&self, history: &[f64]) -> f64 {
        let mut best_alpha = 0.3;
        let mut best_mse = f64::MAX;
        
        for alpha_int in 1..10 {
            let alpha = alpha_int as f64 * 0.1;
            let mse = self.ets_mse(history, alpha);
            
            if mse < best_mse {
                best_mse = mse;
                best_alpha = alpha;
            }
        }
        
        best_alpha
    }
    
    /// Calculate MSE for given alpha
    fn ets_mse(&self, history: &[f64], alpha: f64) -> f64 {
        if history.len() < 2 {
            return f64::MAX;
        }
        
        let mut level = history[0];
        let mut sum_sq_error = 0.0;
        
        for &value in &history[1..] {
            let error = value - level;
            sum_sq_error += error * error;
            level = alpha * value + (1.0 - alpha) * level;
        }
        
        sum_sq_error / (history.len() - 1) as f64
    }
    
    /// Simple Moving Average forecast
    fn simple_moving_average(&self, history: &[f64], periods: usize) -> Vec<f64> {
        let window = self.default_window.min(history.len());
        let recent: Vec<f64> = history.iter()
            .rev()
            .take(window)
            .copied()
            .collect();
        
        let avg = recent.iter().sum::<f64>() / recent.len() as f64;
        vec![avg; periods]
    }
    
    /// Naive forecast (last value)
    fn naive_forecast(&self, history: &[f64], periods: usize) -> Vec<f64> {
        let last = *history.last().unwrap_or(&0.0);
        vec![last; periods]
    }
    
    /// Calculate standard deviation
    fn calculate_std_dev(&self, data: &[f64]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }
        
        let mean = data.iter().sum::<f64>() / data.len() as f64;
        let variance = data.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / (data.len() - 1) as f64;
        
        variance.sqrt()
    }
    
    /// Calculate forecast metrics using hold-out validation
    fn calculate_metrics(&self, history: &[f64]) -> ForecastMetrics {
        if history.len() < 10 {
            return ForecastMetrics {
                mae: 0.0,
                mse: 0.0,
                rmse: 0.0,
                mape: 0.0,
            };
        }
        
        // Use last 20% as test set
        let split = history.len() * 8 / 10;
        let train = &history[..split];
        let test = &history[split..];
        
        // Generate predictions for test period
        let predictions = self.exponential_smoothing(train, test.len());
        
        // Calculate errors
        let errors: Vec<f64> = test.iter()
            .zip(predictions.iter())
            .map(|(&actual, &pred)| actual - pred)
            .collect();
        
        let n = errors.len() as f64;
        
        let mae = errors.iter().map(|e| e.abs()).sum::<f64>() / n;
        let mse = errors.iter().map(|e| e * e).sum::<f64>() / n;
        let rmse = mse.sqrt();
        
        let mape = test.iter()
            .zip(predictions.iter())
            .filter(|(actual, _)| **actual != 0.0)
            .map(|(actual, pred)| ((*actual - *pred) / *actual).abs())
            .sum::<f64>() / n * 100.0;
        
        ForecastMetrics { mae, mse, rmse, mape }
    }
}

impl Default for ForecastEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_forecast_with_trend() {
        let engine = ForecastEngine::new();
        let history: Vec<f64> = (0..50).map(|i| 100.0 + i as f64 * 2.0 + (i as f64 * 0.5).sin() * 10.0).collect();
        
        let result = engine.forecast(&history, 7).unwrap();
        
        assert_eq!(result.predictions.len(), 7);
        assert!(result.predictions[0] > 0.0);
        assert!(result.metrics.rmse > 0.0);
    }
    
    #[test]
    fn test_forecast_insufficient_data() {
        let engine = ForecastEngine::new();
        let history: Vec<f64> = vec![];
        
        let result = engine.forecast(&history, 7);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_reorder_suggestions() {
        let engine = ForecastEngine::new();
        let history: Vec<f64> = vec![10.0; 60]; // Steady demand
        
        let result = engine.forecast(&history, 30).unwrap();
        
        // With steady demand of 10/day, reorder point should be around 70 (7 days lead time)
        assert!(result.suggested_reorder_point > 50.0);
        assert!(result.suggested_reorder_point < 150.0);
    }
}

