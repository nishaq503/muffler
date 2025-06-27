import pathlib

import numpy
import typer

from muffler import denoise_linear_regression

cli = typer.Typer()


@cli.command()
def main(
    inp_path: pathlib.Path = typer.Option(
        ...,
        "--inp-path",
        "-i",
        help="Path to the input file.",
        exists=True,
        file_okay=True,
        dir_okay=False,
        readable=True,
        resolve_path=True,
    ),
    out_dir: pathlib.Path = typer.Option(
        ...,
        "--out-dir",
        "-o",
        help="Directory to save the output file.",
        exists=True,
        file_okay=False,
        dir_okay=True,
        writable=True,
        resolve_path=True,
    ),
) -> None:
    """Solve a CNF file using the muffler package.

    Args:
        inp_path: Path to the input file.
        out_dir: Directory to save the output file.
    """
    print("Running muffler from Python CLI...")
    inp_array = numpy.load(inp_path)
    out_array = denoise_linear_regression(inp_array, 128, 16)
    out_path = out_dir / inp_path.name.replace(".npy", "_denoised.npy")
    numpy.save(out_path, out_array)
    print(f"Output saved to {out_path}")


cli()
