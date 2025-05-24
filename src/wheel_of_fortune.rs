use leptos::prelude::*;
use leptos_use::{use_interval_fn};
use leptos_use::utils::Pausable;

#[component]
pub fn WheelOfFortune() -> impl IntoView {
    let (counter, set_counter) = signal(0);
    let (rotation, set_rotation) = signal(format!("{}deg", counter.get()));

    let Pausable {
        pause,
        resume,
        is_active,
    } = use_interval_fn(
        move || {
            let c = counter.get() + 1;
            set_counter.set(c);
            set_rotation.set(format!("{}deg", c));
        },
        1,
    );
    
    view! {
        <div class="w-28 h-28 p-6 bg-green-500 text-center font-bold 
                text-white text-center font-bold text-white">
                <div style:rotate=move || rotation>
                {rotation}
                </div>
        </div>
    }
}