#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import itertools
import json
import os
import random
import shutil
import subprocess
import sys
import tempfile
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Iterable, Sequence


@dataclass(frozen=True)
class Implementation:
    name: str
    driver: Path


@dataclass(frozen=True)
class CompressionConfig:
    format: str
    level: int
    strategy: str
    mem_level: int
    dictionary: str
    layout: str
    flush: str
    block_size: int
    input_chunk: int
    output_chunk: int
    coding_class: str

    @property
    def name(self) -> str:
        return (
            f"{self.format}-l{self.level}-{self.strategy}-m{self.mem_level}-"
            f"dict-{self.dictionary}-{self.layout}-{self.flush}-"
            f"b{self.block_size}-i{self.input_chunk}-o{self.output_chunk}-"
            f"{self.coding_class}"
        )


@dataclass(frozen=True)
class DecompressionConfig:
    input_chunk: int
    output_chunk: int


@dataclass
class Counts:
    passed: int = 0
    failed: int = 0


def csv_strings(text: str) -> list[str]:
    values = [item.strip() for item in text.split(",") if item.strip()]
    if not values:
        raise argparse.ArgumentTypeError("list must not be empty")
    return values


def csv_ints(text: str) -> list[int]:
    try:
        values = [int(item.strip()) for item in text.split(",") if item.strip()]
    except ValueError as exc:
        raise argparse.ArgumentTypeError(str(exc)) from exc
    if not values:
        raise argparse.ArgumentTypeError("list must not be empty")
    return values


def discover_files(root: Path, recursive: bool) -> list[Path]:
    iterator: Iterable[Path] = root.rglob("*") if recursive else root.iterdir()
    files = sorted(path for path in iterator if path.is_file() and not path.is_symlink())
    if not files:
        raise RuntimeError(f"no regular files found under {root}")
    return files


def hash_file(path: Path) -> bytes:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        while chunk := stream.read(1024 * 1024):
            digest.update(chunk)
    return digest.digest()


def make_dictionary(files: Sequence[Path], output: Path) -> None:
    data = bytearray()
    for path in files:
        with path.open("rb") as stream:
            while len(data) < 32768:
                chunk = stream.read(32768 - len(data))
                if not chunk:
                    break
                data.extend(chunk)
        if len(data) == 32768:
            break
    if not data:
        data.extend(b"zlib preset dictionary")
    output.write_bytes(bytes(data[-32768:]))


def coding_class(level: int, strategy: str) -> str:
    # This is the requested encoding profile. DEFLATE encoders may still choose
    # a stored block for an individual incompressible block.
    if level == 0:
        return "stored"
    if strategy == "fixed":
        return "fixed"
    return "dynamic"


def generate_configs(args: argparse.Namespace) -> list[CompressionConfig]:
    configs: list[CompressionConfig] = []

    for (
        fmt,
        level,
        strategy,
        mem_level,
        dictionary,
        layout,
        flush,
        block_size,
        input_chunk,
        output_chunk,
    ) in itertools.product(
        args.formats,
        args.levels,
        args.strategies,
        args.mem_levels,
        args.dictionary_modes,
        args.layouts,
        args.flush_modes,
        args.block_sizes,
        args.input_chunks,
        args.output_chunks,
    ):
        # Preset dictionaries are supported by raw DEFLATE and the zlib wrapper,
        # but not by the gzip wrapper.
        if fmt == "gzip" and dictionary == "preset":
            continue

        # "single block" means no application-forced intermediate flush.
        if layout == "single":
            if flush != "finish" or block_size != 0:
                continue
        else:
            if flush == "finish" or block_size == 0:
                continue

        klass = coding_class(level, strategy)
        if klass not in args.coding_classes:
            continue

        configs.append(
            CompressionConfig(
                format=fmt,
                level=level,
                strategy=strategy,
                mem_level=mem_level,
                dictionary=dictionary,
                layout=layout,
                flush=flush,
                block_size=block_size,
                input_chunk=input_chunk,
                output_chunk=output_chunk,
                coding_class=klass,
            )
        )

    configs.sort(key=lambda config: config.name)
    return configs


def run_filter(
    command: Sequence[str],
    input_path: Path,
    output_path: Path,
    timeout: float,
) -> str | None:
    try:
        with input_path.open("rb") as stdin, output_path.open("wb") as stdout:
            completed = subprocess.run(
                command,
                stdin=stdin,
                stdout=stdout,
                stderr=subprocess.PIPE,
                timeout=timeout,
                check=False,
            )
    except subprocess.TimeoutExpired:
        return f"timeout after {timeout:g}s"
    except OSError as exc:
        return str(exc)

    if completed.returncode == 0:
        return None

    stderr = completed.stderr.decode("utf-8", errors="replace").strip()
    return f"exit {completed.returncode}" + (f": {stderr}" if stderr else "")


def compression_command(
    impl: Implementation,
    config: CompressionConfig,
    dictionary: Path,
) -> list[str]:
    command = [
        str(impl.driver),
        "compress",
        "--format",
        config.format,
        "--level",
        str(config.level),
        "--strategy",
        config.strategy,
        "--mem-level",
        str(config.mem_level),
        "--flush",
        config.flush,
        "--block-size",
        str(config.block_size),
        "--input-chunk",
        str(config.input_chunk),
        "--output-chunk",
        str(config.output_chunk),
    ]
    if config.dictionary == "preset":
        command += ["--dictionary", str(dictionary)]
    return command


def decompression_command(
    impl: Implementation,
    compression: CompressionConfig,
    decompression: DecompressionConfig,
    dictionary: Path,
) -> list[str]:
    command = [
        str(impl.driver),
        "decompress",
        "--format",
        compression.format,
        "--input-chunk",
        str(decompression.input_chunk),
        "--output-chunk",
        str(decompression.output_chunk),
    ]
    if compression.dictionary == "preset":
        command += ["--dictionary", str(dictionary)]
    return command


def preserve_failure(
    root: Path,
    source: Path,
    producer: Implementation,
    consumer: Implementation | None,
    config: CompressionConfig,
    decompression: DecompressionConfig | None,
    compressed: Path | None,
    output: Path | None,
    reason: str,
) -> None:
    digest = hashlib.sha256(
        (str(source) + producer.name + str(consumer) + config.name + reason).encode()
    ).hexdigest()[:16]
    destination = root / digest
    destination.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source, destination / "input")
    if compressed is not None and compressed.exists():
        shutil.copy2(compressed, destination / "compressed")
    if output is not None and output.exists():
        shutil.copy2(output, destination / "output")
    metadata = {
        "source": str(source),
        "producer": producer.name,
        "consumer": consumer.name if consumer else None,
        "compression": asdict(config),
        "decompression": asdict(decompression) if decompression else None,
        "reason": reason,
    }
    (destination / "failure.json").write_text(
        json.dumps(metadata, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Cross-test two zlib implementations over a parameter matrix."
    )
    parser.add_argument("corpus", type=Path)
    parser.add_argument("--target-driver", type=Path, required=True)
    parser.add_argument("--reference-driver", type=Path, required=True)
    parser.add_argument("--recursive", action="store_true")
    parser.add_argument("--levels", type=csv_ints, default=csv_ints("0,1,6,9"))
    parser.add_argument(
        "--strategies",
        type=csv_strings,
        default=csv_strings("default,filtered,huffman,rle,fixed"),
    )
    parser.add_argument(
        "--mem-levels", type=csv_ints, default=csv_ints("1,8,9")
    )
    parser.add_argument(
        "--formats", type=csv_strings, default=csv_strings("raw,zlib,gzip")
    )
    parser.add_argument(
        "--dictionary-modes",
        type=csv_strings,
        default=csv_strings("none,preset"),
    )
    parser.add_argument(
        "--coding-classes",
        type=csv_strings,
        default=csv_strings("stored,fixed,dynamic"),
        help="derived coding profiles to retain",
    )
    parser.add_argument(
        "--layouts", type=csv_strings, default=csv_strings("single,many")
    )
    parser.add_argument(
        "--flush-modes",
        type=csv_strings,
        default=csv_strings("finish,sync,full,block"),
    )
    parser.add_argument(
        "--block-sizes",
        type=csv_ints,
        default=csv_ints("0,1024"),
        help="0 for single-stream layout; positive sizes for many-block layouts",
    )
    parser.add_argument(
        "--input-chunks", type=csv_ints, default=csv_ints("1")
    )
    parser.add_argument(
        "--output-chunks", type=csv_ints, default=csv_ints("1,2,7")
    )
    parser.add_argument(
        "--decompress-input-chunks", type=csv_ints, default=csv_ints("1")
    )
    parser.add_argument(
        "--decompress-output-chunks", type=csv_ints, default=csv_ints("1,2,7")
    )
    parser.add_argument(
        "--sample-configs",
        type=int,
        default=0,
        help="deterministically sample this many compression configs; 0 means all",
    )
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--timeout", type=float, default=120.0)
    parser.add_argument("--keep-failures", type=Path)
    parser.add_argument("--verbose", action="store_true")
    args = parser.parse_args()

    corpus = args.corpus.resolve()
    if not corpus.is_dir():
        parser.error(f"not a directory: {corpus}")

    implementations = [
        Implementation("target", args.target_driver.resolve()),
        Implementation("reference", args.reference_driver.resolve()),
    ]
    for impl in implementations:
        if not impl.driver.is_file() or not os.access(impl.driver, os.X_OK):
            parser.error(f"driver is not executable: {impl.driver}")

    try:
        files = discover_files(corpus, args.recursive)
    except RuntimeError as exc:
        parser.error(str(exc))

    configs = generate_configs(args)
    if not configs:
        parser.error("the selected dimensions produce no valid configurations")

    if args.sample_configs:
        if args.sample_configs < 0:
            parser.error("--sample-configs must be nonnegative")
        if args.sample_configs < len(configs):
            configs = sorted(
                random.Random(args.seed).sample(configs, args.sample_configs),
                key=lambda config: config.name,
            )

    decompression_configs = [
        DecompressionConfig(input_chunk, output_chunk)
        for input_chunk, output_chunk in itertools.product(
            args.decompress_input_chunks, args.decompress_output_chunks
        )
    ]

    compression_runs = len(files) * len(configs) * len(implementations)
    decompression_runs = compression_runs * len(implementations) * len(decompression_configs)
    print(f"files:                 {len(files)}")
    print(f"compression configs:   {len(configs)}")
    print(f"compression runs:      {compression_runs}")
    print(f"decompression runs:    {decompression_runs}")

    if args.keep_failures is not None:
        args.keep_failures.mkdir(parents=True, exist_ok=True)

    counts: dict[tuple[str, str], Counts] = {
        (producer.name, consumer.name): Counts()
        for producer in implementations
        for consumer in implementations
    }

    with tempfile.TemporaryDirectory(prefix="cross-zlib-") as temporary:
        temp = Path(temporary)
        dictionary = temp / "dictionary.bin"
        make_dictionary(files, dictionary)

        for file_number, source in enumerate(files, 1):
            expected_size = source.stat().st_size
            expected_hash = hash_file(source)
            print(f"[{file_number}/{len(files)}] {source.relative_to(corpus)}")

            for config_number, config in enumerate(configs, 1):
                for producer in implementations:
                    compressed = temp / "compressed.bin"
                    error = run_filter(
                        compression_command(producer, config, dictionary),
                        source,
                        compressed,
                        args.timeout,
                    )
                    if error is not None:
                        print(
                            f"FAIL compress {producer.name} {config.name}: {error}",
                            file=sys.stderr,
                        )
                        if args.keep_failures is not None:
                            preserve_failure(
                                args.keep_failures,
                                source,
                                producer,
                                None,
                                config,
                                None,
                                compressed,
                                None,
                                error,
                            )
                        for consumer in implementations:
                            counts[(producer.name, consumer.name)].failed += len(
                                decompression_configs
                            )
                        continue

                    for consumer in implementations:
                        for decompression in decompression_configs:
                            output = temp / "output.bin"
                            error = run_filter(
                                decompression_command(
                                    consumer, config, decompression, dictionary
                                ),
                                compressed,
                                output,
                                args.timeout,
                            )
                            if error is None and (
                                output.stat().st_size != expected_size
                                or hash_file(output) != expected_hash
                            ):
                                error = (
                                    f"output mismatch: expected {expected_size} bytes, "
                                    f"got {output.stat().st_size}"
                                )

                            bucket = counts[(producer.name, consumer.name)]
                            if error is None:
                                bucket.passed += 1
                                if args.verbose:
                                    print(
                                        f"PASS {producer.name}->{consumer.name} "
                                        f"{config.name} di{decompression.input_chunk} "
                                        f"do{decompression.output_chunk}"
                                    )
                            else:
                                bucket.failed += 1
                                print(
                                    f"FAIL {producer.name}->{consumer.name} "
                                    f"{config.name} di{decompression.input_chunk} "
                                    f"do{decompression.output_chunk}: {error}",
                                    file=sys.stderr,
                                )
                                if args.keep_failures is not None:
                                    preserve_failure(
                                        args.keep_failures,
                                        source,
                                        producer,
                                        consumer,
                                        config,
                                        decompression,
                                        compressed,
                                        output,
                                        error,
                                    )

    print("\nproducer -> consumer        passed     failed")
    total_failed = 0
    for key, result in sorted(counts.items()):
        print(f"{key[0]:9} -> {key[1]:9} {result.passed:10d} {result.failed:10d}")
        total_failed += result.failed

    if total_failed:
        print(f"FAILED: {total_failed} checks", file=sys.stderr)
        return 1
    print("PASSED")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
