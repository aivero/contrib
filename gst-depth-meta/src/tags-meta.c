#include "tags-meta.h"

GType tags_meta_api_get_type(void) {
    static volatile GType type;
    static const gchar *tags[] = {NULL};

    if (g_once_init_enter(&type)) {
        GType _type = gst_meta_api_type_register("TagsMetaAPI", tags);
        g_once_init_leave(&type, _type);
    }
    return type;
}

gboolean tags_meta_init(GstMeta *meta, gpointer params, GstBuffer *buffer) {
    return TRUE;
}

gboolean tags_meta_transform(GstBuffer *dest_buf, GstMeta *src_meta,
                               GstBuffer *src_buf, GQuark type, gpointer data) {
    TagsMeta *src = (TagsMeta *) src_meta;
    tags_meta_add(dest_buf, src->tags);

    return TRUE;
}

void tags_meta_free(GstMeta *meta, GstBuffer *buffer) {
    TagsMeta *local_meta = (TagsMeta *) meta;
    gst_tag_list_unref(local_meta->tags);
}

const GstMetaInfo *tags_meta_get_info(void) {
    static const GstMetaInfo *meta_info = NULL;

    if (g_once_init_enter(&meta_info)) {
        const GstMetaInfo *meta = gst_meta_register(
                tags_meta_api_get_type(), "TagsMeta", sizeof(TagsMeta),
                (GstMetaInitFunction) tags_meta_init,
                (GstMetaFreeFunction) tags_meta_free,
                (GstMetaTransformFunction) tags_meta_transform);
        g_once_init_leave(&meta_info, meta);
    }
    return meta_info;
}

TagsMeta *tags_meta_get(GstBuffer *buffer) {
    g_return_val_if_fail(GST_IS_BUFFER(buffer), NULL);

    return ((TagsMeta *) gst_buffer_get_meta(buffer, tags_meta_api_get_type()));
}

TagsMeta *tags_meta_add(GstBuffer *buffer, GstTagList *meta_tags) {
    TagsMeta *meta;

    g_return_val_if_fail(GST_IS_BUFFER (buffer), NULL);

    meta = (TagsMeta *) gst_buffer_add_meta(buffer, tags_meta_get_info(), NULL);

    if (!meta)
        return NULL;

    meta->tags = gst_tag_list_ref(meta_tags);

    return meta;
}
