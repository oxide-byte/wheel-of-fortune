use crate::components::{GlobalState, GlobalStateStoreFields};
use leptos::prelude::*;
use leptos_use::utils::Pausable;
use leptos_use::use_interval_fn;
use reactive_stores::Store;

#[derive(Clone, Default)]
struct WheelPart {
    pub name: String,
    pub angle: usize,
}

#[component]
pub fn WheelOfFortune() -> impl IntoView {
    let (counter, set_counter) = signal(0);
    let state = expect_context::<Store<GlobalState>>();
    let (parts, set_parts) = signal(Vec::<WheelPart>::new());

    let Pausable {
        pause: _,
        resume: _,
        is_active: _,
    } = use_interval_fn(
        move || {
            let c = counter.get() + 1;
            set_counter.set(c);

            let state = state.name_list().get();
            if !state.is_empty() {
                let offset = 360 / state.len();

                let v = state.iter()
                    .enumerate()
                    .map(|(i, x)| {
                        WheelPart {
                            name: x.clone(),
                            angle: i * offset + c,
                        }
                    }).collect::<Vec<WheelPart>>();

                set_parts.set(v);
            }
        },
        90,
    );

    view! {
        <div class="relative w-105 h-105 bg-gray-100">
            {move || {
                parts.get().into_iter()
                .map(|x| view!{
                  <div class="absolute top-0 left-0 p-50 -ml-48">
                    <div class="w-100 h-20 text-right">
                        <div style:rotate=move || format!("{}deg", x.angle)> {x.name} </div>
                    </div>
                  </div>
                }).collect_view()
            }}
        </div>
    }
}