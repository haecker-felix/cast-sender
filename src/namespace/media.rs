use crate::{Image, Payload, Volume};

// https://developers.google.com/cast/docs/reference/web_receiver/cast.framework.messages
// https://developers.google.com/cast/docs/reference/web_receiver/index-all

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum Media {
    // Request
    GetStatus(GetStatusRequestData),
    Load(LoadRequestData),
    Pause(RequestData),
    Stop(RequestData),
    Play(RequestData),
    SkipAd(RequestData),
    PlayAgain(RequestData),
    Seek(SeekRequestData),
    SetPlaybackRate(SetPlaybackRateRequestData),
    SetVolume(VolumeRequestData),
    EditTracksInfo(EditTracksInfoRequestData),
    EditAudioTracks(EditAudioTracksRequestData),
    Precache(PrecacheRequestData),
    Preload(PreloadRequestData),
    QueueLoad(QueueLoadRequestData),
    QueueInsert(QueueInsertRequestData),
    QueueUpdate(QueueUpdateRequestData),
    QueueRemove(QueueRemoveRequestData),
    QueueReorder(QueueReorderRequestData),
    QueueNext(RequestData),
    QueuePrev(RequestData),
    QueueGetItemRange(FetchItemsRequestData),
    QueueGetItems(GetItemsInfoRequestData),
    QueueGetItemIds(RequestData),
    QueueShuffle(RequestData),
    SetCredentials(SetCredentialsRequestData),
    LoadByEntity(LoadByEntityRequestData),
    UserAction(UserActionRequestData),
    DisplayStatus(DisplayStatusRequestData),
    FocusState(FocusStateRequestData),
    StoreSession(StoreSessionRequestData),
    ResumeSession(ResumeSessionRequestData),

    // Response
    MediaStatus(ResponseData<MediaStatus>),
    CloudStatus(ResponseData<CloudMediaStatus>),
    QueueChange(ResponseData<QueueChange>),
    QueueItems(ResponseData<ItemsInfo>),
    QueueItemIds(ResponseData<QueueIds>),
    SessionState(StoreSessionResponseData),

    InvalidRequest(ErrorResponseData),
    InvalidPlayerState,
    LoadFailed,
    LoadCancelled,
}

impl Into<Payload> for Media {
    fn into(self) -> Payload {
        Payload::Media(self.clone())
    }
}

// REQUEST DATA -------------------------------------------------

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct DisplayStatusRequestData {}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct EditAudioTracksRequestData {
    pub is_suggested_language: Option<bool>,
    pub language: Option<String>,
    pub media_session_id: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct EditTracksInfoRequestData {
    pub active_track_ids: Option<Vec<i32>>,
    pub enable_text_tracks: Option<bool>,
    pub is_suggested_language: Option<bool>,
    pub language: Option<String>,
    pub media_session_id: Option<i32>,
    pub text_track_style: Option<TextTrackStyle>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct FetchItemsRequestData {
    pub item_id: i32,
    pub media_session_id: Option<i32>,
    pub next_count: i32,
    pub prev_count: i32,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct FocusStateRequestData {
    pub state: FocusState,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct GetItemsInfoRequestData {
    pub item_ids: Vec<i32>,
    pub media_session_id: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct GetStatusRequestData {
    pub media_session_id: Option<i32>,
    pub options: Option<GetStatusOptions>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct LoadByEntityRequestData {
    pub entity: String,
    pub shuffle: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct LoadRequestData {
    pub active_track_ids: Option<Vec<i32>>,
    pub autoplay: Option<bool>,
    pub credentials: Option<String>,
    pub credentials_type: Option<String>,
    pub current_time: Option<f64>,
    pub media: MediaInformation,
    pub media_session_id: Option<i32>,
    pub playback_rate: Option<f64>,
    pub queue_data: Option<QueueData>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct PrecacheRequestData {
    pub active_track_ids: Option<Vec<i32>>,
    pub autoplay: Option<bool>,
    pub credentials: Option<String>,
    pub credentials_type: Option<String>,
    pub current_time: Option<f64>,
    pub media: MediaInformation,
    pub media_session_id: Option<i32>,
    pub playback_rate: Option<f64>,
    pub precache_data: Option<String>,
    pub queue_data: Option<QueueData>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct PreloadRequestData {
    pub active_track_ids: Option<Vec<i32>>,
    pub autoplay: Option<bool>,
    pub credentials: Option<String>,
    pub credentials_type: Option<String>,
    pub current_time: Option<f64>,
    pub item_id: i32,
    pub media: MediaInformation,
    pub media_session_id: Option<i32>,
    pub playback_rate: Option<f64>,
    pub queue_data: Option<QueueData>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct QueueInsertRequestData {
    pub current_item_id: Option<i32>,
    pub current_item_index: Option<i32>,
    pub current_time: Option<f64>,
    pub insert_before: Option<bool>,
    pub items: Vec<QueueItem>,
    pub media_session_id: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct QueueLoadRequestData {
    pub current_time: Option<f64>,
    pub items: Vec<QueueItem>,
    pub media_session_id: Option<i32>,
    pub repeat_mode: Option<RepeatMode>,
    pub start_index: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct QueueRemoveRequestData {
    pub current_item_id: Option<i32>,
    pub current_time: Option<f64>,
    pub item_ids: Vec<i32>,
    pub media_session_id: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct QueueReorderRequestData {
    pub current_item_id: Option<i32>,
    pub current_time: Option<f64>,
    pub insert_before: Option<bool>,
    pub item_ids: Vec<i32>,
    pub media_session_id: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct QueueUpdateRequestData {
    pub current_item_id: Option<i32>,
    pub current_time: Option<f64>,
    pub items: Option<Vec<QueueItem>>,
    pub jump: Option<i32>,
    pub media_session_id: Option<i32>,
    pub repeat_mode: Option<RepeatMode>,
    pub shuffle: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct RefreshCredentialsRequestData {}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct RequestData {
    pub media_session_id: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct ResumeSessionRequestData {
    pub session_state: SessionState,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct SeekRequestData {
    pub current_time: Option<f64>,
    pub media_session_id: Option<i32>,
    pub relative_time: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct SetCredentialsRequestData {
    pub credentials: String,
    pub for_request: Option<i32>,
    pub is_likely_3p_account_linked_user: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct SetPlaybackRateRequestData {
    pub media_session_id: Option<i32>,
    pub playback_rate: Option<u32>,
    pub relative_playback_rate: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct StoreSessionRequestData {}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct UserActionRequestData {
    pub clear: Option<bool>,
    pub user_action: UserAction,
    pub user_action_context: Option<UserActionContext>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct VolumeRequestData {
    pub media_session_id: Option<i32>,
    pub volume: Volume,
}

// RESPONSE DATA -------------------------------------------------

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData<T> {
    pub status: Vec<T>,
}

impl<T: Clone + Default> ResponseData<T> {
    pub fn first(&self) -> T {
        self.status.first().cloned().unwrap()
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponseData {
    pub reason: ErrorReason,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct StoreSessionResponseData {
    pub session_state: SessionState,
}

// STRUCTS ------------------------------------------------------

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct AduioTrackInfo {
    pub audio_codec: Option<String>,
    pub num_audio_channels: Option<i32>,
    pub spatial_audio: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
#[serde(rename = "AUDIOBOOK_CHAPTER")]
#[builder(setter(strip_option, into), default)]
pub struct AudiobookChapterMediaMetadata {
    pub book_title: Option<String>,
    pub chapter_number: Option<i32>,
    pub chapter_title: Option<String>,
    pub images: Option<Vec<Image>>,
    pub subtitle: Option<String>,
    pub title: Option<String>,
}

impl Into<MediaMetadata> for AudiobookChapterMediaMetadata {
    fn into(self) -> MediaMetadata {
        MediaMetadata {
            metadata_type: MetadataType::AudiobookChapter(self),
            ..Default::default()
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct AudiobookContainerMetadata {
    pub authors: Option<Vec<String>>,
    pub narrators: Option<Vec<String>>,
    pub publisher: Option<String>,
    pub release_date: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct Break {
    pub break_clip_ids: Vec<String>,
    pub duration: Option<f64>,
    pub id: String,
    pub is_embedded: Option<bool>,
    pub is_watched: bool,
    pub position: f64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct BreakClip {
    pub click_through_url: Option<String>,
    pub content_id: Option<String>,
    pub content_type: Option<String>,
    pub content_url: Option<String>,
    pub duration: Option<f64>,
    pub hls_segment_format: Option<HlsSegmentFormat>,
    pub id: String,
    pub poster_url: Option<String>,
    pub title: Option<String>,
    pub vast_ads_request: Option<VastAdsRequest>,
    pub when_skippable: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct BreakStatus {
    pub break_clip_id: Option<String>,
    pub break_id: Option<String>,
    pub current_break_clip_time: Option<f64>,
    pub current_break_time: Option<f64>,
    pub when_skippable: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct CloudMediaStatus {}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct ContainerMetadata {
    pub container_duration: Option<f64>,
    pub container_images: Option<Vec<Image>>,
    pub container_type: ContainerType,
    pub sections: Option<Vec<MediaMetadata>>,
    pub title: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct ExtendedMediaStatus {
    pub player_state: ExtendedPlayerState,
    pub media: Option<MediaInformation>,
    pub media_session_id: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
#[serde(rename = "GENERIC")]
#[builder(setter(strip_option, into), default)]
pub struct GenericMediaMetadata {
    pub images: Option<Vec<Image>>,
    pub release_date: Option<String>,
    pub subtitle: Option<String>,
    pub title: Option<String>,
}

impl Into<MediaMetadata> for GenericMediaMetadata {
    fn into(self) -> MediaMetadata {
        MediaMetadata {
            metadata_type: MetadataType::Generic(self),
            ..Default::default()
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct ItemsInfo {
    pub items: Option<Vec<QueueItem>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct LiveSeekableRange {
    pub end: Option<f64>,
    pub is_live_done: Option<bool>,
    pub is_moving_window: Option<bool>,
    pub start: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct MediaInformation {
    pub atv_entity: Option<String>,
    pub break_clips: Option<Vec<BreakClip>>,
    pub breaks: Option<Vec<Break>>,
    pub content_id: String,
    pub content_type: String,
    pub duration: Option<f64>,
    pub entity: Option<String>,
    pub hls_segment_format: Option<HlsSegmentFormat>,
    pub hls_video_segment_format: Option<HlsVideoSegmentFormat>,
    pub metadata: Option<MediaMetadata>,
    pub start_absolute_time: Option<f64>,
    pub stream_type: StreamType,
    pub text_track_style: Option<TextTrackStyle>,
    pub tracks: Option<Vec<Track>>,
    pub user_action_states: Option<Vec<UserActionState>>,
    pub vmap_ads_request: Option<VastAdsRequest>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct MediaMetadata {
    #[serde(flatten)]
    pub metadata_type: MetadataType,
    pub poster_url: Option<String>,
    pub queue_item_id: Option<i32>,
    pub section_duration: Option<f64>,
    pub section_start_absolute_time: Option<f64>,
    pub section_start_time_in_container: Option<f64>,
    pub section_start_time_in_media: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct MediaStatus {
    pub active_track_ids: Option<Vec<i32>>,
    pub break_status: Option<BreakStatus>,
    pub current_item_id: Option<i32>,
    pub current_time: f64,
    pub extended_status: Option<ExtendedMediaStatus>,
    pub idle_reason: Option<IdleReason>,
    pub items: Option<Vec<QueueItem>>,
    pub live_seekable_range: Option<LiveSeekableRange>,
    pub loading_item_id: Option<i32>,
    pub media: Option<MediaInformation>,
    pub media_session_id: i32,
    pub playback_rate: i32,
    pub player_state: PlayerState,
    pub preloaded_item_id: Option<i32>,
    pub queue_data: Option<QueueData>,
    pub repeat_mode: Option<RepeatMode>,
    pub supported_media_commands: Command,
    pub video_info: Option<VideoInformation>,
    pub volume: Volume,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
#[serde(rename = "MOVIE")]
#[builder(setter(strip_option, into), default)]
pub struct MovieMediaMetadata {
    pub images: Option<Vec<Image>>,
    pub release_date: Option<String>,
    pub studio: Option<String>,
    pub subtitle: Option<String>,
    pub title: Option<String>,
}

impl Into<MediaMetadata> for MovieMediaMetadata {
    fn into(self) -> MediaMetadata {
        MediaMetadata {
            metadata_type: MetadataType::Movie(self),
            ..Default::default()
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
#[serde(rename = "MUSIC_TRACK")]
#[builder(setter(strip_option, into), default)]
pub struct MusicTrackMediaMetadata {
    pub album_artist: Option<String>,
    pub album_name: Option<String>,
    pub artist: Option<String>,
    pub composer: Option<String>,
    pub disc_number: Option<i32>,
    pub images: Option<Vec<Image>>,
    pub release_date: Option<String>,
    pub secondary_image: Option<Image>,
    pub title: Option<String>,
    pub track_number: Option<i32>,
}

impl Into<MediaMetadata> for MusicTrackMediaMetadata {
    fn into(self) -> MediaMetadata {
        MediaMetadata {
            metadata_type: MetadataType::MusicTrack(self),
            ..Default::default()
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
#[serde(rename = "PHOTO")]
#[builder(setter(strip_option, into), default)]
pub struct PhotoMediaMetadata {
    pub artist: Option<String>,
    pub creation_date_time: Option<String>,
    pub height: Option<u32>,
    pub images: Option<Vec<Image>>,
    pub latitude: Option<i32>,
    pub location: Option<String>,
    pub longitude: Option<i32>,
    pub width: Option<u32>,
}

impl Into<MediaMetadata> for PhotoMediaMetadata {
    fn into(self) -> MediaMetadata {
        MediaMetadata {
            metadata_type: MetadataType::Photo(self),
            ..Default::default()
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct QueueChange {
    pub change_type: Option<QueueChangeType>,
    pub insert_before: Option<i32>,
    pub item_ids: Option<Vec<i32>>,
    pub reorder_item_ids: Option<Vec<i32>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct QueueData {
    pub container_metadata: Option<ContainerMetadata>,
    pub description: Option<String>,
    pub entity: Option<String>,
    pub id: Option<String>,
    pub items: Option<Vec<QueueItem>>,
    pub name: Option<String>,
    pub queue_type: Option<QueueType>,
    pub repeat_mode: Option<RepeatMode>,
    pub shuffle: Option<bool>,
    pub start_index: Option<i32>,
    pub start_time: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct QueueIds {
    pub item_ids: Option<Vec<i32>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct SeekableRange {
    pub end: Option<f64>,
    pub start: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct SessionState {
    pub load_request_data: Option<LoadRequestData>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct QueueItem {
    pub active_track_ids: Option<Vec<i32>>,
    pub autoplay: Option<bool>,
    pub item_id: Option<i32>,
    pub media: Option<MediaInformation>,
    pub playback_duration: Option<f64>,
    pub preload_time: Option<f64>,
    pub start_time: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct TextTrackStyle {
    pub background_color: Option<String>,
    pub edge_color: Option<String>,
    pub edge_type: Option<TextTrackEdgeType>,
    pub font_family: Option<String>,
    pub font_generic_familiy: Option<TextTrackFontGenericFamily>,
    pub font_scale: Option<i32>,
    pub font_style: Option<TextTrackFontStyle>,
    pub foreground_color: Option<String>,
    pub window_color: Option<String>,
    pub window_rounded_corner_radius: Option<f64>,
    pub window_type: Option<TextTrackWindowType>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct Track {
    pub audio_track_info: Option<AduioTrackInfo>,
    pub is_inband: Option<bool>,
    pub language: Option<String>,
    pub name: Option<String>,
    pub roles: Option<Vec<String>>,
    pub subtype: Option<TextTrackType>,
    pub track_content_id: Option<String>,
    pub track_content_type: Option<CaptionMimeType>,
    pub track_id: i32,
    #[serde(rename = "type")]
    pub type_: TrackType,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct TracksInfo {
    pub active_track_ids: Option<Vec<i32>>,
    pub language: Option<String>,
    pub text_track_style: Option<TextTrackStyle>,
    pub tracks: Option<Vec<Track>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
#[serde(rename = "TV_SHOW")]
#[builder(setter(strip_option, into), default)]
pub struct TvShowMediaMetadata {
    pub episode: Option<i32>,
    pub episode_title: Option<String>,
    pub images: Option<Vec<Image>>,
    pub original_airdate: Option<String>,
    pub season: Option<i32>,
    pub series_title: Option<String>,
    pub title: Option<String>,
}

impl Into<MediaMetadata> for TvShowMediaMetadata {
    fn into(self) -> MediaMetadata {
        MediaMetadata {
            metadata_type: MetadataType::TvShow(self),
            ..Default::default()
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct UserActionState {
    pub user_action: UserAction,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct VastAdsRequest {
    pub ads_response: Option<String>,
    pub ad_tag_url: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct VideoInformation {
    pub hdr_type: HdrType,
    pub height: i32,
    pub width: i32,
}

// ENUMS --------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CaptionMimeType {
    #[default]
    Cea608,
    Ttml,
    Vtt,
    TtmlMp3,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CommandOld {
    #[default]
    Pause,
    Seek,
    StreamVolume,
    StreamMute,
    AllBasicMedia,
    QueueNext,
    QueuePrev,
    QueueShuffle,
    QueueRepeatAll,
    QueueRepeatOne,
    QueueRepeat,
    SkipAd,
    EditTracks,
    PlaybackRate,
    Like,
    Dislike,
    Follow,
    Unfollow,
    StreamTransfer,
    Lyrics,
}

bitflags::bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Default)]
    pub struct Command: u32 {
        const Pause = 1;
        const Seek = 2;
        const StreamVolume = 4;
        const StreamMute = 8;
        const AllBasicMedia = 12303;
        const QueueNext = 64;
        const QueuePrev = 128;
        const QueueShuffle = 256;
        const QueueRepeatAll = 1024;
        const QueueRepeatOne = 2048;
        const QueueRepeat = 3072;
        const SkipAd = 512;
        const EditTracks = 4096;
        const PlaybackRate = 8192;
        const Like = 16384;
        const Dislike = 32768;
        const Follow = 65536;
        const Unfollow = 131072;
        const StreamTransfer = 262144;
        const Lyrics = 524288;
    }
}

impl serde::Serialize for Command {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

impl<'de> serde::Deserialize<'de> for Command {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bits = u32::deserialize(deserializer)?;
        Command::from_bits(bits).ok_or_else(|| serde::de::Error::custom("invalid bitflags value"))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContainerType {
    #[default]
    GenericContainer,
    AudiobookContainer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContentFilteringMode {
    #[default]
    FilterExplicit,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorReason {
    #[default]
    None,
    InvalidCommand,
    InvalidParams,
    InvalidMediaSessionId,
    InvalidRequestId,
    SkipLimitReached,
    NotSupported,
    LanguageNotSupported,
    EndOfQueue,
    DuplicateRequestId,
    VideoDeviceRequired,
    PremiumAccountRequired,
    AppError,
    AuthenticationExpired,
    ConcurrentStreamLimit,
    ParentalControlRestricted,
    ContentFiltered,
    NotAvailableInRegion,
    ContentAlreadyPlaying,
    InvalidRequest,
    GenericLoadError,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExtendedPlayerState {
    #[default]
    Loading,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FocusState {
    #[default]
    InFocus,
    NotInFocus,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GetStatusOptions {
    #[default]
    NoMetadata,
    NoQueueItems,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HdrType {
    #[default]
    Sdr,
    Hdr,
    Dv,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HlsSegmentFormat {
    #[default]
    Aac,
    Ac3,
    Mp3,
    Ts,
    TsAac,
    EAc3,
    Fmp4,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HlsVideoSegmentFormat {
    #[default]
    Mpeg2Ts,
    Fmp4,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IdleReason {
    #[default]
    Cancelled,
    Interrupted,
    Finished,
    Error,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaCategory {
    #[default]
    Audio,
    Video,
    Image,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum MetadataType {
    Generic(GenericMediaMetadata),
    Movie(MovieMediaMetadata),
    TvShow(TvShowMediaMetadata),
    MusicTrack(MusicTrackMediaMetadata),
    Photo(PhotoMediaMetadata),
    AudiobookChapter(AudiobookChapterMediaMetadata),
}

impl Default for MetadataType {
    fn default() -> Self {
        Self::Generic(GenericMediaMetadata::default())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlayerState {
    #[default]
    Idle,
    Playing,
    Paused,
    Buffering,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QueueChangeType {
    #[default]
    Insert,
    Remove,
    ItemsChange,
    Update,
    NoChange,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QueueType {
    #[default]
    Album,
    Playlist,
    Audiobook,
    RadioStation,
    PodcastSeries,
    TvSeries,
    VideoPlaylist,
    LiveTv,
    Movie,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RepeatMode {
    #[default]
    RepeatOff,
    RepeatAll,
    RepeatSingle,
    RepeatAllAndShuffle,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SeekResumeState {
    #[default]
    PlaybackStart,
    PlaybackPause,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StreamingProtocolType {
    #[default]
    Unknown,
    MpegDash,
    Hls,
    SmoothStreaming,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StreamType {
    #[default]
    None,
    Buffered,
    Live,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextTrackEdgeType {
    #[default]
    None,
    Outline,
    DropShadow,
    Raised,
    Depressed,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextTrackFontGenericFamily {
    #[default]
    SansSerif,
    MonospacedSansSerif,
    Serif,
    MonospacedSerif,
    Casual,
    Cursive,
    SmallCapitals,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextTrackFontStyle {
    #[default]
    Normal,
    Bold,
    BoldItalic,
    Italic,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextTrackType {
    #[default]
    Subtitles,
    Captions,
    Descriptions,
    Chapters,
    Metadata,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextTrackWindowType {
    #[default]
    None,
    Normal,
    RoundedCorners,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrackType {
    #[default]
    Text,
    Audio,
    Video,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserAction {
    #[default]
    Like,
    Dislike,
    Follow,
    Unfollow,
    Flag,
    SkipAd,
    Lyrics,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserActionContext {
    #[default]
    UnknownContext,
    Track,
    Album,
    Artist,
    Playlist,
    Episode,
    Series,
    Movie,
    Channel,
    Team,
    Player,
    Coach,
}
