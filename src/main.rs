use img_diff_web::component::{
    ImageDiffSumaryInfo, ImageDiffSummary, ImageLoader, ImageSumaryInfo, ImageSummary, SetupIonic,
};
use img_diff_web::image::{diff_image, load_image};
use img_diff_web::web::file_to_binary;
use leptos::*;
use std::rc::Rc;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <SetupIonic />
            <App />
        }
    });
}

#[component]
fn App() -> impl IntoView {
    let (image1, set_image1) = create_signal::<ImageSumaryInfo>(None);
    let (image2, set_image2) = create_signal::<ImageSumaryInfo>(None);
    let (preffered_format1, set_preffered_format1) =
        create_signal::<Option<image::ImageFormat>>(None);
    let (preffered_format2, set_preffered_format2) =
        create_signal::<Option<image::ImageFormat>>(None);

    let (file1, set_file1) = create_signal::<Option<web_sys::File>>(None);
    let (file2, set_file2) = create_signal::<Option<web_sys::File>>(None);

    create_effect(move |_: Option<()>| {
        let preffered_format = preffered_format1();
        let file = file1();
        set_image1(None);
        let file = match file {
            Some(file) => file,
            None => return,
        };
        file_to_binary(file.clone(), move |data| {
            let info = load_image(preffered_format, file.clone(), data);
            set_image1(Some(info.map(Rc::new)));
        });
    });

    create_effect(move |_: Option<()>| {
        let preffered_format = preffered_format2();
        let file = file2();
        set_image2(None);
        let file = match file {
            Some(file) => file,
            None => return,
        };
        file_to_binary(file.clone(), move |data| {
            let info = load_image(preffered_format, file.clone(), data);
            set_image2(Some(info.map(Rc::new)));
        });
    });

    let on_image1_selected = {
        move |file: web_sys::File| {
            set_file1(Some(file));
        }
    };
    let on_image2_selected = {
        move |file: web_sys::File| {
            set_file2(Some(file));
        }
    };

    let (diff_info, set_diff_info) = create_signal::<ImageDiffSumaryInfo>(None);
    create_effect(move |_: Option<()>| {
        let image1 = image1();
        let image2 = image2();

        set_diff_info(None);
        let image1 = match image1.and_then(|e| e.ok()) {
            Some(image1) => image1,
            None => return,
        };
        let image2 = match image2.and_then(|e| e.ok()) {
            Some(image2) => image2,
            None => return,
        };
        let image1 = &image1.as_ref().image;
        let image2 = &image2.as_ref().image;
        let diff_info = diff_image(image1, image2);
        set_diff_info(Some(diff_info.map(Rc::new)));
    });

    view! {
        <ion-app mode="ios">
            <ion-content>
                <ion-text style:text-align="center">
                    <h1>Image Diffing Tool</h1>
                </ion-text>
                <ion-text style:text-align="center">
                    <p>Select two images to compare. Comparison is done offline in the browser.</p>
                </ion-text>
                <ion-text style:text-align="center">
                    <a
                        href="https://github.com/luma-dev/img-diff-web"
                        target="_blank"
                        style:display="block"
                    >
                        Source Code (GitHub)
                    </a>
                </ion-text>
                <div
                    style:margin-top="8px"
                    style:display="flex"
                    style:flex-wrap="wrap"
                    style:gap="10px"
                    style:flex-direction="row"
                    style:justify-content="center"
                >
                    <ImageLoader
                        on_image_selected=on_image1_selected
                        on_format_selected=set_preffered_format1
                    />
                    <ImageLoader
                        on_image_selected=on_image2_selected
                        on_format_selected=set_preffered_format2
                    />
                </div>
                <div
                    style:display="flex"
                    style:flex-wrap="wrap"
                    style:gap="10px"
                    style:flex-direction="row"
                    style:justify-content="center"
                >
                    <ImageSummary info=move || image1().clone() image_name="First Image".into() />
                    <ImageSummary info=move || image2().clone() image_name="Second Image".into() />
                </div>
                <div
                    style:display="flex"
                    style:flex-wrap="wrap"
                    style:gap="10px"
                    style:flex-direction="row"
                    style:justify-content="center"
                >
                    <ImageDiffSummary info=diff_info />
                </div>
            </ion-content>
        </ion-app>
    }
}
