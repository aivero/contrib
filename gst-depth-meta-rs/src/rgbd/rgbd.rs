use crate::buffer::BufferMeta;
use crate::tags::TagsMeta;
use crate::RgbdError;
use gst::CoreError;

use core::ops::ControlFlow::*;
use gst::buffer::BufferMetaForeachAction::*;

/// Fill the `main_buffer` with `buffer` and mark it appropriately with `tag`.
///
/// # Arguments
/// * `main_buffer` - Output buffer that will be tagged and contain `buffer` after this function gets executed.
/// * `buffer` - Buffer containing the data.
/// * `tag` - Tag of the buffer.
///
/// # Returns
/// * `Ok()` on success.
/// * `Err(gst::ErrorMessage)` on failure, occurs only if mutable references to tags/buffer cannot be obtained.
pub fn fill_main_buffer_and_tag(
    main_buffer: &mut gst::Buffer,
    buffer: gst::Buffer,
    tag: &str,
) -> Result<(), gst::ErrorMessage> {
    // Put the frame directly into the output buffer
    *main_buffer = buffer;

    // Tag the buffer appropriately
    tag_buffer_with_title(
        main_buffer.get_mut().ok_or_else(|| {
            gst::error_msg!(
                gst::ResourceError::Failed,
                [
                    "Cannot get mutable reference to the buffer for {} stream",
                    tag
                ]
            )
        })?,
        tag,
    )?;

    // Return Ok() if everything went fine
    Ok(())
}

/// Attach the `buffer` to the `main_buffer` and mark it appropriately with `tag`.
///
/// # Arguments
/// * `main_buffer` - Output buffer that will have `buffer` attached to it after this function gets executed.
/// * `buffer` - Buffer containing the data, which will be tagged and attached to `main_buffer`.
/// * `tag` - Tag of the buffer.
///
/// # Returns
/// * `Ok()` on success.
/// * `Err(gst::ErrorMessage)` on failure, occurs only if mutable references to tags/buffer cannot be obtained.
pub fn attach_aux_buffer_and_tag(
    main_buffer: &mut gst::BufferRef,
    buffer: &mut gst::Buffer,
    tag: &str,
) -> Result<(), gst::ErrorMessage> {
    // Tag the buffer appropriately
    tag_buffer_with_title(
        buffer.get_mut().ok_or_else(|| {
            gst::error_msg!(
                gst::ResourceError::Failed,
                [
                    "Cannot get mutable reference to the buffer for {} stream",
                    tag
                ]
            )
        })?,
        tag,
    )?;

    // Attach this new buffer as meta to the output buffer
    BufferMeta::add(main_buffer, buffer);

    // Return Ok() if everything went fine
    Ok(())
}

/// Given a main buffer, returns all auxiliary buffers attached to the `main_buffer`.
///
/// # Arguments
/// * `main_buffer` - The main buffer that contains aux buffers
///
/// # Returns
/// * Vec<gst::Buffer> containing the auxiliary buffers.
pub fn get_all_aux_buffers(main_buffer: &gst::BufferRef) -> impl Iterator<Item = gst::Buffer> + '_ {
    main_buffer
        .iter_meta::<BufferMeta>()
        .map(|meta| meta.buffer_owned())
}
/// Get the main buffer, plus all auxiliary buffers attached to the `main_buffer`.
///
/// # Arguments
/// * `main_buffer` - The main buffer that contains aux buffers
///
/// # Returns
/// * Vec<gst::Buffer> containing the main buffer at [0], followed by the auxiliary buffers.
pub fn get_all_buffers(main_buffer: gst::Buffer) -> Vec<gst::Buffer> {
    let mut vec_buf = main_buffer
        .iter_meta::<BufferMeta>()
        .map(|meta| meta.buffer_owned())
        .collect::<Vec<gst::Buffer>>();
    vec_buf.insert(0, main_buffer);
    vec_buf
}

/// Tag a `buffer` with `tag`.
///
/// # Arguments
/// * `buffer` - Buffer to be tagged.
/// * `tag` - The tag to assign to the buffer.
///
/// # Returns
/// * `Ok()` on success.
/// * `Err(gst::ErrorMessage)` on failure, occurs only if mutable references to tags cannot be obtained.
pub fn tag_buffer_with_title(
    buffer: &mut gst::BufferRef,
    tag: &str,
) -> Result<(), gst::ErrorMessage> {
    // Create an appropriate tag
    let mut tags = gst::tags::TagList::new();
    tags.get_mut()
        .ok_or_else(|| {
            gst::error_msg!(
                gst::ResourceError::Failed,
                ["Cannot get mutable reference to {} tag", tag]
            )
        })?
        .add::<gst::tags::Title>(&tag, gst::TagMergeMode::Append);

    // Add the tag to the output buffer
    TagsMeta::add(buffer, &mut tags);

    // Return Ok() if everything went fine
    Ok(())
}

/// Get tag of the `buffer`.
///
/// # Arguments
/// * `buffer` - Buffer to read the tag from.
///
/// # Returns
/// * `Ok(&str)` with tag on success.
/// * `Err(gst::ErrorMessage)` on failure, if buffer has invalid tag.
pub fn get_tag(buffer: &gst::BufferRef) -> Result<String, gst::ErrorMessage> {
    // Get TagList from GstBuffer
    let tag_list = buffer
        .meta::<TagsMeta>()
        .ok_or_else(|| {
            gst::error_msg!(
                gst::ResourceError::Failed,
                ["Buffer {:?} has no tags", buffer]
            )
        })?
        .get_tag_list();

    // Get the title tag from TagList
    let tag = tag_list.get::<gst::tags::Title>().ok_or_else(|| {
        gst::error_msg!(
            gst::ResourceError::Failed,
            ["Buffer {:?} has no title tag", buffer]
        )
    })?;

    // Return it as string slice
    Ok(String::from(tag.get()))
}

/// Remove all tags of a `buffer`.
///
/// # Arguments
/// * `buffer` - The buffer to remove the tags from.
pub fn clear_tags(buffer: &mut gst::BufferRef) {
    buffer.foreach_meta_mut(|meta| {
        if meta.as_ref().downcast_ref::<TagsMeta>().is_some() {
            Continue(Remove)
        } else {
            Continue(Keep)
        }
    });
}

/// Replaces all existing tags of `buffer` with `tag`.
///
/// # Arguments
/// * `buffer` - The buffer to remove the tags from.
/// * `tag` - The tag to assign to the buffer.
///
/// # Returns
/// * `Ok()` on success.
/// * `Err(gst::ErrorMessage)` on failure, occurs only if mutable references to tags cannot be obtained.
pub fn replace_tag(buffer: &mut gst::BufferRef, tag: &str) -> Result<(), gst::ErrorMessage> {
    // First remove all tags
    clear_tags(buffer);

    // Retag the buffer appropriately
    tag_buffer_with_title(buffer, tag)?;

    // Return Ok() if everything went fine
    Ok(())
}

/// Removes the specific auxiliary buffers attached to the `main_buffer` based on the specified `tags`.
///
/// # Arguments
/// * `main_buffer` - The main buffer to remove auxiliary buffers from.
/// * `tags` - The tags that specify what buffers to remove.
///
/// # Returns
/// * `Ok()` on success.
/// * `Err(gst::ErrorMessage)` on failure, occurs only if mutable references to tags cannot be obtained.
pub fn remove_aux_buffers_with_tags(
    main_buffer: &mut gst::BufferRef,
    tags: &[&str],
) -> Result<(), gst::ErrorMessage> {
    // Loop over all auxiliary buffers
    main_buffer.foreach_meta_mut(|meta| {
        if let Some(meta) = meta.as_ref().downcast_ref::<BufferMeta>() {
            let auxiliary_buffer = meta.buffer();
            let tag = match get_tag(auxiliary_buffer) {
                Err(_) => return Continue(Keep),
                Ok(tag) => tag,
            };

            if tags.contains(&&*tag) {
                // Remove buffers with the corresponding tags
                Continue(Remove)
            } else {
                Continue(Keep)
            }
        } else {
            Continue(Keep)
        }
    });

    // Return Ok() if everything went fine
    Ok(())
}

/// Converts the given `caps` and `framerate` into a `gst::VideoInfo` for the stream with the given
/// name.
/// # Arguments
/// * `caps` - The caps to convert.
/// * `framerate` - The framerate of the stream.
/// # Returns
/// * `Ok(VideoInfo)` - If the CAPS were successfully converted.
/// * `Err(RgbdError::MissingCapsField)` - If any required fields on the CAPS do not exist.
/// * `Err(RgbdError::WrongCapsFormat)` - If a field on the CAPS are not of correct type.
/// * `Err(RgbdError::NoVideoInfo)` - If `caps` could not be converted into `VideoInfo`.
pub fn get_video_info(
    caps: &gst::StructureRef,
    stream_name: &str,
) -> Result<gst_video::VideoInfo, gst::ErrorMessage> {
    let framerate = caps
        .get::<gst::Fraction>("framerate")
        .map_err(|e| gst::error_msg!(CoreError::Caps, ["{}", e]))?;
    let stream_width = caps
        .get::<i32>(&format!("{}_width", stream_name))
        .map_err(|e| gst::error_msg!(CoreError::Caps, ["{}", e]))?;
    let stream_height = caps
        .get::<i32>(&format!("{}_height", stream_name))
        .map_err(|e| gst::error_msg!(CoreError::Caps, ["{}", e]))?;
    let stream_format = caps
        .get::<&str>(&format!("{}_format", stream_name))
        .map_err(|e| gst::error_msg!(CoreError::Caps, ["{}", e]))?;

    gst_video::VideoInfo::builder(
        stream_format
            .parse()
            .map_err(|e| gst::error_msg!(CoreError::Caps, ["{}", e]))?,
        stream_width as u32,
        stream_height as u32,
    )
    .fps(framerate)
    .build()
    .map_err(|e| gst::error_msg!(CoreError::Caps, ["{}", e]))
}

/// Aligns `buffer` to u16, such that it can be used to store depth video.
/// # Arguments
/// * `buffer` - The buffer to convert to depth.
/// # Returns
/// * `Ok` - If the buffer is properly aligned to u16.
/// * `Err` - If not.
pub fn to_depth_buffer(buffer: &[u8]) -> Result<&[u16], RgbdError> {
    use byte_slice_cast::*;

    buffer
        .as_slice_of::<u16>()
        .map_err(|_| RgbdError::BufferNotAligned)
}

pub fn to_depth_buffer_mut(buffer: &mut [u8]) -> Result<&mut [u16], RgbdError> {
    use byte_slice_cast::*;

    buffer
        .as_mut_slice_of::<u16>()
        .map_err(|_| RgbdError::BufferNotAligned)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gst::Buffer;

    #[test]
    fn new_buffer_no_tag() {
        gst::init().unwrap();

        let buffer = Buffer::new();

        // Make sure there is no tag in a new buffer
        assert!(get_tag(&buffer).is_err());
    }

    #[test]
    fn beffer_tag() {
        gst::init().unwrap();

        let mut buffer = Buffer::new();
        let original_tag = "depth";

        // Create main buffer with a tag
        tag_buffer_with_title(buffer.get_mut().unwrap(), original_tag).unwrap();

        // Extract the tag from the buffer
        let tag = get_tag(&buffer).unwrap();

        // Make sure the tag stayed the same
        assert_eq!(tag, original_tag);
    }

    #[test]
    fn main_buffer_tag() {
        gst::init().unwrap();

        let mut main_buffer = Buffer::new();
        let buffer = Buffer::new();
        let original_tag = "depth";

        // Create main buffer with a tag
        fill_main_buffer_and_tag(&mut main_buffer, buffer, original_tag).unwrap();

        // Extract the tag from the buffer
        let tag = get_tag(&main_buffer).unwrap();

        // Make sure the tag stayed the same
        assert_eq!(tag, original_tag);
    }

    #[test]
    fn aux_buffers_tag() {
        gst::init().unwrap();

        let mut main_buffer = Buffer::new();
        let mut buffer_aux0 = Buffer::new();
        let mut buffer_aux1 = Buffer::new();
        let original_tag_main = "main";
        let original_tag_aux0 = "aux0";
        let original_tag_aux1 = "aux1";

        // Tag the main buffer
        tag_buffer_with_title(main_buffer.make_mut(), original_tag_main).unwrap();

        // Attach buffers to the main buffer
        attach_aux_buffer_and_tag(
            main_buffer.get_mut().unwrap(),
            &mut buffer_aux0,
            original_tag_aux0,
        )
        .unwrap();
        attach_aux_buffer_and_tag(
            main_buffer.get_mut().unwrap(),
            &mut buffer_aux1,
            original_tag_aux1,
        )
        .unwrap();

        // Get all buffers buffers
        let all_buffers = get_all_buffers(main_buffer);

        // Make sure the length is correct
        assert_eq!(all_buffers.len(), 3);

        // Extract the tag from the buffer
        // todo: the ordering here should only be
        let tag_main = get_tag(&all_buffers[0]).unwrap();
        let tag_aux0 = get_tag(&all_buffers[1]).unwrap();
        let tag_aux1 = get_tag(&all_buffers[2]).unwrap();

        // Make sure the tags stayed the same
        assert_eq!(tag_main, original_tag_main);
        assert_eq!(tag_aux0, original_tag_aux0);
        assert_eq!(tag_aux1, original_tag_aux1);
    }

    #[test]
    fn tag_clearing() {
        gst::init().unwrap();

        let mut buffer = Buffer::new();
        let original_tag = "depth";

        // Create main buffer with a tag
        tag_buffer_with_title(buffer.get_mut().unwrap(), original_tag).unwrap();

        // Extract the tag from the buffer
        let tag = get_tag(&buffer).unwrap();

        // Make sure the tag stayed the same
        assert_eq!(tag, original_tag);

        clear_tags(buffer.get_mut().unwrap());

        // Make sure there is no tag anymore
        assert!(get_tag(&buffer).is_err());
    }

    #[test]
    fn tag_replace() {
        gst::init().unwrap();

        let mut buffer = Buffer::new();
        let original_tag = "depth";
        let new_tag = "color";

        // Create main buffer with a tag
        tag_buffer_with_title(buffer.get_mut().unwrap(), original_tag).unwrap();

        // Extract the tag from the buffer
        let tag = get_tag(&buffer).unwrap();

        // Make sure the tag stayed the same
        assert_eq!(tag, original_tag);

        replace_tag(buffer.get_mut().unwrap(), new_tag).unwrap();

        // Extract the tag from the buffer again
        let tag = get_tag(&buffer).unwrap();

        // Make sure the tag changed
        assert_eq!(tag, new_tag);
    }

    #[test]
    fn remove_specific_aux_buffers() {
        gst::init().unwrap();

        let mut main_buffer = Buffer::new();
        let mut buffer_depth = Buffer::new();
        let mut buffer_color = Buffer::new();
        let original_tag_main = "infra";
        let original_tag_depth = "depth";
        let original_tag_color = "color";

        // Create main buffer with a tag
        tag_buffer_with_title(main_buffer.get_mut().unwrap(), original_tag_main).unwrap();

        // Attach buffers to the main buffer
        attach_aux_buffer_and_tag(
            main_buffer.get_mut().unwrap(),
            &mut buffer_depth,
            original_tag_depth,
        )
        .unwrap();
        attach_aux_buffer_and_tag(
            main_buffer.get_mut().unwrap(),
            &mut buffer_color,
            original_tag_color,
        )
        .unwrap();

        // Get the auxiliary buffers
        let all_buffers: Vec<gst::Buffer> = get_all_aux_buffers(&main_buffer).collect();

        // Make sure the length is correct
        assert_eq!(all_buffers.len(), 2);

        // Extract the tag from the buffer
        let tag_depth = get_tag(&all_buffers[0]).unwrap();
        let tag_color = get_tag(&all_buffers[1]).unwrap();

        // Make sure the tags stayed the same
        assert_eq!(tag_depth, original_tag_depth);
        assert_eq!(tag_color, original_tag_color);

        // let mut main_buffer = all_buffers[0];
        // Remove depth and color
        remove_aux_buffers_with_tags(
            main_buffer.get_mut().unwrap(),
            &[original_tag_color, original_tag_depth],
        )
        .unwrap();

        // Get the auxiliary buffers
        let all_buffers = get_all_buffers(main_buffer);

        // Make sure the length is correct
        assert_eq!(all_buffers.len(), 1);

        // Make sure the correct buffer remains (infra)
        let tag_main = get_tag(&all_buffers[0]).unwrap();
        assert_eq!(tag_main, original_tag_main);
    }
}
