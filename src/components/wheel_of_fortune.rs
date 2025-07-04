use crate::components::{GlobalState, GlobalStateStoreFields};
use leptos::prelude::*;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;
use reactive_stores::Store;
use std::f64::consts::PI;
use thaw::SpinButton;
use web_sys::js_sys::Math;

#[derive(Clone, Default)]
struct WheelPart {
    pub name: String,
    pub angle: usize,
}
const WHEEL_COLORS: [&str; 12] = [
    "#FF6B6B", "#4ECDC4", "#45B7D1", "#96CEB4", "#FFEAA7", "#DDA0DD", "#98D8C8", "#F7DC6F",
    "#BB8FCE", "#85C1E9", "#F8C471", "#82E0AA",
];

#[component]
pub fn WheelOfFortune() -> impl IntoView {
    let (counter, set_counter) = signal(0);
    let (interval, set_interval) = signal(20_u64);
    let speed = RwSignal::<usize>::new(50);
    let state = expect_context::<Store<GlobalState>>();
    let (parts, set_parts) = signal(Vec::<WheelPart>::new());
    let (is_spinning, set_is_spinning) = signal(false);
    let (current_speed, set_current_speed) = signal(0.0);
    let (winner, set_winner) = signal("".to_string());

    let Pausable {
        pause: _,
        resume,
        is_active,
    } = use_interval_fn(
        move || {
            let c = counter.get() + current_speed.get() as usize;
            set_counter.set(c);

            if is_spinning.get()
                && Math::random() > 0.1 {
                    let new_speed = current_speed.get() * 0.995;
                    if new_speed > 0.1 {
                        set_current_speed.set(new_speed);
                    } else {
                        set_current_speed.set(0.0);
                        set_is_spinning.set(false);
                    }
                }

            let name_list = state.name_list().get();
            if !name_list.is_empty() {
                let offset = 360 / name_list.len();

                let v = name_list
                    .iter()
                    .enumerate()
                    .map(|(i, x)| WheelPart {
                        name: x.clone(),
                        angle: i * offset + c,
                    })
                    .collect::<Vec<WheelPart>>();

                set_parts.set(v);

                let num_names = name_list.len();
                let slice_angle = 360.0 / num_names as f64;
                let rotation = c % 360;
                let target_angle = (360 - rotation) % 360;
                let winner_index = (target_angle as f64 / slice_angle).floor() as usize;
                if winner_index < num_names {
                    set_winner.set(name_list[winner_index].clone());
                }
            } else {
                set_winner.set("".to_string());
            }
        },
        interval,
    );

    let start_spinning = move |_| {
        set_current_speed.set(speed.get() as f64);
        set_is_spinning.set(true);
        if !is_active.get() {
            resume();
        }
    };

    let create_slice_path = |start_angle: f64, end_angle: f64, radius: f64| {
        let start_rad = (start_angle - 90.0) * PI / 180.0;
        let end_rad = (end_angle - 90.0) * PI / 180.0;

        let x1 = radius * start_rad.cos();
        let y1 = radius * start_rad.sin();
        let x2 = radius * end_rad.cos();
        let y2 = radius * end_rad.sin();

        let large_arc_flag = if (end_angle - start_angle) > 180.0 {
            1
        } else {
            0
        };

        format!(
            "M 0 0 L {x1} {y1} A {radius} {radius} 0 {large_arc_flag} 1 {x2} {y2} Z"
        )
    };

    view! {
        <div class="flex flex-col items-center">
            <div class="text-center mb-4">
                <p class="text-3xl font-mono animate-pulse">{move || winner.get()}</p>
            </div>
            <div class="relative w-105 h-105 bg-gray-100 rounded-full overflow-hidden">
                <div
                    class="absolute top-0 left-1/2 -translate-x-1/2 z-20"
                    style="width: 0; height: 0; border-left: 15px solid transparent; border-right: 15px solid transparent; border-top: 20px solid red;"
                ></div>
                {move || {
                    let state = state.name_list().get();
                    let slice_angle = if state.is_empty() { 0.0 } else { 360.0 / state.len() as f64 };
                    let radius = 250.0;
                    let current_rotation = counter.get() as f64;

                    view! {
                        <div class="absolute inset-0 w-full h-full">
                            <svg
                                class="w-full h-full"
                                viewBox="-250 -250 500 500"
                                style:transform=move || format!("rotate({current_rotation}deg)")
                            >
                                {if !state.is_empty() {
                                    state.iter().enumerate().map(|(i, _)| {
                                        let start_angle = i as f64 * slice_angle;
                                        let end_angle = (i + 1) as f64 * slice_angle;
                                        let color = WHEEL_COLORS[i % WHEEL_COLORS.len()];

                                        view! {
                                            <path
                                                d=create_slice_path(start_angle, end_angle, radius)
                                                fill=color
                                                stroke="#333"
                                                stroke-width="2"
                                            />
                                        }
                                    }).collect::<Vec<_>>()
                                } else {
                                    Vec::<View<_>>::new()
                                }}
                                <circle cx="0" cy="0" r="20" fill="#333" stroke="#fff" stroke-width="2"/>
                                <circle cx="0" cy="0" r="250" fill="none" stroke="#333" stroke-width="3"/>
                            </svg>
                        </div>
                    }.into_view()
                }}

                {move || {
                    parts.get().into_iter()
                    .map(|x| {
                        let font_size_class = if parts.get().len() > 18 { "text-xs" } else { "text-lg" };
                        let slice_angle = 360.0 / parts.get().len() as f64;
                        let text_angle = x.angle as f64 + slice_angle / 2.0;
                        let text_radius = 150.0;
                        let text_rad = (text_angle - 90.0) * PI / 180.0;
                        let text_x = text_radius * text_rad.cos();
                        let text_y = text_radius * text_rad.sin();
                        view!{
                            <div
                                class="absolute text-white font-bold text-lg drop-shadow-lg pointer-events-none"
                                style:left=move || format!("calc(50% + {text_x}px)")
                                style:top=move || format!("calc(50% + {text_y}px)")
                                style:transform=move || format!("translate(-50%, -50%) rotate({}deg)", text_angle + 90.0)
                            >
                            <span class=font_size_class>{x.name}</span>
                            </div>
                        }
                    }).collect_view()
                }}

                // Start/Stop Button
                <div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 z-10">
                    <button
                        class="w-45 h-40 rounded-full hover:shadow-m transition-shadow duration-200 flex items-center justify-center"
                        on:click=start_spinning
                        title="Start Wheel"
                    >
                        <img
                            src="./public/start_button.png"
                            alt="Start/Stop Wheel"
                            class="w-45 h-40"
                        />
                    </button>
                </div>
            </div>
            <div class="p-2">
                <div><h1 class="text-4xl font-extrabold">SPEED</h1></div>
                <div><SpinButton<u64> value=(interval, set_interval) step_page=10 min=0 max=1000/></div>
                <div><SpinButton<usize> value=speed step_page=1 min=0 max=100/></div>
            </div>
        </div>
    }
}