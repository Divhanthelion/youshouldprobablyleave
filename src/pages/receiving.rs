//! Receiving Page

use leptos::*;
use crate::components::Card;

#[component]
pub fn ReceivingPage() -> impl IntoView {
    view! {
        <div class="page receiving-page">
            <div class="page-header">
                <div>
                    <h1>"Receiving"</h1>
                    <p class="subtitle">"Process inbound shipments"</p>
                </div>
                <div class="page-actions">
                    <button class="btn btn-primary">"+ New Receipt"</button>
                </div>
            </div>
            
            <Card>
                <table class="data-table">
                    <thead>
                        <tr>
                            <th>"Receipt #"</th>
                            <th>"PO Number"</th>
                            <th>"Supplier"</th>
                            <th>"Expected"</th>
                            <th>"Status"</th>
                            <th>"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>"RCV-00000045"</code></td>
                            <td>"PO-2024-0123"</td>
                            <td>"Global Supplies Inc"</td>
                            <td>"Today"</td>
                            <td><span class="badge badge-warning">"Receiving"</span></td>
                            <td>
                                <button class="btn btn-sm btn-primary">"Continue"</button>
                            </td>
                        </tr>
                        <tr>
                            <td><code>"RCV-00000044"</code></td>
                            <td>"PO-2024-0122"</td>
                            <td>"Parts Direct"</td>
                            <td>"Yesterday"</td>
                            <td><span class="badge badge-success">"Completed"</span></td>
                            <td>
                                <button class="btn btn-sm btn-ghost">"View"</button>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </Card>
        </div>
    }
}

