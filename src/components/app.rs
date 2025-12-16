use crate::components::{NameList, WheelOfFortune};
use leptos::prelude::*;
use reactive_stores::Store;
use web_sys as sys;

#[derive(Clone, Debug, Default, Store)]
pub struct GlobalState {
    pub name_list: Vec<String>,
}

#[component]
pub fn App() -> impl IntoView {

    let store = Store::new(GlobalState::default());

    if let Some(window) = sys::window() {
        if let Ok(href) = window.location().href() {
            if let Ok(url) = sys::Url::new(&href) {
                let params = url.search_params();

                if let Some(single) = params.get("names") {
                    let collected: Vec<String> = single
                        .split(',')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .collect();

                    if !collected.is_empty() {
                        store.name_list().set(collected);
                    }
                }
            }
        }
    }

    provide_context(store);
    mount_to_body(|| {
        view! {
            <div class="flex flex-row">
            <NameList/>
            <WheelOfFortune/>
            </div>
        }
    })
}