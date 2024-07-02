use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub telegram: Option<String>,
    #[prop_or_default]
    pub twitter: Option<String>,
    #[prop_or_default]
    pub github: Option<String>,
    #[prop_or_default]
    pub web: Option<String>,
}

#[function_component(SocialLinks)]
pub fn social_links(props: &Props) -> Html {
    let telegram_link = if let Some(telegram) = &props.telegram {
        html! {
            <a href={telegram.clone()} target="_blank" rel="noopener">
                <i class="fab fa-telegram"></i>
            </a>
        }
    } else {
        html! {}
    };

    let twitter_link = if let Some(twitter) = &props.twitter {
        html! {
            <a href={twitter.clone()} target="_blank" rel="noopener">
                <i class="fab fa-twitter"></i>
            </a>
        }
    } else {
        html! {}
    };

    let github_link = if let Some(github) = &props.github {
        html! {
            <a href={github.clone()} target="_blank" rel="noopener">
                <i class="fab fa-github"></i>
            </a>
        }
    } else {
        html! {}
    };

    let web_link = if let Some(web) = &props.web {
        html! {
            <a href={web.clone()} target="_blank" rel="noopener">
                <i class="fas fa-globe"></i>
            </a>
        }
    } else {
        html! {}
    };

    html! {
        <div class="social-links">
            {telegram_link}
            {twitter_link}
            {github_link}
            {web_link}
        </div>
    }
}
