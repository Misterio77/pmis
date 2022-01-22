use crate::{Paste, Result, Url, Uuid};

use anyhow::anyhow;
use atty::Stream;
use bat::{Input, PrettyPrinter};
use chrono_humanize::HumanTime;
use reqwest::StatusCode;

pub async fn download(api: Url, id: Uuid) -> Result<()> {
    let client = reqwest::Client::new();

    let url = api.join(&format!("/p/{}", id))?;
    let res = client.get(url).send().await?;

    if res.status() == StatusCode::NOT_FOUND {
        return Err(anyhow!("The requested paste does not exist"));
    }

    let paste: Paste = res.error_for_status()?.json().await?;

    if atty::is(Stream::Stdout) {
        let title = paste.title.as_deref().unwrap_or("Untitled");
        let creator = paste.creator;
        let time = HumanTime::from(paste.creation);

        PrettyPrinter::new()
            .input(
                Input::from_bytes(paste.content.as_bytes())
                    .name(title)
                    .title(format!("{}, by u/{}. Created {}.", title, creator, time)),
            )
            .line_numbers(true)
            .header(true)
            .grid(true)
            .theme("base16")
            .print()?;
    } else {
        println!("{}", paste.content.replace("\r", ""));
    }

    Ok(())
}
