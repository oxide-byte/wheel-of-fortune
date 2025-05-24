mod wheel_of_fortune;

use wheel_of_fortune::WheelOfFortune;
use leptos::prelude::*;
fn main() {
    mount_to_body(|| view! {
        <WheelOfFortune/>
    })
}