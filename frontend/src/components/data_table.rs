//! Data Table Component
//!
//! Virtualized data table for displaying large datasets efficiently.

use leptos::prelude::*;

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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Asc,
    Desc,
}

/// Basic data table component - placeholder for future generic table
#[component]
pub fn DataTable(
    columns: Vec<Column>,
    #[prop(default = false)] loading: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let sort_column = RwSignal::new(None::<String>);
    let sort_direction = RwSignal::new(SortDirection::Asc);

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
                            let col_key2 = col_key.clone();
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
                                        if sortable && sort_column.get().as_ref() == Some(&col_key2) {
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
                        }).collect::<Vec<_>>()}
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
                        {children()}
                    </Show>
                </tbody>
            </table>
        </div>
    }
}
