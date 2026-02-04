//! Dashboard Page

use leptos::prelude::*;
use crate::components::{Card, StatCard, Chart, DataPoint};

/// Main dashboard with key metrics
#[component]
pub fn Dashboard() -> impl IntoView {
    // Mock data - would come from Tauri commands
    let (inventory_count, _) = signal("12,456".to_string());
    let (pending_shipments, _) = signal("47".to_string());
    let (deliveries_today, _) = signal("23".to_string());
    let (low_stock_items, _) = signal("8".to_string());

    // Convert to Signal<String> for StatCard
    let inventory_signal: Signal<String> = Signal::from(inventory_count);
    let pending_signal: Signal<String> = Signal::from(pending_shipments);
    let deliveries_signal: Signal<String> = Signal::from(deliveries_today);
    let low_stock_signal: Signal<String> = Signal::from(low_stock_items);

    let chart_data = vec![
        DataPoint { label: "Mon".to_string(), value: 120.0 },
        DataPoint { label: "Tue".to_string(), value: 150.0 },
        DataPoint { label: "Wed".to_string(), value: 180.0 },
        DataPoint { label: "Thu".to_string(), value: 140.0 },
        DataPoint { label: "Fri".to_string(), value: 200.0 },
        DataPoint { label: "Sat".to_string(), value: 90.0 },
        DataPoint { label: "Sun".to_string(), value: 60.0 },
    ];

    view! {
        <div class="dashboard">
            <div class="page-header">
                <h1>"Dashboard"</h1>
                <p class="subtitle">"Welcome back! Here's your warehouse overview."</p>
            </div>

            <div class="stats-grid">
                <StatCard
                    title="Total Inventory"
                    value=inventory_signal
                    icon="📦"
                    trend=5.2
                />
                <StatCard
                    title="Pending Shipments"
                    value=pending_signal
                    icon="🚚"
                    trend=-2.1
                />
                <StatCard
                    title="Deliveries Today"
                    value=deliveries_signal
                    icon="📍"
                    trend=12.5
                />
                <StatCard
                    title="Low Stock Alerts"
                    value=low_stock_signal
                    icon="⚠️"
                    trend=-3.0
                />
            </div>

            <div class="dashboard-grid">
                <Card title="Shipments This Week">
                    <Chart data=chart_data.clone() show_labels=true />
                </Card>

                <Card title="Recent Activity">
                    <div class="activity-list">
                        <ActivityItem
                            icon="📥"
                            title="Receipt RCV-00000123 completed"
                            time="5 minutes ago"
                        />
                        <ActivityItem
                            icon="🚚"
                            title="Shipment SHP-00000456 dispatched"
                            time="12 minutes ago"
                        />
                        <ActivityItem
                            icon="📍"
                            title="Delivery DEL-00000789 delivered"
                            time="25 minutes ago"
                        />
                        <ActivityItem
                            icon="⚠️"
                            title="Low stock alert: SKU-12345"
                            time="1 hour ago"
                        />
                    </div>
                </Card>

                <Card title="Quick Actions">
                    <div class="quick-actions">
                        <QuickActionButton icon="📷" label="Scan Barcode" />
                        <QuickActionButton icon="📦" label="New Shipment" />
                        <QuickActionButton icon="📥" label="Receive Goods" />
                        <QuickActionButton icon="🗺️" label="Route Planner" />
                    </div>
                </Card>

                <Card title="Top Products">
                    <table class="mini-table">
                        <thead>
                            <tr>
                                <th>"SKU"</th>
                                <th>"Name"</th>
                                <th>"Stock"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr><td>"SKU-001"</td><td>"Widget Pro"</td><td>"1,234"</td></tr>
                            <tr><td>"SKU-002"</td><td>"Gadget Plus"</td><td>"956"</td></tr>
                            <tr><td>"SKU-003"</td><td>"Tool Master"</td><td>"723"</td></tr>
                            <tr><td>"SKU-004"</td><td>"Part Essential"</td><td>"512"</td></tr>
                        </tbody>
                    </table>
                </Card>
            </div>
        </div>
    }
}

#[component]
fn ActivityItem(
    icon: &'static str,
    title: &'static str,
    time: &'static str,
) -> impl IntoView {
    view! {
        <div class="activity-item">
            <span class="activity-icon">{icon}</span>
            <div class="activity-content">
                <span class="activity-title">{title}</span>
                <span class="activity-time">{time}</span>
            </div>
        </div>
    }
}

#[component]
fn QuickActionButton(
    icon: &'static str,
    label: &'static str,
) -> impl IntoView {
    view! {
        <button class="quick-action-btn">
            <span class="qa-icon">{icon}</span>
            <span class="qa-label">{label}</span>
        </button>
    }
}
