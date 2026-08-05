use crate::{CALDAV_SERVER, CALDAV_USER, LogseqCaldav, SyncPrioritise};
use alleged_lib::graph::Graph;
use anyhow::Result;
use argh::FromArgs;
use http::Uri;
use hyper_rustls::HttpsConnectorBuilder;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use libdav::dav::WebDavClient;
use tower_http::auth::AddAuthorization;
use vstorage::caldav::CalDavStorage;

fn default_caldav_uri() -> Uri {
    // This const is a valid URI.
    #[allow(clippy::unwrap_used)]
    Uri::try_from(CALDAV_SERVER).unwrap()
}

fn default_caldav_user() -> String {
    CALDAV_USER.into()
}

const fn default_sync_prioritise() -> SyncPrioritise {
    SyncPrioritise::CalDav
}

/// sync your logseq graph to a calendar
#[derive(FromArgs, PartialEq, Eq, Debug)]
#[argh(subcommand, name = "caldav-sync")]
pub struct CalDavSyncCommand {
    #[allow(clippy::doc_markdown)]
    /// caldav server uri [default: http://127.0.0.1:5232]
    #[argh(option, short = 's', default = "default_caldav_uri()")]
    pub server: Uri,
    /// caldav collection uri
    #[argh(option, short = 'c')]
    pub collection: String,
    /// caldav login [default: user]
    #[argh(option, short = 'u', default = "default_caldav_user()")]
    pub user: String,
    /// whether to prioritise logseq or caldav for syncing task statuses [default: caldav]
    #[argh(option, short = 'p', default = "default_sync_prioritise()")]
    pub prioritise: SyncPrioritise,
}

impl CalDavSyncCommand {
    // FIXME: Async function is not `Send`!
    pub async fn sync(self, password: String, graph: Graph) -> Result<()> {
        let https_connector = HttpsConnectorBuilder::new()
            .with_native_roots()?
            .https_or_http()
            .enable_http1()
            .build();
        let client = AddAuthorization::basic(
            Client::builder(TokioExecutor::new()).build(https_connector),
            &self.user,
            &password,
        );
        let webdav = WebDavClient::new(self.server, client);
        let storage = CalDavStorage::builder(webdav).build().await?;

        let handler = LogseqCaldav::builder()
            .graph(graph)
            .storage(storage)
            .collection(Uri::try_from(self.collection.trim_end_matches('/'))?)
            .build();

        match self.prioritise {
            SyncPrioritise::CalDav => handler.caldav_statuses_to_graph().await?,
            SyncPrioritise::Logseq => unimplemented!("SyncPrioritise::Logseq"),
        }

        handler.graph_items_to_caldav().await?;

        Ok(())
    }
}
