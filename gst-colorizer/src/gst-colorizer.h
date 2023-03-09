#ifndef __GST_COLORIZER_H__
#define __GST_COLORIZER_H__

#include <gst/gst.h>
#include <gst/video/gstvideofilter.h>
#include <gst/video/video.h>

G_BEGIN_DECLS
#define GST_TYPE_COLORIZER (gst_colorizer_get_type())
#define GST_COLORIZER(obj) \
  (G_TYPE_CHECK_INSTANCE_CAST((obj), GST_TYPE_COLORIZER, GstColorizer))
#define GST_COLORIZER_CLASS(klass) \
  (G_TYPE_CHECK_CLASS_CAST((klass), GST_TYPE_COLORIZER, GstColorizerClass))
#define GST_IS_COLORIZER(obj) \
  (G_TYPE_CHECK_INSTANCE_TYPE((obj), GST_TYPE_COLORIZER))
#define GST_IS_COLORIZER_CLASS(klass) \
  (G_TYPE_CHECK_CLASS_TYPE((klass), GST_TYPE_COLORIZER))
typedef struct _GstColorizer GstColorizer;
typedef struct _GstColorizerClass GstColorizerClass;

/**
 * GstColorizerPreset:
 * @GST_CLUT_PRESET_NONE: Do nothing preset (default)
 * @GST_CLUT_PRESET_JET: Apply jet color map to image
 *
 * The lookup table to use to convert grayscale to the given color palette
 */
typedef enum {
  GST_COLORIZER_PRESET_NONE,
  GST_COLORIZER_PRESET_JET,
} GstColorizerPreset;

/**
 * GstColorizer:
 *
 * Opaque datastructure.
 */
struct _GstColorizer {
  GstVideoFilter videofilter;

  /* < private > */
  GstColorizerPreset preset;
  guint8 *table;
  guint16 near_cut;
  guint16 far_cut;

  /* video format */
  GstVideoFormat format;
  gint width;
  gint height;
  GstVideoInfo *info;

  void (*process)(GstColorizer *filter, GstVideoFrame *frame);
};

struct _GstColorizerClass {
  GstVideoFilterClass parent_class;
};

GType gst_colorizer_get_type(void);

G_END_DECLS
#endif

/* __GST_COLORIZER_H__ */
