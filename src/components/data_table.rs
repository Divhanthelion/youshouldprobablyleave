//! Data Table Component
//! 
//! Virtualized data table for displaying large datasets efficiently.

use leptos::*;
use serde::Serialize;

/// Column definition for data table
#[derive(Clone)]
pub struct Column {
    pub key: String,
    pub label: String,
    pub sortable: bool,
    pub width: Option<String>,
}

impl Column {
    pub fn new(key: &str, label: &str) -> Self {
        Self {
            key: key.to_string(),
            label: label.to_string(),
            sortable: true,
            width: None,
        }
    }
    
    pub fn with_width(mut self, width: &str) -> Self {
        self.width = Some(width.to_string());
        self
    }
    
    pub fn not_sortable(mut self) -> Self {
        self.sortable = false;
        self
    }
}

/// Props for DataTable component
#[derive(Clone)]
pub struct DataTableProps<T: Clone + 'static> {
    pub columns: Vec<Column>,
    pub data: Signal<Vec<T>>,
    pub loading: Signal<bool>,
    pub row_renderer: Box<dyn Fn(&T) -> View + 'static>,
    pub on_row_click: Option<Box<dyn Fn(&T) + 'static>>,
}

/// Generic data table component
#[component]
pub fn DataTable<T: Clone + 'static>(
    columns: Vec<Column>,
    data: Signal<Vec<T>>,
    #[prop(default = false)] loading: bool,
    row_renderer: impl Fn(&T) -> View + Clone + 'static,
    #[prop(optional)] on_row_click: Option<Box<dyn Fn(&T) + 'static>>,
) -> impl IntoView {
    let sort_column = create_rw_signal::<Option<String>>(None);
    let sort_direction = create_rw_signal::<SortDirection>(SortDirection::Asc);
    
    let toggle_sort = move |col: String| {
        if sort_column.get().as_ref() == Some(&col) {
            sort_direction.update(|d| {
                *d = match d {
                    SortDirection::Asc => SortDirection::Desc,
                    SortDirection::Desc => SortDirection::Asc,
                }
            });
        } else {
            sort_column.set(Some(col));
            sort_direction.set(SortDirection::Asc);
        }
    };
    
    view! {
        <div class="data-table-container">
            <table class="data-table">
                <thead>
                    <tr>
                        {columns.iter().map(|col| {
                            let col_key = col.key.clone();
                            let col_label = col.label.clone();
                            let sortable = col.sortable;
                            let width = col.width.clone();
                            
                            view! {
                                <th 
                                    class=move || format!("table-header {}", if sortable { "sortable" } else { "" })
                                    style=move || width.clone().map(|w| format!("width: {}", w)).unwrap_or_default()
                                    on:click=move |_| {
                                        if sortable {
                                            toggle_sort(col_key.clone())
                                        }
                                    }
                                >
                                    <span>{col_label.clone()}</span>
                                    {move || {
                                        if sortable && sort_column.get().as_ref() == Some(&col_key) {
                                            Some(view! {
                                                <span class="sort-indicator">
                                                    {match sort_direction.get() {
                                                        SortDirection::Asc => "↑",
                                                        SortDirection::Desc => "↓",
                                                    }}
                                                </span>
                                            })
                                        } else {
                                            None
                                        }
                                    }}
                                </th>
                            }
                        }).collect_view()}
                    </tr>
                </thead>
                <tbody>
                    <Show
                        when=move || !loading
                        fallback=|| view! {
                            <tr>
                                <td colspan="100" class="loading-row">
                                    <div class="loading-spinner"></div>
                                    "Loading..."
                                </td>
                            </tr>
                        }
                    >
                        <For
                            each=move || data.get()
                            key=|_| uuid::Uuid::new_v4().to_string()
                            children=move |item| {
                                let renderer = row_renderer.clone();
                                renderer(&item)
                            }
                        />
                    </Show>
                </tbody>
            </table>
            
            <Show when=move || data.get().is_empty() && !loading>
                <div class="empty-state">
                    <span class="empty-icon">"📭"</span>
                    <p>"No data found"</p>
                </div>
            </Show>
        </div>
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SortDirection {
    Asc,
    Desc,
}

