"""
Basic tests for the LAME Python bindings
"""

import pytest


def test_import():
    """Test that the module can be imported"""
    import lame
    assert lame is not None


def test_version():
    """Test version info functions"""
    import lame
    version = lame.get_version()
    assert isinstance(version, str)
    assert len(version) > 0

    url = lame.get_url()
    assert isinstance(url, str)
    assert "lame" in url.lower() or "mp3" in url.lower()


def test_quality_enum():
    """Test Quality enum"""
    import lame

    assert hasattr(lame.Quality, 'Best')
    assert hasattr(lame.Quality, 'Standard')
    assert hasattr(lame.Quality, 'Fastest')

    assert lame.Quality.Best == 0
    assert lame.Quality.Standard == 5
    assert lame.Quality.Fastest == 9


def test_vbr_mode_enum():
    """Test VbrMode enum"""
    import lame

    assert hasattr(lame.VbrMode, 'Off')
    assert hasattr(lame.VbrMode, 'Vbr')
    assert hasattr(lame.VbrMode, 'Abr')


def test_encoder_builder():
    """Test encoder builder pattern"""
    import lame

    encoder = (
        lame.LameEncoder.builder()
        .sample_rate(44100)
        .channels(1)
        .bitrate(128)
        .quality(lame.Quality.Standard)
        .build()
    )

    assert encoder is not None


def test_mono_encoding():
    """Test mono PCM encoding"""
    import lame

    encoder = (
        lame.LameEncoder.builder()
        .sample_rate(44100)
        .channels(1)
        .bitrate(128)
        .build()
    )

    # Create silence PCM data (1152 samples = 1 MP3 frame)
    pcm_data = [0] * 1152

    # Encode
    mp3_data = encoder.encode_mono(pcm_data)
    assert isinstance(mp3_data, bytes)

    # Flush
    final_data = encoder.flush()
    assert isinstance(final_data, bytes)


def test_stereo_encoding():
    """Test stereo PCM encoding"""
    import lame

    encoder = (
        lame.LameEncoder.builder()
        .sample_rate(44100)
        .channels(2)
        .bitrate(128)
        .build()
    )

    # Create silence PCM data
    pcm_left = [0] * 1152
    pcm_right = [0] * 1152

    # Encode
    mp3_data = encoder.encode(pcm_left, pcm_right)
    assert isinstance(mp3_data, bytes)

    # Flush
    final_data = encoder.flush()
    assert isinstance(final_data, bytes)


def test_interleaved_encoding():
    """Test interleaved stereo encoding"""
    import lame

    encoder = (
        lame.LameEncoder.builder()
        .sample_rate(44100)
        .channels(2)
        .bitrate(128)
        .build()
    )

    # Create interleaved silence (L, R, L, R, ...)
    pcm_interleaved = [0] * (1152 * 2)

    # Encode
    mp3_data = encoder.encode_interleaved(pcm_interleaved)
    assert isinstance(mp3_data, bytes)

    # Flush
    final_data = encoder.flush()
    assert isinstance(final_data, bytes)


def test_id3_tags():
    """Test ID3 tag functionality"""
    import lame

    encoder = (
        lame.LameEncoder.builder()
        .sample_rate(44100)
        .channels(1)
        .bitrate(128)
        .build()
    )

    # Set ID3 tags
    encoder.id3_tag() \
        .title("Test Song") \
        .artist("Test Artist") \
        .album("Test Album") \
        .year("2024") \
        .comment("Test Comment") \
        .track(1) \
        .genre("Rock") \
        .apply()

    # Encode some data
    pcm_data = [0] * 1152
    mp3_data = encoder.encode_mono(pcm_data)
    assert isinstance(mp3_data, bytes)
    assert len(mp3_data) > 0


def test_vbr_mode():
    """Test VBR encoding mode"""
    import lame

    encoder = (
        lame.LameEncoder.builder()
        .sample_rate(44100)
        .channels(1)
        .vbr_mode(lame.VbrMode.Vbr)
        .vbr_quality(4)
        .build()
    )

    pcm_data = [0] * 1152
    mp3_data = encoder.encode_mono(pcm_data)
    assert isinstance(mp3_data, bytes)


def test_different_sample_rates():
    """Test different sample rates"""
    import lame

    sample_rates = [44100, 48000, 32000, 22050, 16000]

    for rate in sample_rates:
        encoder = (
            lame.LameEncoder.builder()
            .sample_rate(rate)
            .channels(1)
            .bitrate(128)
            .build()
        )

        pcm_data = [0] * 1152
        mp3_data = encoder.encode_mono(pcm_data)
        assert isinstance(mp3_data, bytes)


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
