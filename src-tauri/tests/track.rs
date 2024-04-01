use bragi::player::track::Track;

#[test]
fn test_track_duration_mp3() {
    let track = Track::try_new("tests/assets/track.mp3").unwrap();

    let actual = track.length().unwrap().round();

    assert_eq!(4.0, actual);
}

#[test]
fn test_track_duration_flac() {
    let track = Track::try_new("tests/assets/track.flac").unwrap();

    let actual = track.length().unwrap().round();

    assert_eq!(6.0, actual);
}
