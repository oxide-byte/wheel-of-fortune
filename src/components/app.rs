use crate::components::{NameList, WheelOfFortune};
use leptos::prelude::*;
use reactive_stores::Store;

#[derive(Clone, Debug, Default, Store)]
pub struct GlobalState {
    pub name_list: Vec<String>,
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(Store::new(GlobalState::default()));
    mount_to_body(|| view! {
        <div class="flex flex-row">
        <NameList/>
        <WheelOfFortune/>
        </div>
    })
}