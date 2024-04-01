use crate::model::TestResult;
use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HistoryEntryProps {
    pub test_result: TestResult,
}

#[function_component(HistoryEntry)]
pub fn history_entry(props: &HistoryEntryProps) -> Html {
    let code = props.test_result.to_base64();

    html! {
        <div class="history-entry">
            <Link<Route> to={Route::Results { code }}>
                <ul>
                    <li>
                        <div class="history-entry_type">
                            { props.test_result.test_type.to_string() }
                        </div>
                    </li>
                    <li>
                        <div class="history-entry_result">
                            { props.test_result.performance_percentage }
                        </div>
                    </li>
                    <li>
                        <div class="history-entry_date">
                            { props.test_result.date.format("%Y-%m-%d %H:%M:%S").to_string() }
                        </div>
                    </li>
                </ul>
            </Link<Route>>
        </div>
    }
}
