//! Deliveries Page

use leptos::*;
use leptos_router::*;
use crate::components::Card;

#[component]
pub fn DeliveriesPage() -> impl IntoView {
    view! {
        <div class="page deliveries-page">
            <div class="page-header">
                <div>
                    <h1>"Deliveries"</h1>
                    <p class="subtitle">"Track and optimize delivery routes"</p>
                </div>
                <div class="page-actions">
                    <button class="btn btn-secondary">"🗺️ Route Planner"</button>
                    <button class="btn btn-primary">"+ New Delivery"</button>
                </div>
            </div>
            
            <div class="delivery-stats">
                <div class="stat-card">
                    <span class="stat-icon">"🚚"</span>
                    <span class="stat-value">"12"</span>
                    <span class="stat-label">"In Transit"</span>
                </div>
                <div class="stat-card">
                    <span class="stat-icon">"✅"</span>
                    <span class="stat-value">"28"</span>
                    <span class="stat-label">"Delivered Today"</span>
                </div>
                <div class="stat-card">
                    <span class="stat-icon">"⏳"</span>
                    <span class="stat-value">"5"</span>
                    <span class="stat-label">"Pending"</span>
                </div>
            </div>
            
            <Card title=Some("Today's Deliveries".to_string())>
                <table class="data-table">
                    <thead>
                        <tr>
                            <th>"Delivery #"</th>
                            <th>"Customer"</th>
                            <th>"Address"</th>
                            <th>"Time Window"</th>
                            <th>"Status"</th>
                            <th>"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>"DEL-00000789"</code></td>
                            <td>"John Smith"</td>
                            <td>"123 Main St, NYC"</td>
                            <td>"9:00 - 12:00"</td>
                            <td><span class="badge badge-info">"En Route"</span></td>
                            <td>
                                <A href="/deliveries/DEL-00000789" class="btn btn-sm btn-ghost">"Track"</A>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </Card>
        </div>
    }
}

#[component]
pub fn DeliveryDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.get().get("id").cloned().unwrap_or_default();
    
    view! {
        <div class="page delivery-detail">
            <div class="page-header">
                <A href="/deliveries" class="back-link">"← Back to Deliveries"</A>
                <h1>"Delivery: " {id}</h1>
            </div>
            
            <div class="delivery-map">
                <div class="map-placeholder">"🗺️ Map would render here"</div>
            </div>
            
            <Card title=Some("Delivery Details".to_string())>
                <dl class="detail-list">
                    <dt>"Customer"</dt><dd>"John Smith"</dd>
                    <dt>"Address"</dt><dd>"123 Main St, New York, NY 10001"</dd>
                    <dt>"Phone"</dt><dd>"+1 (555) 123-4567"</dd>
                    <dt>"Instructions"</dt><dd>"Leave at front door"</dd>
                </dl>
            </Card>
        </div>
    }
}

