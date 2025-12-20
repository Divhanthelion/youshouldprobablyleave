//! Dashboard Page

use leptos::*;
use crate::components::{Card, StatCard, Chart, DataPoint};

/// Main dashboard with key metrics
#[component]
pub fn Dashboard() -> impl IntoView {
    // Mock data - would come from Tauri commands
    let inventory_count = create_signal("12,456".to_string());
    let pending_shipments = create_signal("47".to_string());
    let deliveries_today = create_signal("23".to_string());
    let low_stock_items = create_signal("8".to_string());
    
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
                    value=inventory_count.0.into()
                    icon=Some("📦")
                    trend=Some(5.2)
                />
                <StatCard 
                    title="Pending Shipments" 
                    value=pending_shipments.0.into()
                    icon=Some("🚚")
                    trend=Some(-2.1)
                />
                <StatCard 
                    title="Deliveries Today" 
                    value=deliveries_today.0.into()
                    icon=Some("📍")
                    trend=Some(12.5)
                />
                <StatCard 
                    title="Low Stock Alerts" 
                    value=low_stock_items.0.into()
                    icon=Some("⚠️")
                    trend=Some(-3.0)
                />
            </div>
            
            <div class="dashboard-grid">
                <Card title=Some("Shipments This Week".to_string())>
                    <Chart data=chart_data.clone() show_labels=true />
                </Card>
                
                <Card title=Some("Recent Activity".to_string())>
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
                
                <Card title=Some("Quick Actions".to_string())>
                    <div class="quick-actions">
                        <QuickActionButton icon="📷" label="Scan Barcode" />
                        <QuickActionButton icon="📦" label="New Shipment" />
                        <QuickActionButton icon="📥" label="Receive Goods" />
                        <QuickActionButton icon="🗺️" label="Route Planner" />
                    </div>
                </Card>
                
                <Card title=Some("Top Products".to_string())>
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

