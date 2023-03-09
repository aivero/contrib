#pragma once

#include <gst/gst.h>
#include <gst/gstmeta.h>
#include <gst/gsttaglist.h>
#include <gst/video/video.h>

G_BEGIN_DECLS

typedef struct _TagsMeta TagsMeta;

struct _TagsMeta {
  GstMeta meta;
  GstTagList *tags;
};

GType tags_meta_api_get_type(void);
const GstMetaInfo *tags_meta_get_info(void);
TagsMeta *tags_meta_get(GstBuffer *buffer);
TagsMeta *tags_meta_add(GstBuffer *buffer, GstTagList *tags);
gboolean tags_meta_remove(GstBuffer *buffer, GstTagList *tags);

G_END_DECLS
