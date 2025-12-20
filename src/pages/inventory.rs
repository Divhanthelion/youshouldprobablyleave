//! Inventory Page

use leptos::*;
use leptos_router::*;
use crate::components::{Card, SearchInput, Badge};

/// Inventory list page
#[component]
pub fn InventoryPage() -> impl IntoView {
    let items = create_rw_signal(vec![
        InventoryRow { sku: "SKU-001".into(), name: "Widget Pro".into(), qty: 1234, reorder: 100, status: "In Stock".into() },
        InventoryRow { sku: "SKU-002".into(), name: "Gadget Plus".into(), qty: 956, reorder: 200, status: "In Stock".into() },
        InventoryRow { sku: "SKU-003".into(), name: "Tool Master".into(), qty: 45, reorder: 100, status: "Low Stock".into() },
        InventoryRow { sku: "SKU-004".into(), name: "Part Essential".into(), qty: 512, reorder: 50, status: "In Stock".into() },
    ]);
    
    view! {
        <div class="page inventory-page">
            <div class="page-header">
                <div>
                    <h1>"Inventory"</h1>
                    <p class="subtitle">"Manage your warehouse inventory"</p>
                </div>
                <div class="page-actions">
                    <button class="btn btn-secondary">"Export"</button>
                    <button class="btn btn-primary">"+ Add Item"</button>
                </div>
            </div>
            
            <Card>
                <div class="table-toolbar">
                    <SearchInput placeholder=Some("Search by SKU or name...") />
                    <select class="form-select">
                        <option>"All Categories"</option>
                        <option>"Electronics"</option>
                        <option>"Hardware"</option>
                    </select>
                </div>
                
                <table class="data-table">
                    <thead>
                        <tr>
                            <th>"SKU"</th>
                            <th>"Name"</th>
                            <th>"Quantity"</th>
                            <th>"Reorder Point"</th>
                            <th>"Status"</th>
                            <th>"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <For
                            each=move || items.get()
                            key=|item| item.sku.clone()
                            children=move |item| {
                                let status_class = if item.status == "Low Stock" { "badge-warning" } else { "badge-success" };
                                view! {
                                    <tr>
                                        <td><code>{item.sku.clone()}</code></td>
                                        <td>{item.name.clone()}</td>
                                        <td>{item.qty.to_string()}</td>
                                        <td>{item.reorder.to_string()}</td>
                                        <td><span class=format!("badge {}", status_class)>{item.status.clone()}</span></td>
                                        <td>
                                            <A href=format!("/inventory/{}", item.sku) class="btn btn-sm btn-ghost">"View"</A>
                                        </td>
                                    </tr>
                                }
                            }
                        />
                    </tbody>
                </table>
            </Card>
        </div>
    }
}

#[derive(Clone)]
struct InventoryRow {
    sku: String,
    name: String,
    qty: i32,
    reorder: i32,
    status: String,
}

/// Inventory detail page
#[component]
pub fn InventoryDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.get().get("id").cloned().unwrap_or_default();
    
    view! {
        <div class="page inventory-detail">
            <div class="page-header">
                <A href="/inventory" class="back-link">"← Back to Inventory"</A>
                <h1>"Item: " {id}</h1>
            </div>
            
            <div class="detail-grid">
                <Card title=Some("Item Details".to_string())>
                    <dl class="detail-list">
                        <dt>"SKU"</dt><dd>{id}</dd>
                        <dt>"Name"</dt><dd>"Widget Pro"</dd>
                        <dt>"Category"</dt><dd>"Electronics"</dd>
                        <dt>"Unit"</dt><dd>"Each"</dd>
                        <dt>"Barcode"</dt><dd>"5012345678901"</dd>
                    </dl>
                </Card>
                
                <Card title=Some("Stock Levels".to_string())>
                    <div class="stock-info">
                        <div class="stock-stat">
                            <span class="stat-value">"1,234"</span>
                            <span class="stat-label">"On Hand"</span>
                        </div>
                        <div class="stock-stat">
                            <span class="stat-value">"50"</span>
                            <span class="stat-label">"Reserved"</span>
                        </div>
                        <div class="stock-stat">
                            <span class="stat-value">"100"</span>
                            <span class="stat-label">"Reorder Point"</span>
                        </div>
                    </div>
                </Card>
            </div>
        </div>
    }
}

