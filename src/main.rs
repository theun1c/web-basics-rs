mod logger;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Video {
    title: AttrValue, 
    url: AttrValue
}

#[derive(Properties, PartialEq)]
struct  VideoListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[component]
fn VideoDetails(VideosDetailsProps { video}: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ &*video.title }</h3>
            <h5>{ &*video.url }</h5>
        </div>
    }
}

#[component]
fn VideoList(VideoListProps { videos , on_click}: &VideoListProps) -> Html {
    let on_selected = |video: &Video| {
        let on_click = on_click.clone();
        let video = video.clone();
        Callback::from(move |_| {
            on_click.emit(video.clone())
        })
    };
    html!{
        for video in videos {
            <p onclick={on_selected(video)}> { format!("{} {}", video.title, video.url) } </p>
        }
    }
}


#[component]
fn App() -> Html {
    let videos = vec![
        Video{
            title: "title 1".into(),
            url: "url 1".into()
        },
        Video{
            title: "title 2".into(),
            url: "url 2".into()
        },
    ];

    let selected_video = use_state(|| None);
    
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video));
        } )
    };


    html! {
        <>
            <h1 class="test">{"Hello, world!"}</h1>

            <div>
                <VideoList {videos} on_click={on_video_select}/> 
            </div>

            if let Some(video) = &*selected_video {
                <VideoDetails video={video.clone()}/>
            }
        </>
    }
}

fn main() { 
    yew::Renderer::<App>::new().render();
}
