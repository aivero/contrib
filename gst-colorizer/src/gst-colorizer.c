#include "gst-colorizer.h"

#include <gst/video/video.h>

GST_DEBUG_CATEGORY_STATIC(colorizer_debug);
#define GST_CAT_DEFAULT (colorizer_debug)

enum { PROP_0, PROP_PRESET, PROP_NEAR_CUT, PROP_FAR_CUT };

#define gst_colorizer_parent_class parent_class
G_DEFINE_TYPE(GstColorizer, gst_colorizer, GST_TYPE_VIDEO_FILTER);

static GstStaticPadTemplate gst_colorizer_src_template =
    GST_STATIC_PAD_TEMPLATE("src", GST_PAD_SRC, GST_PAD_ALWAYS,
                            GST_STATIC_CAPS(GST_VIDEO_CAPS_MAKE("{ RGB }")));

static GstStaticPadTemplate gst_colorizer_sink_template =
    GST_STATIC_PAD_TEMPLATE(
        "sink", GST_PAD_SINK, GST_PAD_ALWAYS,
        GST_STATIC_CAPS(GST_VIDEO_CAPS_MAKE("{ GRAY16_LE }")));

#define GST_TYPE_COLORIZER_PRESET (gst_colorizer_preset_get_type())
static GType gst_colorizer_preset_get_type(void) {
  static GType preset_type = 0;

  static const GEnumValue presets[] = {
      {GST_COLORIZER_PRESET_JET, "Apply jet color map to image", "jet"},
      {0, NULL, NULL},
  };

  if (!preset_type) {
    preset_type = g_enum_register_static("GstColorizerPreset", presets);
  }
  return preset_type;
}

static GstFlowReturn gst_colorizer_transform_gray16(GstColorizer *filter,
                                                    GstVideoFrame *inframe,
                                                    GstVideoFrame *outframe) {
  gint width = GST_VIDEO_FRAME_WIDTH(outframe);
  gint height = GST_VIDEO_FRAME_HEIGHT(outframe);

  guint16 *in = (guint16 *)inframe->data[0];
  guint8 *out = (guint8 *)outframe->data[0];

  for (guint i = 0; i < width * height; i++) {
    guint16 gray = in[i];
    // Check if the value is to be truncated (i.e. too near or too far), if so set it to pitch black
    if (gray <= filter->near_cut || gray > filter->far_cut) {
        out[i * 3 + 0] = 0;
        out[i * 3 + 1] = 0;
        out[i * 3 + 2] = 0;
    }
    // Otherwise apply its color scheme
    else {
        out[i * 3 + 0] = filter->table[gray * 3 + 0];
        out[i * 3 + 1] = filter->table[gray * 3 + 1];
        out[i * 3 + 2] = filter->table[gray * 3 + 2];
    }
  }

  return GST_FLOW_OK;
}

static gboolean gst_colorizer_set_info(GstVideoFilter *vfilter, GstCaps *incaps,
                                       GstVideoInfo *in_info, GstCaps *outcaps,
                                       GstVideoInfo *out_info) {
  GstColorizer *filter = GST_COLORIZER(vfilter);

  GST_DEBUG_OBJECT(filter, "in %" GST_PTR_FORMAT " out %" GST_PTR_FORMAT,
                   incaps, outcaps);

  filter->process = NULL;

  filter->info = out_info;
  filter->format = GST_VIDEO_INFO_FORMAT(in_info);
  filter->width = GST_VIDEO_INFO_WIDTH(in_info);
  filter->height = GST_VIDEO_INFO_HEIGHT(in_info);

  GST_OBJECT_LOCK(filter);

  switch (filter->format) {
    case GST_VIDEO_FORMAT_GRAY16_LE:
      filter->process = gst_colorizer_transform_gray16;
      break;
    default:
      break;
  }

  GST_OBJECT_UNLOCK(filter);

  return filter->process != NULL;
}

double clamp(double v) {
  const double t = v < 0 ? 0 : v;
  return t > 1.0 ? 1.0 : t;
}

void generate_map(GstColorizer *filter, GstColorizerPreset preset) {
  if (filter->table) {
    free(filter->table);
    filter->table = NULL;
  }

  if (preset == GST_COLORIZER_PRESET_JET) {
    filter->table = calloc(3 * filter->far_cut, sizeof(guint8));
    for (guint16 i = filter->near_cut; i < filter->far_cut; i++) {
      double t = ((i - filter->near_cut) * 2.0 /
                  (filter->far_cut - filter->near_cut - 1)) -
                 1;
      guint8 red = clamp(1.5 - ABS(2.0 * t - 1.0)) * 255;
      guint8 green = clamp(1.5 - ABS(2.0 * t)) * 255;
      guint8 blue = clamp(1.5 - ABS(2.0 * t + 1.0)) * 255;
      filter->table[i * 3 + 0] = red;
      filter->table[i * 3 + 1] = green;
      filter->table[i * 3 + 2] = blue;
    }
  } else {
    g_assert_not_reached();
  }
}

static void gst_colorizer_set_property(GObject *object, guint prop_id,
                                       const GValue *value, GParamSpec *pspec) {
  GstColorizer *filter = GST_COLORIZER(object);

  switch (prop_id) {
    case PROP_PRESET:
      GST_OBJECT_LOCK(filter);
      filter->preset = g_value_get_enum(value);
      generate_map(filter, filter->preset);
      GST_OBJECT_UNLOCK(filter);
      break;
    case PROP_NEAR_CUT:
      filter->near_cut = g_value_get_uint(value);
      generate_map(filter, filter->preset);
      break;
    case PROP_FAR_CUT:
      filter->far_cut = g_value_get_uint(value);
      generate_map(filter, filter->preset);
      break;
    default:
      G_OBJECT_WARN_INVALID_PROPERTY_ID(object, prop_id, pspec);
      break;
  }
}

static void gst_colorizer_get_property(GObject *object, guint prop_id,
                                       GValue *value, GParamSpec *pspec) {
  GstColorizer *filter = GST_COLORIZER(object);

  switch (prop_id) {
    case PROP_PRESET:
      GST_OBJECT_LOCK(filter);
      g_value_set_enum(value, filter->preset);
      GST_OBJECT_UNLOCK(filter);
      break;
    case PROP_NEAR_CUT:
      GST_OBJECT_LOCK(filter);
      g_value_set_uint(value, filter->near_cut);
      GST_OBJECT_UNLOCK(filter);
      break;
    case PROP_FAR_CUT:
      GST_OBJECT_LOCK(filter);
      g_value_set_uint(value, filter->far_cut);
      GST_OBJECT_UNLOCK(filter);
      break;
    default:
      G_OBJECT_WARN_INVALID_PROPERTY_ID(object, prop_id, pspec);
      break;
  }
}

static GstCaps *gst_colorizer_transform_caps(GstBaseTransform *trans,
                                             GstPadDirection direction,
                                             GstCaps *caps, GstCaps *filter) {
  GstCaps *other_caps = gst_caps_copy(caps);
  gsize caps_size = gst_caps_get_size(other_caps);

  for (gsize i = 0; i < caps_size; i++) {
    GstStructure *structure = gst_caps_get_structure(other_caps, i);

    if (direction == GST_PAD_SRC) {
      gst_structure_set_name(structure, "video/x-raw");
      GValue gvalue = G_VALUE_INIT;
      g_value_init(&gvalue, G_TYPE_STRING);
      g_value_set_string(&gvalue, "GRAY16_LE");
      gst_structure_set_value(structure, "format", &gvalue);
    } else {
      gst_structure_set_name(structure, "video/x-raw");
      GValue gvalue = G_VALUE_INIT;
      g_value_init(&gvalue, G_TYPE_STRING);
      g_value_set_string(&gvalue, "RGB");
      gst_structure_set_value(structure, "format", &gvalue);
    }
  }

  if (filter) {
    return gst_caps_intersect_full(filter, other_caps,
                                   GST_CAPS_INTERSECT_FIRST);
  } else {
    return other_caps;
  }
}

static void gst_colorizer_class_init(GstColorizerClass *klass) {
  GObjectClass *gobject_class = (GObjectClass *)klass;
  GstElementClass *element_class = (GstElementClass *)klass;
  GstVideoFilterClass *vfilter_class = (GstVideoFilterClass *)klass;
  GstBaseTransformClass *base_transform_class = GST_BASE_TRANSFORM_CLASS(klass);

  GST_DEBUG_CATEGORY_INIT(colorizer_debug, "colorizer", 0, "colorizer");

  base_transform_class->transform_caps = gst_colorizer_transform_caps;

  gobject_class->set_property = gst_colorizer_set_property;
  gobject_class->get_property = gst_colorizer_get_property;

  g_object_class_install_property(
      gobject_class, PROP_PRESET,
      g_param_spec_enum("preset", "Preset", "Color effect preset to use",
                        GST_TYPE_COLORIZER_PRESET, GST_COLORIZER_PRESET_JET,
                        G_PARAM_READWRITE | G_PARAM_STATIC_STRINGS));

  g_object_class_install_property(
      G_OBJECT_CLASS(klass), PROP_NEAR_CUT,
      g_param_spec_uint(
          "near-cut", "Near cut", "Near cut off", 0, 65535, 0,
          G_PARAM_READWRITE | GST_PARAM_CONTROLLABLE | G_PARAM_STATIC_STRINGS));

  g_object_class_install_property(
      G_OBJECT_CLASS(klass), PROP_FAR_CUT,
      g_param_spec_uint(
          "far-cut", "Far cut", "Far cut off", 0, 65535, 65535,
          G_PARAM_READWRITE | GST_PARAM_CONTROLLABLE | G_PARAM_STATIC_STRINGS));

  vfilter_class->set_info = GST_DEBUG_FUNCPTR(gst_colorizer_set_info);
  vfilter_class->transform_frame =
      GST_DEBUG_FUNCPTR(gst_colorizer_transform_gray16);

  gst_element_class_set_static_metadata(
      element_class, "Grayscale colorizer", "Filter/Effect/Video",
      "Grayscale colorizer", "Aivero AS <code@aivero.com>");

  gst_element_class_add_static_pad_template(element_class,
                                            &gst_colorizer_sink_template);
  gst_element_class_add_static_pad_template(element_class,
                                            &gst_colorizer_src_template);
}

static void gst_colorizer_init(GstColorizer *filter) {
  filter->preset = GST_COLORIZER_PRESET_JET;
  filter->near_cut = 0;
  filter->far_cut = 65535;
  generate_map(filter, filter->preset);
}

static gboolean plugin_init(GstPlugin *plugin) {
  gst_element_register(plugin, "colorizer", GST_RANK_NONE,
                       gst_colorizer_get_type());
  return TRUE;
}

#ifndef VERSION
#define VERSION "0.1"
#endif
#ifndef PACKAGE
#define PACKAGE "colorizer"
#endif
#ifndef PACKAGE_NAME
#define PACKAGE_NAME "colorizer"
#endif
#ifndef GST_PACKAGE_ORIGIN
#define GST_PACKAGE_ORIGIN "https://aivero.com"
#endif

GST_PLUGIN_DEFINE(GST_VERSION_MAJOR, GST_VERSION_MINOR, colorizer,
                  "Depth image colorizer", plugin_init, VERSION, "MIT",
                  PACKAGE_NAME, GST_PACKAGE_ORIGIN)
