import numpy

def denoise_decision_tree(
    samnples: numpy.ndarray,
    window_size: int,
    stride: int,
) -> numpy.ndarray:
    """Denoise a set of time-series samples using decision tree models.

    Args:
        samples: A 2D numpy array of shape (n_samples, n_features) representing
         the time-series samples.
        window_size: The number of elements in each window.
        stride: The number of elements to skip between windows.

    Returns:
        The denoised time-series samples as a 2D numpy array of the same shape
        as the input samples.
    """
    ...

def denoise_linear_regression(
    samnples: numpy.ndarray,
    window_size: int,
    stride: int,
) -> numpy.ndarray:
    """Denoise a set of time-series samples using linear regression

    Args:
        samples: A 2D numpy array of shape (n_samples, n_features) representing
         the time-series samples.
        window_size: The number of elements in each window.
        stride: The number of elements to skip between windows.

    Returns:
        The denoised time-series samples as a 2D numpy array of the same shape
        as the input samples.
    """
    ...
