#include "buffer-meta.h"

GType buffer_meta_api_get_type(void) {
  static volatile GType type;
  static const gchar *tags[] = {NULL};

  if (g_once_init_enter(&type)) {
    GType _type = gst_meta_api_type_register("BufferMetaAPI", tags);
    g_once_init_leave(&type, _type);
  }
  return type;
}

gboolean buffer_meta_init(GstMeta *meta, gpointer params, GstBuffer *buffer) {
  return TRUE;
}

gboolean buffer_meta_transform(GstBuffer *dest_buf, GstMeta *src_meta,
                               GstBuffer *src_buf, GQuark type, gpointer data) {
  BufferMeta *src = (BufferMeta *)src_meta;
  buffer_meta_add(dest_buf, src->buffer);

  return TRUE;
}

void buffer_meta_free(GstMeta *meta, GstBuffer *buffer) {
  BufferMeta *local_meta = (BufferMeta *)meta;
  gst_buffer_unref(local_meta->buffer);
}

const GstMetaInfo *buffer_meta_get_info(void) {
  static const GstMetaInfo *meta_info = NULL;

  if (g_once_init_enter(&meta_info)) {
    const GstMetaInfo *meta = gst_meta_register(
        buffer_meta_api_get_type(), "BufferMeta", sizeof(BufferMeta),
        (GstMetaInitFunction)buffer_meta_init,
        (GstMetaFreeFunction)buffer_meta_free,
        (GstMetaTransformFunction)buffer_meta_transform);
    g_once_init_leave(&meta_info, meta);
  }
  return meta_info;
}

BufferMeta *buffer_meta_get(GstBuffer *buffer) {
  g_return_val_if_fail(GST_IS_BUFFER(buffer), NULL);

  return (
      (BufferMeta *)gst_buffer_get_meta(buffer, buffer_meta_api_get_type()));
}

BufferMeta *buffer_meta_add(GstBuffer *buffer, GstBuffer *meta_buffer) {
  BufferMeta *meta;

  g_return_val_if_fail(GST_IS_BUFFER(buffer), NULL);

  meta =
      (BufferMeta *)gst_buffer_add_meta(buffer, buffer_meta_get_info(), NULL);

  if (!meta) return NULL;

  meta->buffer = gst_buffer_ref(meta_buffer);

  return meta;
}
