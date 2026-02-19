use std::time::Duration;

use dioxus::prelude::*;
use futures_timer::Delay;

const WORK_SECONDS: i32 = 25 * 60;
const BREAK_SECONDS: i32 = 5 * 60;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut running = use_signal(|| false);
    let mut on_break = use_signal(|| false);
    let mut seconds_left = use_signal(|| WORK_SECONDS);
    let mut completed_sessions = use_signal(|| 0_u32);

    use_future(move || async move {
        loop {
            Delay::new(Duration::from_secs(1)).await;

            if !running() {
                continue;
            }

            if seconds_left() > 0 {
                seconds_left -= 1;
                continue;
            }

            if on_break() {
                on_break.set(false);
                seconds_left.set(WORK_SECONDS);
            } else {
                completed_sessions += 1;
                on_break.set(true);
                seconds_left.set(BREAK_SECONDS);
            }
        }
    });

    let total_seconds = seconds_left();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    rsx! {
        style { {APP_STYLE} }
        main { class: "container",
            h1 { "Pomodoro" }
            p { class: "mode", if on_break() { "Break" } else { "Focus" } }
            p { class: "timer", "{minutes:02}:{seconds:02}" }
            p { class: "sessions", "Completed sessions: {completed_sessions}" }
            div { class: "actions",
                button {
                    onclick: move |_| running.set(!running()),
                    if running() { "Pause" } else { "Start" }
                }
                button {
                    onclick: move |_| {
                        running.set(false);
                        on_break.set(false);
                        seconds_left.set(WORK_SECONDS);
                    },
                    "Reset"
                }
                button {
                    onclick: move |_| {
                        running.set(false);
                        let next_is_break = !on_break();
                        on_break.set(next_is_break);
                        seconds_left.set(if next_is_break { BREAK_SECONDS } else { WORK_SECONDS });
                    },
                    "Skip"
                }
            }
        }
    }
}

const APP_STYLE: &str = r#"
:root {
    color-scheme: dark;
    font-family: Inter, system-ui, -apple-system, sans-serif;
}

body {
    margin: 0;
    background: radial-gradient(circle at top, #1f2937, #0b1020 50%);
    min-height: 100vh;
}

.container {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    color: #e5e7eb;
}

.mode {
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: #93c5fd;
}

.timer {
    font-size: clamp(3rem, 12vw, 6rem);
    font-variant-numeric: tabular-nums;
    font-weight: 700;
    margin: 0;
}

.sessions {
    color: #cbd5e1;
}

.actions {
    display: flex;
    gap: 0.75rem;
}

button {
    border: 0;
    border-radius: 0.75rem;
    padding: 0.65rem 1.2rem;
    background: #2563eb;
    color: white;
    cursor: pointer;
    font-weight: 600;
}

button:hover {
    background: #1d4ed8;
}
"#;
