use yew::prelude::*;
use yew_agent::{Agent, AgentLink, Bridged, Context, Dispatched, HandlerId};

#[function_component(App)]
fn app() -> _ {
    html! {
        <>
            <Producer/>
            <Consumer />
        </>
    }
}

#[function_component(Producer)]
fn producer() -> _ {
    let bus = use_ref(move || EventBus::dispatcher());

    let onclick = move |_| (*bus).borrow_mut().send(());

    html! {
        <button {onclick}>{ "Click me" }</button>
    }
}

#[function_component(Consumer)]
fn consumer() -> _ {
    let state = use_state(|| 0);

    {
        let state = state.clone();
        use_ref(move || {
            let producer = EventBus::bridge(Callback::from(move |msg| {
                // In this callback we have moved 'state' which is a
                // `UseStateHandle` which contains an `Rc<u32>`. This
                // `Rc` remains valid even when the hook state changes
                // state because it gets a new `Rc` - this effectively
                // detaches the value in existing `UseStateHandle`s like
                // the one in this closure.
                gloo_console::log!("state: %i\nmsg: %i", *state, msg);
                if *state != msg {
                    state.set(msg);
                }
            }));

            || drop(producer)
        });
    }

    // Note: state must be changing as the counter keeps going up!
    html! {
        <span>{ *state }</span>
    }
}

pub struct EventBus {
    count: u32,
    link: AgentLink<Self>,
    sub: Option<HandlerId>,
}

impl Agent for EventBus {
    type Reach = Context<Self>;
    type Message = ();
    type Input = ();
    type Output = u32;

    fn create(link: yew_agent::AgentLink<Self>) -> Self {
        Self {
            count: 0,
            link,
            sub: None,
        }
    }

    fn update(&mut self, _: Self::Message) {
        todo!()
    }

    fn handle_input(&mut self, _: Self::Input, _: yew_agent::HandlerId) {
        if let Some(handle) = self.sub.as_ref() {
            self.link.respond(*handle, self.count);
            self.count += 1;
        }
    }

    fn connected(&mut self, id: yew_agent::HandlerId) {
        if self.sub.is_none() {
            self.sub = Some(id);
        }
    }
}
fn main() {
    yew::start_app::<App>();
}
