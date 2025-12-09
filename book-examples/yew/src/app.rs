use yew::prelude::*;

#[component]
pub fn App() -> Html {
    #[allow(unused_mut)]
    let mut children: Vec<Html> = vec![];

    #[cfg(feature = "icons")]
    {
        use crate::icons::Icons;
        children.push(html! { <Icons /> });
    }

    html! { <div class="w-full h-full flex justify-center items-start">{ children }</div> }
}
