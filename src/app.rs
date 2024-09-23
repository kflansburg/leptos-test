use leptos::*;

use crate::components::show_data_from_api::ShowDataFromApi;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <h1>"Hello Workers CI!"</h1>
        <ShowDataFromApi />
    }
}
