use prometheus::Encoder;

use crate::infrastructure;

/// PrometheusService handles metric collection and reporting using Prometheus
#[derive(Debug)]
pub struct PrometheusService;

impl PrometheusService {
    /// Gathers and encodes all registered Prometheus metrics
    /// Returns encoded metrics as bytes
    pub async fn get_metrics() -> Vec<u8> {
        let encoder = prometheus::TextEncoder::new();
        let mut buffer = vec![];
        let metric_families = prometheus::gather();
        encoder.encode(&metric_families, &mut buffer).unwrap_or_else(|_| {
            buffer.append(&mut b"Error in encoder metrics".into_iter().map(|b| *b).collect::<Vec<u8>>())
        });

        buffer
    }

    /// Increments campaign creation counter with time advance label
    pub fn increment_campaign_created(time_advance: u32) {
        if let Ok(metrics) = infrastructure::metrics::prometheus::APP_METRICS.lock() {
            metrics
                .campaigns_created
                .with_label_values(&[&time_advance.to_string()])
                .inc();
        }
    }

    /// Increments campaign deletion counter with time advance label  
    pub fn increment_campaign_deleted(time_advance: u32) {
        if let Ok(metrics) = infrastructure::metrics::prometheus::APP_METRICS.lock() {
            metrics
                .campaigns_deleted
                .with_label_values(&[&time_advance.to_string()])
                .inc();
        }
    }

    /// Increments campaign update counter with time advance label
    pub fn increment_campaign_updated(time_advance: u32) {
        if let Ok(metrics) = infrastructure::metrics::prometheus::APP_METRICS.lock() {
            metrics
                .campaigns_updated
                .with_label_values(&[&time_advance.to_string()])
                .inc();
        }
    }

    /// Increments the ad impression counter and tracks associated revenue
    /// metrics
    ///
    /// # Arguments
    /// * `time_advance` - Time window identifier for metric aggregation
    /// * `make_money` - Revenue generated from this ad impression
    ///
    /// Records:
    /// - Total ad impressions for the time window
    /// - Revenue for the time window
    /// - Cumulative revenue across all time windows
    pub fn ads_visits(time_advance: u32, make_money: f64) {
        if let Ok(metrics) = infrastructure::metrics::prometheus::APP_METRICS.lock() {
            metrics.ads_visits.with_label_values(&[&time_advance.to_string()]).inc();
            metrics
                .make_money_visits
                .with_label_values(&[&time_advance.to_string()])
                .add(make_money);
            metrics.total_make_money_visits.add(make_money);
        }
    }

    /// Increments the ad click counter and tracks associated revenue metrics
    ///
    /// # Arguments
    /// * `time_advance` - Time window identifier for metric aggregation
    /// * `make_money` - Revenue generated from this ad click
    ///
    /// Records:
    /// - Total ad clicks for the time window
    /// - Revenue for the time window
    /// - Cumulative revenue across all time windows
    pub fn ads_clicks(time_advance: u32, make_money: f64) {
        if let Ok(metrics) = infrastructure::metrics::prometheus::APP_METRICS.lock() {
            metrics.ads_clicks.with_label_values(&[&time_advance.to_string()]).inc();
            metrics
                .make_money_clicks
                .with_label_values(&[&time_advance.to_string()])
                .add(make_money);
            metrics.total_make_money_clicks.add(make_money);
        }
    }

    /// Adds to total client counter
    pub fn add_total_clients(value: i64) {
        if let Ok(metrics) = infrastructure::metrics::prometheus::APP_METRICS.lock() {
            metrics.total_clients.add(value);
        }
    }

    /// Adds to total advertiser counter
    pub fn add_total_advertisers(value: i64) {
        if let Ok(metrics) = infrastructure::metrics::prometheus::APP_METRICS.lock() {
            metrics.total_advertisers.add(value);
        }
    }

    /// Increments HTTP request counter with method and path labels
    pub fn increment_http_requests_total(request_method: &str, request_path: &str) {
        if let Ok(metrics) = infrastructure::metrics::prometheus::APP_METRICS.lock() {
            metrics
                .http_requests_total
                .with_label_values(&[request_method, request_path])
                .inc();
        }
    }

    /// Records HTTP response time with method and path labels
    pub fn observe_http_response_time(value: f64, request_method: &str, request_path: &str) {
        if let Ok(metrics) = infrastructure::metrics::prometheus::APP_METRICS.lock() {
            metrics
                .http_response_time
                .with_label_values(&[request_method, request_path])
                .observe(value);
        }
    }
}
