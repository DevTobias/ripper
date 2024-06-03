import { z } from 'zod';

export const DiscPropertiesSchema = z
  .object({
    disc_type: z.string(),
    name: z.string(),
    titles: z.array(
      z.object({
        id: z.number(),
        name: z.string(),
        chapter_count: z.number(),
        duration: z.number(),
        disk_size: z.string(),
        video_stream: z.object({
          video_size: z.string(),
        }),
        audio_streams: z.array(
          z.object({
            name: z.string(),
            lang_name: z.string(),
            codec_long: z.string(),
          })
        ),
        subtitle_streams: z.array(
          z.object({
            lang_name: z.string(),
          })
        ),
      })
    ),
  })
  .transform((data) => ({
    discType: data.disc_type,
    name: data.name,
    titles: data.titles.map((title) => ({
      id: title.id.toString(),
      name: title.name,
      chapterCount: title.chapter_count,
      duration: title.duration,
      diskSize: title.disk_size,
      videoSize: title.video_stream.video_size,
      audioStreams: title.audio_streams.map((audio) => ({
        name: audio.name,
        langName: audio.lang_name,
        codecLong: audio.codec_long,
      })),
      subtitleStreams: title.subtitle_streams.map((subtitle) => subtitle.lang_name),
    })),
  }));
