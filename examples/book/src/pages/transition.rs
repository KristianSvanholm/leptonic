use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageTransition(cx: Scope) -> impl IntoView {
    let (transition_collapse, set_transition_collapse) = create_signal(cx, false);
    let (transition_fade, set_transition_fade) = create_signal(cx, false);
    let (transition_grow, set_transition_grow) = create_signal(cx, false);
    let (transition_slide, set_transition_slide) = create_signal(cx, false);
    let (transition_zoom, set_transition_zoom) = create_signal(cx, false);

    view! { cx,
        <h2>"Transitions"</h2>

        <h2>"Transition - Collapse"</h2>
        <Toggle on=transition_collapse set_on=set_transition_collapse />
        <Collapse show=transition_collapse axis=CollapseAxis::X>
            <Skeleton height="5em">"Collapse"</Skeleton>
        </Collapse>

        <Toggle on=transition_collapse set_on=set_transition_collapse />
        <Collapse show=transition_collapse axis=CollapseAxis::Y>
            <Skeleton height="5em">"Collapse"</Skeleton>
        </Collapse>

        <Separator />

        <h2>"Transition - Fade"</h2>
        <Toggle on=transition_fade set_on=set_transition_fade />
        <Fade inn=Signal::derive(cx, move || transition_fade.get())>
            <Skeleton>"Fade"</Skeleton>
        </Fade>

        <Separator />

        <h2>"Transition - Grow"</h2>
        <Toggle on=transition_grow set_on=set_transition_grow />
        <Grow inn=Signal::derive(cx, move || transition_grow.get())>
            <Skeleton>"Grow"</Skeleton>
        </Grow>

        <Separator />

        <h2>"Transition - Slide"</h2>
        <Toggle on=transition_slide set_on=set_transition_slide />
        <Slide inn=Signal::derive(cx, move || transition_slide.get())>
            <Skeleton>"Slide"</Skeleton>
        </Slide>

        <Separator />

        <h2>"Transition - Zoom"</h2>
        <Toggle on=transition_zoom set_on=set_transition_zoom />
        <Zoom inn=Signal::derive(cx, move || transition_zoom.get())>
            <Skeleton>"Zoom"</Skeleton>
        </Zoom>
    }
}