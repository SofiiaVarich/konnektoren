use crate::model::DetailTrait;
use yew::prelude::*;
use yew_bootstrap::component::card::*;

#[derive(Properties, PartialEq)]
pub struct ModelProps<D: DetailTrait> {
    pub detail: D,
}

pub struct CarouselCard<D: DetailTrait> {
    detail: D,
}

pub enum Msg {}

impl<D: DetailTrait + 'static> Component for CarouselCard<D> {
    type Message = Msg;
    type Properties = ModelProps<D>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            detail: ctx.props().detail.clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Card class="text-center">
                <CardBody>
                    <CardTitle class="mb-4">{ &*self.detail.get_detail() }</CardTitle>
                    <CardText>{ &*self.detail.get_example() }</CardText>
                </CardBody>
            </Card>
        }
    }
}
