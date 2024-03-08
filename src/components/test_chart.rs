use crate::model::{CategorizedTest, DetailTrait, TypeTrait};
use charts_rs::PieChart;
use serde::Serialize;
use serde_json::json;
use yew::prelude::*;

#[derive(Serialize)]
struct ChartData {
    name: String,
    value: f32,
}

fn prepare_pie_chart_data<T: TypeTrait, D: DetailTrait>(test: &CategorizedTest<T, D>) -> String {
    let correct_count = test.answers.iter().filter(|a| a.is_correct()).count() as f32;
    let total_answered = test
        .answers
        .iter()
        .filter(|record| record.was_answered)
        .count();
    let incorrect_count = (total_answered - correct_count as usize) as f32;

    let chart_data = json!({
        "legend_show": false,
        "series_list": [
            {
                "name": "Correct",
                "data": [correct_count]
            },
            {
                "name": "Incorrect",
                "data": [incorrect_count]
            }
        ]
    });

    let json_str = chart_data.to_string();
    json_str
}

#[derive(Properties, PartialEq)]
pub struct ChartProps<T: TypeTrait, D: DetailTrait> {
    pub test: CategorizedTest<T, D>,
}

#[function_component(TestChart)]
pub fn test_chart<T: TypeTrait, D: DetailTrait>(props: &ChartProps<T, D>) -> Html {
    let test = &props.test;

    let chart_json = prepare_pie_chart_data(test);

    let mut chart = PieChart::from_json(&chart_json).unwrap();
    chart.series_colors = vec!["#4CAF50".into(), "#FF5252".into()];
    let chart_svg = chart.svg().unwrap();

    let parsed = Html::from_html_unchecked(AttrValue::from(chart_svg));

    let key = test.current_index();

    let total_answered = test
        .answers
        .iter()
        .filter(|record| record.was_answered)
        .count();

    if total_answered > 0 {
        html! {
            <div key={key} class="chart-container" >
                {parsed}
            </div>
        }
    } else {
        html! { <></>}
    }
}
