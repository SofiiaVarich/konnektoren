use crate::model::{player::PLAYER_KEY, Player};
use crate::utils::translation::LANGUAGE_KEY;
use gloo_storage::{LocalStorage, Storage};
use gloo_timers::callback::Timeout;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_i18n::use_translation;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let mut i18n = use_translation();
    let selected_language =
        use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));
    let _ = i18n.set_translation_language(&selected_language);

    let player =
        use_state(|| LocalStorage::get::<Player>(PLAYER_KEY).unwrap_or_else(|_| Player::default()));

    let name = use_state(|| player.name.clone());
    let account = use_state(|| player.account.clone().unwrap_or_default());

    let show_save_notification = use_state(|| false);

    let has_changes = {
        let name = name.clone();
        let account = account.clone();
        let initial_name = player.name.clone();
        let initial_account = player.account.clone().unwrap_or_default();
        move || *name != initial_name || *account != initial_account
    };

    let on_name_change = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let on_account_change = {
        let account = account.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            account.set(input.value());
        })
    };

    let on_save = {
        let name = name.clone();
        let account = account.clone();
        let show_save_notification = show_save_notification.clone();
        Callback::from(move |_| {
            let updated_player = Player {
                name: (*name).clone(),
                account: Some((*account).clone()),
            };
            LocalStorage::set(PLAYER_KEY, updated_player).expect("Failed to save player");
            show_save_notification.set(true);
            let show_save_notification = show_save_notification.clone();
            let timeout = Timeout::new(3000, move || {
                show_save_notification.set(false);
            });
            timeout.forget();
        })
    };

    html! {
        <div class="profile-page">
            <h2>{ i18n.t("Player Profile") }</h2>
                <p>{ i18n.t("Use a nickname as your name. It will be used for certificates and displayed on the leaderboard.") }</p>
            <p>{ i18n.t("Connect your Solana account to receive NFTs as rewards for top players.") }</p>
            <div>
                <label for="name">{ "Name (Nickname): " }</label>
                <input id="name" type="text" value={(*name).clone()} oninput={on_name_change} />
            </div>
            <div>
                <label for="account">{ "Solana Account: " }</label>
                <input id="account" type="text" value={(*account).clone()} oninput={on_account_change} placeholder="Solana Account Address" />
            </div>
            if has_changes() {
                <button onclick={on_save}>{ "Save Changes" }</button>
            }
            if *show_save_notification {
                <div class={classes!("notification", show_save_notification.then(|| "show"))}>
                    { "Changes saved successfully!" }
                </div>
            }
        </div>
    }
}
