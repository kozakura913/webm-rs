
extern crate libc;

pub mod mux {

    use libc::{c_void, size_t};

    #[repr(C)]
    pub struct IWriter;
    pub type WriterMutPtr = *mut IWriter;

    pub type WriterWriteFn = extern "C" fn(*mut c_void,
                                           *const c_void,
                                           size_t) -> bool;
    pub type WriterGetPosFn = extern "C" fn(*const c_void) -> u64;
    pub type WriterSetPosFn = extern "C" fn(*mut c_void, u64) -> bool;
    pub type WriterElementStartNotifyFn = extern "C" fn(*mut c_void, u64, i64);

    // audio
    pub const OPUS_CODEC_ID: u32 = 0;
    pub const VORBIS_CODEC_ID: u32 = 1;

    // video
    pub const VP8_CODEC_ID: u32 = 0;
    pub const VP9_CODEC_ID: u32 = 1;

    #[repr(C)]
    pub struct Segment;
    pub type SegmentMutPtr = *mut Segment;

    #[repr(C)]
    pub struct Track;
    pub type TrackMutPtr = *mut Track;

    #[repr(C)]
    pub struct VideoTrack;
    pub type VideoTrackMutPtr = *mut VideoTrack;
    #[repr(C)]
    pub struct AudioTrack;
    pub type AudioTrackMutPtr = *mut AudioTrack;


    #[link(name = "webm", kind = "static")]
    extern "C" {
        pub fn mux_new_writer(write: Option<WriterWriteFn>,
                              get_pos: Option<WriterGetPosFn>,
                              set_pos: Option<WriterSetPosFn>,
                              element_start_notify: Option<WriterElementStartNotifyFn>,
                              user_data: *mut c_void) -> WriterMutPtr;
        pub fn mux_delete_writer(writer: WriterMutPtr);


        pub fn mux_new_segment() -> SegmentMutPtr;
        pub fn mux_initialize_segment(segment: SegmentMutPtr, writer: WriterMutPtr) -> bool;
        pub fn mux_delete_segment(segment: SegmentMutPtr);

        pub fn mux_video_track_base_mut(track: VideoTrackMutPtr) -> TrackMutPtr;
        pub fn mux_audio_track_base_mut(track: AudioTrackMutPtr) -> TrackMutPtr;

        pub fn mux_segment_add_video_track(segment: SegmentMutPtr,
                                           width: i32, height: i32,
                                           number: i32, codec_id: u32) -> VideoTrackMutPtr;
        pub fn mux_segment_add_audio_track(segment: SegmentMutPtr,
                                           sample_rate: i32, channels: i32,
                                           number: i32, codec_id: u32) -> AudioTrackMutPtr;
        pub fn mux_segment_add_frame(segment: SegmentMutPtr,
                                     track: TrackMutPtr,
                                     frame: *const u8, length: size_t,
                                     timestamp_ns: u64, keyframe: bool) -> bool;
    }
}
