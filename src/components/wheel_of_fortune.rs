use crate::components::{GlobalState, GlobalStateStoreFields};
use leptos::prelude::*;
use leptos_use::utils::Pausable;
use leptos_use::use_interval_fn;
use reactive_stores::Store;
use thaw::SpinButton;
use std::f64::consts::PI;

#[derive(Clone, Default)]
struct WheelPart {
    pub name: String,
    pub angle: usize,
}
const WHEEL_COLORS: [&str; 12] = [
    "#FF6B6B", "#4ECDC4", "#45B7D1", "#96CEB4", 
    "#FFEAA7", "#DDA0DD", "#98D8C8", "#F7DC6F",
    "#BB8FCE", "#85C1E9", "#F8C471", "#82E0AA"
];

#[component]
pub fn WheelOfFortune() -> impl IntoView {
    let (counter, set_counter) = signal(0);
    let (interval, set_interval) = signal(100_u64);
    let speed = RwSignal::<usize>::new(5);
    let state = expect_context::<Store<GlobalState>>();
    let (parts, set_parts) = signal(Vec::<WheelPart>::new());

    let Pausable {
        pause: _,
        resume: _,
        is_active: _,
    } = use_interval_fn(
        move || {
            let c = counter.get() + speed.get();
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
        interval,
    );
    
    let create_slice_path = |start_angle: f64, end_angle: f64, radius: f64| {
        let start_rad = (start_angle - 90.0) * PI / 180.0;
        let end_rad = (end_angle - 90.0) * PI / 180.0;
        
        let x1 = radius * start_rad.cos();
        let y1 = radius * start_rad.sin();
        let x2 = radius * end_rad.cos();
        let y2 = radius * end_rad.sin();
        
        let large_arc_flag = if (end_angle - start_angle) > 180.0 { 1 } else { 0 };
        
        format!("M 0 0 L {} {} A {} {} 0 {} 1 {} {} Z", 
                x1, y1, radius, radius, large_arc_flag, x2, y2)
    };

    view! {
        <div class="relative w-105 h-105 bg-gray-100 rounded-full overflow-hidden">
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
                            style:transform=move || format!("rotate({}deg)", current_rotation)
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
                    let slice_angle = 360.0 / parts.get().len() as f64;
                    let text_angle = x.angle as f64 + slice_angle / 2.0; 
                    let text_radius = 150.0; 
                    let text_rad = (text_angle - 90.0) * PI / 180.0;
                    let text_x = text_radius * text_rad.cos();
                    let text_y = text_radius * text_rad.sin();
                    
                    view!{
                        <div 
                            class="absolute text-white font-bold text-lg drop-shadow-lg pointer-events-none"
                            style:left=move || format!("calc(50% + {}px)", text_x)
                            style:top=move || format!("calc(50% + {}px)", text_y)
                            style:transform=move || format!("translate(-50%, -50%) rotate({}deg)", text_angle + 90.0)
                        > 
                            {x.name} 
                        </div>
                    }
                }).collect_view()
            }}
        </div>
        <div class="p-2">
            <div><h1 class="text-4xl font-extrabold">SPEED</h1></div>
            <div><SpinButton<u64> value=(interval, set_interval) step_page=10 min=0 max=1000/></div>
            <div><SpinButton<usize> value=speed step_page=1 min=0 max=100/></div>
        </div>
    }
}