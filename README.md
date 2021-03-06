# tensorflux-sys [![Version][version-icon]][version-page] [![Status][status-icon]][status-page]

The package provides bindings to [TensorFlow][tensorflow].

## [Documentation][documentation]

## [Example](examples/multiplication.rs)

## Requirements

The build prerequisites can be found on the [corresponding
page][tensorflow-setup] of TensorFlow’s documentation. In particular,
[Bazel][bazel], [NumPy][numpy], and [SWIG][swig] are assumed to be installed.

## Configuration

The compilation process is configured via a number of environment variables, all
of which can be found in TensorFlow’s [configure][tensorflow-configure] script.
In particular, `TF_NEED_CUDA` is used to indicate if GPU support is needed.

## Collaboration

Rust has an IRC culture, and most real-time collaborations happen in a variety
of channels on Mozilla’s IRC network, irc.mozilla.org. The channels that are
relevant to TensorFlow are #rust-machine-learning and #rust-tensorflow.

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[bazel]: http://www.bazel.io
[numpy]: http://www.numpy.org
[swig]: http://www.swig.org
[tensorflow]: https://www.tensorflow.org
[tensorflow-configure]: https://github.com/tensorflow/tensorflow/blob/r0.9/configure
[tensorflow-setup]: https://www.tensorflow.org/versions/r0.9/get_started/os_setup.html

[documentation]: https://stainless-steel.github.io/tensorflux-sys
[status-icon]: https://travis-ci.org/stainless-steel/tensorflux-sys.svg?branch=master
[status-page]: https://travis-ci.org/stainless-steel/tensorflux-sys
[version-icon]: https://img.shields.io/crates/v/tensorflux-sys.svg
[version-page]: https://crates.io/crates/tensorflux-sys
