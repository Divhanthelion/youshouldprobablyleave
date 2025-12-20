//! Timesheets Page

use leptos::*;
use crate::components::Card;

#[component]
pub fn TimesheetsPage() -> impl IntoView {
    let clocked_in = create_rw_signal(false);
    
    let toggle_clock = move |_| {
        clocked_in.update(|c| *c = !*c);
    };
    
    view! {
        <div class="page timesheets-page">
            <div class="page-header">
                <div>
                    <h1>"Timesheets"</h1>
                    <p class="subtitle">"Track your work hours"</p>
                </div>
                <div class="page-actions">
                    <button class="btn btn-secondary">"Export"</button>
                </div>
            </div>
            
            <Card class=Some("clock-card".to_string())>
                <div class="clock-section">
                    <div class="current-time">"09:45 AM"</div>
                    <div class="clock-status">
                        {move || if clocked_in.get() {
                            "Clocked in since 8:00 AM"
                        } else {
                            "Not clocked in"
                        }}
                    </div>
                    <button 
                        class=move || format!("clock-btn {}", if clocked_in.get() { "clocked-in" } else { "" })
                        on:click=toggle_clock
                    >
                        {move || if clocked_in.get() { "🔴 Clock Out" } else { "🟢 Clock In" }}
                    </button>
                    <p class="biometric-note">"Biometric verification required"</p>
                </div>
            </Card>
            
            <Card title=Some("This Week".to_string())>
                <table class="data-table">
                    <thead>
                        <tr>
                            <th>"Date"</th>
                            <th>"Clock In"</th>
                            <th>"Clock Out"</th>
                            <th>"Break"</th>
                            <th>"Total"</th>
                            <th>"Status"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"Mon, Jan 15"</td>
                            <td>"8:00 AM"</td>
                            <td>"5:30 PM"</td>
                            <td>"30 min"</td>
                            <td>"9.0 hrs"</td>
                            <td><span class="badge badge-success">"Approved"</span></td>
                        </tr>
                        <tr>
                            <td>"Tue, Jan 16"</td>
                            <td>"8:15 AM"</td>
                            <td>"5:00 PM"</td>
                            <td>"30 min"</td>
                            <td>"8.25 hrs"</td>
                            <td><span class="badge badge-success">"Approved"</span></td>
                        </tr>
                        <tr class="current-row">
                            <td>"Wed, Jan 17"</td>
                            <td>"8:00 AM"</td>
                            <td>"-"</td>
                            <td>"-"</td>
                            <td>"-"</td>
                            <td><span class="badge badge-info">"Active"</span></td>
                        </tr>
                    </tbody>
                </table>
                
                <div class="week-summary">
                    <span>"Week Total: "</span>
                    <strong>"17.25 hrs"</strong>
                    <span class="overtime">" (0.25 hrs overtime)"</span>
                </div>
            </Card>
        </div>
    }
}

