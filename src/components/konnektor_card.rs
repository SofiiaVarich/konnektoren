use crate::model::KonnektorDetail;
use yew::prelude::*;
use yew_bootstrap::component::card::*;

#[derive(Properties, PartialEq)]
pub struct ModelProps {
    pub detail: KonnektorDetail,
}

pub struct KonnektorCard {
    detail: KonnektorDetail,
}

pub enum Msg {}

impl Component for KonnektorCard {
    type Message = Msg;
    type Properties = ModelProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            detail: ctx.props().detail.clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Card>
                <CardBody>
                    <CardTitle>{ &*self.detail.konnektor }</CardTitle>
                    <CardText>{  &*self.detail.example }</CardText>
                </CardBody>
            </Card>
        }
    }
}
