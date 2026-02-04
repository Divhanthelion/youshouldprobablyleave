//! Chart Component
//!
//! Simple SVG-based charts for dashboard visualizations.

use leptos::prelude::*;

/// Chart data point
#[derive(Clone)]
pub struct DataPoint {
    pub label: String,
    pub value: f64,
}

/// Simple bar chart
#[component]
pub fn Chart(
    data: Vec<DataPoint>,
    #[prop(optional)] height: Option<u32>,
    #[prop(optional)] show_labels: bool,
) -> impl IntoView {
    let height = height.unwrap_or(200);
    let max_value = data.iter().map(|d| d.value).fold(0.0_f64, f64::max);
    let bar_width = 100.0 / (data.len() as f64 * 1.5);
    
    view! {
        <div class="chart-container">
            <svg 
                class="chart"
                viewBox=format!("0 0 100 {}", height)
                preserveAspectRatio="none"
            >
                {data.iter().enumerate().map(|(i, point)| {
                    let bar_height = if max_value > 0.0 {
                        (point.value / max_value) * (height as f64 - 20.0)
                    } else {
                        0.0
                    };
                    let x = (i as f64) * bar_width * 1.5 + bar_width * 0.25;
                    let y = height as f64 - bar_height - 10.0;
                    
                    view! {
                        <g class="chart-bar">
                            <rect
                                x=x
                                y=y
                                width=bar_width
                                height=bar_height
                                class="bar"
                                rx="2"
                            />
                            {if show_labels {
                                Some(view! {
                                    <text
                                        x=x + bar_width / 2.0
                                        y=height as f64 - 2.0
                                        class="bar-label"
                                        text-anchor="middle"
                                    >
                                        {point.label.clone()}
                                    </text>
                                })
                            } else {
                                None
                            }}
                        </g>
                    }
                }).collect::<Vec<_>>()}
            </svg>
        </div>
    }
}

/// Sparkline mini chart
#[component]
pub fn Sparkline(
    data: Vec<f64>,
    #[prop(optional)] color: Option<&'static str>,
) -> impl IntoView {
    let width = 100.0;
    let height = 30.0;
    let max_val = data.iter().fold(0.0_f64, |a, &b| a.max(b));
    let min_val = data.iter().fold(f64::MAX, |a, &b| a.min(b));
    let range = max_val - min_val;
    
    let points: String = data.iter().enumerate().map(|(i, &val)| {
        let x = (i as f64 / (data.len() - 1) as f64) * width;
        let y = if range > 0.0 {
            height - ((val - min_val) / range * height)
        } else {
            height / 2.0
        };
        format!("{:.1},{:.1}", x, y)
    }).collect::<Vec<_>>().join(" ");
    
    view! {
        <svg class="sparkline" viewBox=format!("0 0 {} {}", width, height)>
            <polyline
                points=points
                fill="none"
                stroke=color.unwrap_or("currentColor")
                stroke-width="2"
            />
        </svg>
    }
}

