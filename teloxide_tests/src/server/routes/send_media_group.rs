use crate::mock_bot::State;
use crate::server::{SentMediaGroup, MESSAGES, RESPONSES};
use crate::{
    MockMessageAudio, MockMessageDocument, MockMessagePhoto, MockMessageVideo, MockPhotoSize,
    MockVideo,
};
use std::collections::HashMap;

use actix_multipart::Multipart;
use actix_web::Responder;
use actix_web::{error::ErrorBadRequest, web};
use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;
use serde_json::Value;
use teloxide::types::{Me, Message, MessageEntity, MessageId, ParseMode, ReplyParameters, Seconds};

use crate::server::routes::check_if_message_exists;

use super::{
    get_raw_multipart_fields, make_telegram_result, Attachment, BodyChatId, MediaGroupInputMedia,
    MediaGroupInputMediaAudio, MediaGroupInputMediaDocument, MediaGroupInputMediaPhoto,
    MediaGroupInputMediaVideo,
};

pub async fn send_media_group(
    mut payload: Multipart,
    me: web::Data<Me>,
    state: web::Data<State>,
) -> impl Responder {
    let (fields, attachments) = get_raw_multipart_fields(&mut payload).await;
    let body = SendMediaGroupBody::serialize_raw_fields(&fields, &attachments).unwrap();
    if body.media.len() > 10 {
        return ErrorBadRequest("Too many media items").into();
    } else if body.media.len() < 2 {
        return ErrorBadRequest("Too few media items").into();
    }
    let chat = body.chat_id.chat();
    let protect_content = body.protect_content;
    let mut reply_to_message = None;
    if let Some(reply_parameters) = &body.reply_parameters {
        check_if_message_exists!(reply_parameters.message_id.0);
        // All of messages in the media group are replying to the same message
        reply_to_message = Some(Box::new(
            MESSAGES.get_message(reply_parameters.message_id.0).unwrap(),
        ));
    }
    let media_group_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let mut messages: Vec<Message> = vec![];

    for media in &body.media {
        let file_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let file_unique_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
        let last_id = MESSAGES.max_message_id();
        let message: Message;
        match media {
            MediaGroupInputMedia::InputMediaAudio(audio) => {
                let mut mock_message = MockMessageAudio::new();
                mock_message.chat = chat.clone();
                mock_message.from = Some(me.user.clone());

                mock_message.has_protected_content = protect_content.unwrap_or(false);
                mock_message.reply_to_message = reply_to_message.clone();
                mock_message.caption = audio.caption.clone();
                mock_message.caption_entities = audio.caption_entities.clone().unwrap_or_default();
                mock_message.media_group_id = Some(media_group_id.clone());
                mock_message.performer = audio.performer.clone();
                mock_message.title = audio.title.clone();
                mock_message.duration = audio.duration.unwrap_or(Seconds::from_seconds(1));

                mock_message.file_name = Some(audio.file_name.clone());
                mock_message.file_id = file_id.clone();
                mock_message.file_unique_id = file_unique_id.clone();
                mock_message.file_size = audio.file_data.bytes().len() as u32;
                mock_message.mime_type = mime_guess::from_path(&audio.file_name).first();

                mock_message.id = MessageId(last_id + 1);
                message = mock_message.build();

                state.files.lock().unwrap().push(teloxide::types::File {
                    meta: message.audio().unwrap().file.clone(),
                    path: audio.file_name.clone(),
                });
            }
            MediaGroupInputMedia::InputMediaDocument(document) => {
                let mut mock_message = MockMessageDocument::new();
                mock_message.chat = chat.clone();
                mock_message.from = Some(me.user.clone());

                mock_message.has_protected_content = protect_content.unwrap_or(false);
                mock_message.reply_to_message = reply_to_message.clone();
                mock_message.caption = document.caption.clone();
                mock_message.caption_entities =
                    document.caption_entities.clone().unwrap_or_default();
                mock_message.media_group_id = Some(media_group_id.clone());

                mock_message.file_name = Some(document.file_name.clone());
                mock_message.file_id = file_id.clone();
                mock_message.file_unique_id = file_unique_id.clone();
                mock_message.file_size = document.file_data.bytes().len() as u32;
                mock_message.mime_type = mime_guess::from_path(&document.file_name).first();

                mock_message.id = MessageId(last_id + 1);
                message = mock_message.build();

                state.files.lock().unwrap().push(teloxide::types::File {
                    meta: message.document().unwrap().file.clone(),
                    path: document.file_name.clone(),
                });
            }
            MediaGroupInputMedia::InputMediaPhoto(photo) => {
                let mut mock_message = MockMessagePhoto::new();
                mock_message.chat = chat.clone();
                mock_message.from = Some(me.user.clone());

                mock_message.has_protected_content = protect_content.unwrap_or(false);
                mock_message.reply_to_message = reply_to_message.clone();
                mock_message.caption = photo.caption.clone();
                mock_message.caption_entities = photo.caption_entities.clone().unwrap_or_default();
                mock_message.media_group_id = Some(media_group_id.clone());

                let mut mock_photo = MockPhotoSize::new();

                mock_photo.file_id = file_id.clone();
                mock_photo.file_unique_id = file_unique_id.clone();
                mock_photo.file_size = photo.file_data.bytes().len() as u32;

                mock_message.photo = vec![mock_photo.build()];

                mock_message.id = MessageId(last_id + 1);
                message = mock_message.build();

                state.files.lock().unwrap().push(teloxide::types::File {
                    meta: message.photo().unwrap().first().unwrap().clone().file,
                    path: photo.file_name.clone(),
                });
            }
            MediaGroupInputMedia::InputMediaVideo(video) => {
                let mut mock_message = MockMessageVideo::new();
                mock_message.chat = chat.clone();
                mock_message.from = Some(me.user.clone());

                mock_message.has_protected_content = protect_content.unwrap_or(false);
                mock_message.reply_to_message = reply_to_message.clone();
                mock_message.caption = video.caption.clone();
                mock_message.caption_entities = video.caption_entities.clone().unwrap_or_default();
                mock_message.media_group_id = Some(media_group_id.clone());

                let mut mock_video = MockVideo::new();

                mock_video.mime_type = mime_guess::from_path(&video.file_name).first();
                mock_video.width = video.width.unwrap_or(100);
                mock_video.height = video.height.unwrap_or(100);
                mock_video.duration = video.duration.unwrap_or(Seconds::from_seconds(1));
                mock_video.file_id = file_id.clone();
                mock_video.file_unique_id = file_unique_id.clone();
                mock_video.file_size = video.file_data.bytes().len() as u32;
                mock_video.file_name = Some(video.file_name.clone());

                mock_message.video = mock_video.build();

                mock_message.id = MessageId(last_id + 1);
                message = mock_message.build();

                state.files.lock().unwrap().push(teloxide::types::File {
                    meta: message.video().unwrap().file.clone(),
                    path: video.file_name.clone(),
                });
            }
        }

        messages.push(message.clone());
        MESSAGES.add_message(message);
    }

    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock.sent_messages.extend(messages.clone());
    responses_lock.sent_media_group.push(SentMediaGroup {
        messages: messages.clone(),
        bot_request: body,
    });
    make_telegram_result(messages)
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendMediaGroupBody {
    pub chat_id: BodyChatId,
    pub message_thread_id: Option<i64>,
    pub media: Vec<MediaGroupInputMedia>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
}

impl SendMediaGroupBody {
    fn serialize_raw_fields(
        fields: &HashMap<String, String>,
        attachments: &HashMap<String, Attachment>,
    ) -> Option<Self> {
        let raw_media: Vec<Value> = serde_json::from_str(fields.get("media")?).ok()?;
        let mut media: Vec<MediaGroupInputMedia> = vec![];
        for raw_media_item in raw_media.iter() {
            let raw_media_string = raw_media_item.get("media").unwrap().as_str().unwrap();
            let file_name;
            let file_data;
            if raw_media_string.starts_with("attach://") {
                let raw_name = raw_media_string.strip_prefix("attach://").unwrap();
                let attachment = attachments
                    .values()
                    .find(|a| a.raw_name == raw_name)
                    .expect("No attachment was found!");
                file_name = Some(attachment.file_name.clone());
                file_data = attachment.file_data.clone();
            } else {
                file_name = None;
                file_data = raw_media_item.get("media").unwrap().to_string();
            }

            let media_type = raw_media_item.get("type").unwrap();
            let caption = raw_media_item
                .get("caption")
                .map(|s| serde_json::from_value(s.clone()).unwrap());
            let parse_mode: Option<ParseMode> = raw_media_item
                .get("parse_mode")
                .map(|s| serde_json::from_value(s.clone()).unwrap());
            let caption_entities: Option<Vec<MessageEntity>> = raw_media_item
                .get("caption_entities")
                .map(|s| serde_json::from_value(s.clone()).unwrap());
            let duration: Option<Seconds> = raw_media_item
                .get("duration")
                .map(|s| serde_json::from_value(s.clone()).unwrap());
            let performer = raw_media_item
                .get("performer")
                .map(|s| serde_json::from_value(s.clone()).unwrap());
            let title = raw_media_item
                .get("title")
                .map(|s| serde_json::from_value(s.clone()).unwrap());
            let disable_content_type_detection: Option<bool> = raw_media_item
                .get("disable_content_type_detection")
                .map(|s| serde_json::from_value(s.clone()).unwrap());
            let show_caption_above_media: Option<bool> = raw_media_item
                .get("show_caption_above_media")
                .map(|s| serde_json::from_value(s.clone()).unwrap());
            let has_spoiler: Option<bool> = raw_media_item
                .get("has_spoiler")
                .map(|s| serde_json::from_value(s.clone()).unwrap());
            let width: Option<u32> = raw_media_item
                .get("width")
                .map(|s| serde_json::from_value(s.clone()).unwrap());
            let height: Option<u32> = raw_media_item
                .get("height")
                .map(|s| serde_json::from_value(s.clone()).unwrap());
            let supports_streaming: Option<bool> = raw_media_item
                .get("supports_streaming")
                .map(|s| serde_json::from_value(s.clone()).unwrap());

            if media_type == "audio" {
                media.push(MediaGroupInputMedia::InputMediaAudio(
                    MediaGroupInputMediaAudio {
                        r#type: "audio".to_string(),
                        file_name: file_name.unwrap_or("no_name.mp3".to_string()),
                        file_data,
                        caption,
                        parse_mode,
                        caption_entities,
                        duration,
                        performer,
                        title,
                    },
                ));
            } else if media_type == "document" {
                media.push(MediaGroupInputMedia::InputMediaDocument(
                    MediaGroupInputMediaDocument {
                        r#type: "document".to_string(),
                        file_name: file_name.unwrap_or("no_name.txt".to_string()),
                        file_data,
                        caption,
                        parse_mode,
                        caption_entities,
                        disable_content_type_detection,
                    },
                ));
            } else if media_type == "photo" {
                media.push(MediaGroupInputMedia::InputMediaPhoto(
                    MediaGroupInputMediaPhoto {
                        r#type: "photo".to_string(),
                        file_name: file_name.unwrap_or("no_name.jpg".to_string()),
                        file_data,
                        caption,
                        parse_mode,
                        caption_entities,
                        show_caption_above_media,
                        has_spoiler,
                    },
                ));
            } else if media_type == "video" {
                media.push(MediaGroupInputMedia::InputMediaVideo(
                    MediaGroupInputMediaVideo {
                        r#type: "video".to_string(),
                        file_name: file_name.unwrap_or("no_name.mp4".to_string()),
                        file_data,
                        caption,
                        parse_mode,
                        caption_entities,
                        duration,
                        supports_streaming,
                        show_caption_above_media,
                        width,
                        height,
                        has_spoiler,
                    },
                ));
            } else {
                panic!("Unknown media type: {}", media_type);
            }
        }

        Some(Self {
            chat_id: serde_json::from_str(&fields.get("chat_id").unwrap().clone()).unwrap(),
            message_thread_id: fields.get("message_thread_id").map(|s| s.parse().unwrap()),
            media,
            disable_notification: fields
                .get("disable_notification")
                .map(|s| s.parse().unwrap()),
            protect_content: fields.get("protect_content").map(|s| s.parse().unwrap()),
            message_effect_id: fields.get("message_effect_id").map(|s| s.to_string()),
            reply_parameters: fields
                .get("reply_parameters")
                .map(|s| serde_json::from_str(s).unwrap()),
        })
    }
}
