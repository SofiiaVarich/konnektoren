use web_sys::HtmlAudioElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SoundPlayerProps {
    pub sound_url: String,
}

#[function_component(SoundPlayer)]
pub fn sound_player(props: &SoundPlayerProps) -> Html {
    let sound_url = props.sound_url.clone();

    use_effect(move || {
        let audio = HtmlAudioElement::new_with_src(&sound_url).unwrap();
        let _ = audio.play().unwrap();

        || ()
    });

    html! {}
}
