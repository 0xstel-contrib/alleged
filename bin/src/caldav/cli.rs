use crate::{CALDAV_SERVER, CALDAV_USER, SyncPrioritise};
use argh::FromArgs;
use http::Uri;

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
