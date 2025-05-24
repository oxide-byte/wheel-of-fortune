use leptos::prelude::*;
use leptos_use::utils::Pausable;
use leptos_use::{use_raf_fn};

#[component]
pub fn WheelOfFortune() -> impl IntoView {
    let (counter, set_counter) = signal(0);
    let (rotation1, set_rotation1) = signal(format!("{}deg", counter.get()));
    let (rotation2, set_rotation2) = signal(format!("{}deg", counter.get() + 90));

    let Pausable { pause:_, resume:_, is_active:_ } = use_raf_fn(move |_| {
            let c = counter.get() + 1;
            set_counter.set(c);
            set_rotation1.set(format!("{}deg", c));
            set_rotation2.set(format!("{}deg", c + 90));
        },
    );

    view! {
        <div class="relative w-105 h-105 bg-gray-100">

          <div class="absolute top-0 left-0 p-50 -ml-48">
            <div class="w-100 h-20 text-right">
                <div style:rotate=move || rotation1> {rotation1} </div>
            </div>
          </div>
                
          <div class="absolute top-0 left-0 p-50 -ml-48">
            <div class="w-100 h-20 text-right">
                <div style:rotate=move || rotation2> {rotation2} </div>
            </div>
          </div>

        </div>
    }
}