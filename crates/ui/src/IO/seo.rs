use crate::impls::state::State;
use dioxus::prelude::*;
use dioxus_fullstack::response::{IntoResponse, Response};

use crate::root::Route;

#[get("/sitemap.xml", state: State)]
pub async fn get_sitemap() -> ServerFnResult<Response> {
    let mut sitemap = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE urlset PUBLIC "-//sitemaps.org//DTD Sitemap 0.9//EN" "http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.dtd">
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
    );

    let site_url = state.0.config.web.site_url.as_str();
    let all_routes = Route::static_routes();
    for route in all_routes {
        if route.is_no_index() {
            continue;
        }

        let url = format!("{}{}", site_url, route);
        let escaped_url = askama_escape::escape(&url, askama_escape::Html);
        sitemap.push_str(&format!(
            r#"<url>  
                <loc>{}</loc>  
            </url>
            "#,
            escaped_url
        ));
    }

    sitemap.push_str("\n</urlset>");

    Ok((
        [("content-type", "application/xml; charset=utf-8")],
        sitemap,
    )
        .into_response())
}

#[get("/robots.txt", state: State)]
pub async fn get_robots() -> ServerFnResult<Response> {
    let sitemap_url = format!("{}/sitemap.xml", state.0.config.web.site_url);
    let content = format!(
        r#"# As a condition of accessing this website, you agree to abide by the following  
# content signals:  

# (a)  If a Content-Signal = yes, you may collect content for the corresponding  
#      use.  
# (b)  If a Content-Signal = no, you may not collect content for the  
#      corresponding use.  
# (c)  If the website operator does not include a Content-Signal for a  
#      corresponding use, the website operator neither grants nor restricts  
#      permission via Content-Signal with respect to the corresponding use.  

# The content signals and their meanings are:  

# search:   building a search index and providing search results (e.g., returning  
#           hyperlinks and short excerpts from your website's contents). Search does not  
#           include providing AI-generated search summaries.  
# ai-input: inputting content into one or more AI models (e.g., retrieval  
#           augmented generation, grounding, or other real-time taking of content for  
#           generative AI search answers).  
# ai-train: training or fine-tuning AI models.  

# ANY RESTRICTIONS EXPRESSED VIA CONTENT SIGNALS ARE EXPRESS RESERVATIONS OF  
# RIGHTS UNDER ARTICLE 4 OF THE EUROPEAN UNION DIRECTIVE 2019/790 ON COPYRIGHT  
# AND RELATED RIGHTS IN THE DIGITAL SINGLE MARKET.  

# BEGIN Cloudflare Managed content  

User-agent: *  
Content-Signal: search=yes,ai-train=no  
Allow: /  

User-agent: Amazonbot  
Disallow: /  

User-agent: Applebot-Extended  
Disallow: /  

User-agent: Bytespider  
Disallow: /  

User-agent: CCBot  
Disallow: /  

User-agent: ClaudeBot  
Disallow: /  

User-agent: CloudflareBrowserRenderingCrawler  
Disallow: /  

User-agent: Google-Extended  
Disallow: /  

User-agent: GPTBot  
Disallow: /  

User-agent: meta-externalagent  
Disallow: /  

# END Cloudflare Managed content  
Sitemap: {}
        "#,
        sitemap_url
    );
    Ok(([("content-type", "text/plain; charset=utf-8")], content).into_response())
}
