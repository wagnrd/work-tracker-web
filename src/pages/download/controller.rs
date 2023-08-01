use actix_web::get;
use sailfish::TemplateOnce;

use crate::utils::ssr;

#[derive(TemplateOnce)]
#[template(path = "pages/download/page.html")]
struct DownloadTemplate {
    platform_options: String,
}

struct DownloadOption {
    name: &'static str,
    link_url: &'static str,
}

#[derive(TemplateOnce)]
#[template(path = "pages/download/platform_options.html")]
struct PlatformOptionsTemplate {
    stores: Vec<DownloadOption>,
    installers: Vec<DownloadOption>,
    other_platforms: Vec<&'static str>,
}

#[get("/")]
pub async fn page(request: actix_web::HttpRequest) -> impl actix_web::Responder {
    let stores = vec![
        DownloadOption {
            name: "Flathub",
            link_url: "https://flathub.org",
        },
        DownloadOption {
            name: "Snap Store",
            link_url: "https://snapcraft.io/store",
        },
    ];
    let installers = vec![
        DownloadOption {
            name: "Flatpak",
            link_url: ".",
        },
        DownloadOption {
            name: "Snap",
            link_url: ".",
        },
        DownloadOption {
            name: "AppImage",
            link_url: ".",
        },
        DownloadOption {
            name: "DEB",
            link_url: ".",
        },
        DownloadOption {
            name: "RPM",
            link_url: ".",
        },
    ];
    let other_platforms = vec!["macOS", "Windows"];

    let platform_options = PlatformOptionsTemplate {
        stores,
        installers,
        other_platforms,
    }
    .render_once()
    .unwrap();

    let template = DownloadTemplate { platform_options };
    ssr::render_page(template, request)
}
