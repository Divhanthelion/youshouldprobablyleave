//! Customers Page

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use crate::components::Card;

#[component]
pub fn CustomersPage() -> impl IntoView {
    view! {
        <div class="page customers-page">
            <div class="page-header">
                <div>
                    <h1>"Customers"</h1>
                    <p class="subtitle">"Manage customer relationships"</p>
                </div>
                <div class="page-actions">
                    <button class="btn btn-primary">"+ Add Customer"</button>
                </div>
            </div>

            <Card>
                <table class="data-table">
                    <thead>
                        <tr>
                            <th>"Customer #"</th>
                            <th>"Company"</th>
                            <th>"Contact"</th>
                            <th>"Email"</th>
                            <th>"Type"</th>
                            <th>"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>"CUS-000001"</code></td>
                            <td>"Acme Corporation"</td>
                            <td>"Jane Doe"</td>
                            <td>"jane@acme.com"</td>
                            <td><span class="badge">"Wholesale"</span></td>
                            <td>
                                <A href="/customers/CUS-000001" attr:class="btn btn-sm btn-ghost">"View"</A>
                            </td>
                        </tr>
                        <tr>
                            <td><code>"CUS-000002"</code></td>
                            <td>"Tech Solutions"</td>
                            <td>"Bob Smith"</td>
                            <td>"bob@techsol.com"</td>
                            <td><span class="badge">"Retail"</span></td>
                            <td>
                                <A href="/customers/CUS-000002" attr:class="btn btn-sm btn-ghost">"View"</A>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </Card>
        </div>
    }
}

#[component]
pub fn CustomerDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.get().get("id").map(|s| s.clone()).unwrap_or_default();

    view! {
        <div class="page customer-detail">
            <div class="page-header">
                <A href="/customers" attr:class="back-link">"← Back to Customers"</A>
                <h1>"Customer: " {id}</h1>
            </div>

            <div class="detail-grid">
                <Card title="Contact Information">
                    <dl class="detail-list">
                        <dt>"Company"</dt><dd>"Acme Corporation"</dd>
                        <dt>"Contact"</dt><dd>"Jane Doe"</dd>
                        <dt>"Email"</dt><dd>"jane@acme.com"</dd>
                        <dt>"Phone"</dt><dd>"+1 (555) 987-6543"</dd>
                    </dl>
                </Card>

                <Card title="Shipping Address">
                    <address>
                        "Acme Corporation"<br/>
                        "456 Business Ave"<br/>
                        "Suite 100"<br/>
                        "Los Angeles, CA 90001"
                    </address>
                </Card>
            </div>
        </div>
    }
}
