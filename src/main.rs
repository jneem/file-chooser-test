use ashpd::desktop::file_chooser::{AsyncFileChooserProxy, SelectedFiles};
use ashpd::zbus::{azync::Connection, fdo};
use ashpd::Response;

#[tokio::main(flavor = "current_thread")]
async fn main() -> fdo::Result<()> {
    let connection = Connection::new_session().await?;
    let proxy = AsyncFileChooserProxy::new(&connection);
    let reply = proxy
        .open_file(Default::default(), "Title", Default::default())
        .await?;

    reply
        .connect_response(|r: Response<SelectedFiles>| {
            // Do something...
            Box::pin(futures::future::ok(()))
        })
        .await?;
    dbg!("done connecting response");

    while reply.next_signal().await?.is_some() {}

    Ok(())
}
