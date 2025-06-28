use crate::components::{GlobalState, GlobalStateStoreFields};
use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::prelude::*;
use reactive_stores::Store;

#[component]
pub fn NameList() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let button_class = "bg-blue-700 hover:bg-blue-800 px-20 py-3 text-white rounded-lg";
    let name_input_class = "bg-gray-50 border border-gray-900 text-gray-900 rounded-lg w-full p-2.5";

    let name_list = state.name_list();

    let (name, set_name) = signal(String::default());
    let input_element: NodeRef<html::Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let element = input_element
            .get()
            .expect("<input> should be mounted");

        let value = element.value();

        let mut add = name_list.get();
        add.push(value);
        name_list.set(add);

        set_name.set(String::default());
        element.set_value("");
    };

    let delete_name = move |index: usize| {
        let mut current_list = name_list.get();
        if index < current_list.len() {
            current_list.remove(index);
            name_list.set(current_list);
        }
    };

    view! {
        <div class="z-10">
        <div>
            <form on:submit=on_submit>
                <div class="p-2">
                    <input type="text"
                        class=name_input_class
                        placeholder="Name..."
                        value=name
                        node_ref=input_element
                    />
                </div>
                <div class="p-2">
                    <button type="Submit" class=button_class>
                        Submit
                    </button>
                </div>
            </form>
        </div>

        <div class="p-2">
            <ul class="ps-5 mt-2 space-y-1 list-disc list-inside">
            {move || {
                name_list.get().into_iter()
                .enumerate()
                .map(|(index, x)| view!{
                    <li class="flex items-center justify-between group">
                        <span>{x}</span>
                        <button
                            class="ml-2 text-red-500"
                            on:click=move |_| delete_name(index)
                            title="Delete name"
                        >
                            <i class="fa-solid fa-trash"></i>
                        </button>
                    </li>
                })
                .collect_view()
            }}
            </ul>
        </div>
        </div>
    }
}