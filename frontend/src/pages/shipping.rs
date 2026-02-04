//! Shipping Page

use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::Card;

/// Shipping list page
#[component]
pub fn ShippingPage() -> impl IntoView {
    view! {
        <div class="page shipping-page">
            <div class="page-header">
                <div>
                    <h1>"Shipping"</h1>
                    <p class="subtitle">"Manage outbound shipments"</p>
                </div>
                <div class="page-actions">
                    <A href="/shipping/new" attr:class="btn btn-primary">"+ New Shipment"</A>
                </div>
            </div>

            <div class="shipments-tabs">
                <button class="tab active">"All"</button>
                <button class="tab">"Draft"</button>
                <button class="tab">"Picking"</button>
                <button class="tab">"Packed"</button>
                <button class="tab">"Shipped"</button>
            </div>

            <Card>
                <table class="data-table">
                    <thead>
                        <tr>
                            <th>"Shipment #"</th>
                            <th>"Customer"</th>
                            <th>"Items"</th>
                            <th>"Ship Date"</th>
                            <th>"Status"</th>
                            <th>"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>"SHP-00000123"</code></td>
                            <td>"Acme Corp"</td>
                            <td>"5"</td>
                            <td>"2024-01-15"</td>
                            <td><span class="badge badge-warning">"Picking"</span></td>
                            <td>
                                <button class="btn btn-sm btn-ghost">"View"</button>
                                <button class="btn btn-sm btn-primary">"Print Label"</button>
                            </td>
                        </tr>
                        <tr>
                            <td><code>"SHP-00000122"</code></td>
                            <td>"Tech Solutions"</td>
                            <td>"12"</td>
                            <td>"2024-01-15"</td>
                            <td><span class="badge badge-success">"Shipped"</span></td>
                            <td>
                                <button class="btn btn-sm btn-ghost">"View"</button>
                                <button class="btn btn-sm btn-ghost">"Track"</button>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </Card>
        </div>
    }
}

/// New shipment page
#[component]
pub fn NewShipmentPage() -> impl IntoView {
    view! {
        <div class="page new-shipment">
            <div class="page-header">
                <A href="/shipping" attr:class="back-link">"← Back to Shipping"</A>
                <h1>"New Shipment"</h1>
            </div>

            <form class="shipment-form">
                <Card title="Ship To">
                    <div class="form-grid">
                        <div class="form-group">
                            <label>"Customer"</label>
                            <select class="form-select">
                                <option>"Select customer..."</option>
                                <option>"Acme Corp"</option>
                                <option>"Tech Solutions"</option>
                            </select>
                        </div>
                        <div class="form-group">
                            <label>"Name"</label>
                            <input type="text" class="form-input" placeholder="Recipient name" />
                        </div>
                        <div class="form-group full-width">
                            <label>"Address"</label>
                            <input type="text" class="form-input" placeholder="Street address" />
                        </div>
                        <div class="form-group">
                            <label>"City"</label>
                            <input type="text" class="form-input" />
                        </div>
                        <div class="form-group">
                            <label>"State"</label>
                            <input type="text" class="form-input" />
                        </div>
                        <div class="form-group">
                            <label>"ZIP Code"</label>
                            <input type="text" class="form-input" />
                        </div>
                    </div>
                </Card>

                <Card title="Items">
                    <div class="items-section">
                        <button type="button" class="btn btn-secondary">"+ Add Item"</button>
                        <button type="button" class="btn btn-secondary">"📷 Scan"</button>
                    </div>
                </Card>

                <div class="form-actions">
                    <button type="button" class="btn btn-secondary">"Save Draft"</button>
                    <button type="submit" class="btn btn-primary">"Create Shipment"</button>
                </div>
            </form>
        </div>
    }
}
