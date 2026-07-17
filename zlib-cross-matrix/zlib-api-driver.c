#define _POSIX_C_SOURCE 200809L
#include <errno.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <zlib.h>

struct options {
    int compress;
    int window_bits;
    int level;
    int strategy;
    int mem_level;
    int flush_mode;
    size_t block_size;
    size_t input_chunk;
    size_t output_chunk;
    const char *dictionary_path;
};

static void usage(const char *prog) {
    fprintf(stderr,
        "usage: %s compress|decompress [options]\n"
        "  --format raw|zlib|gzip\n"
        "  --level 0..9\n"
        "  --strategy default|filtered|huffman|rle|fixed\n"
        "  --mem-level 1..9\n"
        "  --dictionary PATH\n"
        "  --flush finish|sync|full|block\n"
        "  --block-size N       0 means no forced intermediate boundary\n"
        "  --input-chunk N\n"
        "  --output-chunk N\n",
        prog);
    exit(2);
}

static size_t parse_size(const char *text, const char *what, int allow_zero) {
    char *end = NULL;
    unsigned long long value;

    errno = 0;
    value = strtoull(text, &end, 10);
    if (errno || end == text || *end != '\0' || (!allow_zero && value == 0) ||
        value > (unsigned long long)UINT_MAX) {
        fprintf(stderr, "invalid %s: %s\n", what, text);
        exit(2);
    }
    return (size_t)value;
}

static int parse_format(const char *value) {
    if (strcmp(value, "raw") == 0) return -15;
    if (strcmp(value, "zlib") == 0) return 15;
    if (strcmp(value, "gzip") == 0) return 31;
    fprintf(stderr, "invalid format: %s\n", value);
    exit(2);
}

static int parse_strategy(const char *value) {
    if (strcmp(value, "default") == 0) return Z_DEFAULT_STRATEGY;
    if (strcmp(value, "filtered") == 0) return Z_FILTERED;
    if (strcmp(value, "huffman") == 0) return Z_HUFFMAN_ONLY;
    if (strcmp(value, "rle") == 0) return Z_RLE;
    if (strcmp(value, "fixed") == 0) return Z_FIXED;
    fprintf(stderr, "invalid strategy: %s\n", value);
    exit(2);
}

static int parse_flush(const char *value) {
    if (strcmp(value, "finish") == 0) return Z_NO_FLUSH;
    if (strcmp(value, "sync") == 0) return Z_SYNC_FLUSH;
    if (strcmp(value, "full") == 0) return Z_FULL_FLUSH;
    if (strcmp(value, "block") == 0) return Z_BLOCK;
    fprintf(stderr, "invalid flush mode: %s\n", value);
    exit(2);
}

static struct options parse_options(int argc, char **argv) {
    struct options o = {
        .compress = 0,
        .window_bits = 15,
        .level = 6,
        .strategy = Z_DEFAULT_STRATEGY,
        .mem_level = 8,
        .flush_mode = Z_NO_FLUSH,
        .block_size = 0,
        .input_chunk = 32768,
        .output_chunk = 32768,
        .dictionary_path = NULL,
    };
    int i;

    if (argc < 2) usage(argv[0]);
    if (strcmp(argv[1], "compress") == 0) o.compress = 1;
    else if (strcmp(argv[1], "decompress") == 0) o.compress = 0;
    else usage(argv[0]);

    for (i = 2; i < argc; ++i) {
        const char *arg = argv[i];
        const char *value;
        if (i + 1 >= argc) usage(argv[0]);
        value = argv[++i];

        if (strcmp(arg, "--format") == 0) o.window_bits = parse_format(value);
        else if (strcmp(arg, "--level") == 0) {
            o.level = (int)parse_size(value, "level", 1);
            if (o.level > 9) usage(argv[0]);
        } else if (strcmp(arg, "--strategy") == 0) {
            o.strategy = parse_strategy(value);
        } else if (strcmp(arg, "--mem-level") == 0) {
            o.mem_level = (int)parse_size(value, "mem-level", 0);
            if (o.mem_level < 1 || o.mem_level > 9) usage(argv[0]);
        } else if (strcmp(arg, "--dictionary") == 0) {
            o.dictionary_path = value;
        } else if (strcmp(arg, "--flush") == 0) {
            o.flush_mode = parse_flush(value);
        } else if (strcmp(arg, "--block-size") == 0) {
            o.block_size = parse_size(value, "block-size", 1);
        } else if (strcmp(arg, "--input-chunk") == 0) {
            o.input_chunk = parse_size(value, "input-chunk", 0);
        } else if (strcmp(arg, "--output-chunk") == 0) {
            o.output_chunk = parse_size(value, "output-chunk", 0);
        } else {
            usage(argv[0]);
        }
    }

    if (!o.compress && o.window_bits == 31 && o.dictionary_path != NULL) {
        fprintf(stderr, "gzip streams do not support preset dictionaries\n");
        exit(2);
    }
    return o;
}

static unsigned char *read_file(const char *path, uInt *length) {
    FILE *f;
    long size;
    unsigned char *data;

    f = fopen(path, "rb");
    if (f == NULL) {
        fprintf(stderr, "cannot open dictionary %s: %s\n", path, strerror(errno));
        exit(2);
    }
    if (fseek(f, 0, SEEK_END) != 0 || (size = ftell(f)) < 0 ||
        fseek(f, 0, SEEK_SET) != 0) {
        fprintf(stderr, "cannot size dictionary %s\n", path);
        fclose(f);
        exit(2);
    }
    if ((unsigned long)size > UINT_MAX) {
        fprintf(stderr, "dictionary is too large\n");
        fclose(f);
        exit(2);
    }
    data = (unsigned char *)malloc(size == 0 ? 1U : (size_t)size);
    if (data == NULL) {
        fprintf(stderr, "out of memory\n");
        fclose(f);
        exit(2);
    }
    if (size != 0 && fread(data, 1, (size_t)size, f) != (size_t)size) {
        fprintf(stderr, "cannot read dictionary %s\n", path);
        free(data);
        fclose(f);
        exit(2);
    }
    fclose(f);
    *length = (uInt)size;
    return data;
}

static int write_all(const unsigned char *data, size_t length) {
    while (length != 0) {
        size_t written = fwrite(data, 1, length, stdout);
        if (written == 0) return -1;
        data += written;
        length -= written;
    }
    return 0;
}

static int compress_stream(const struct options *o) {
    z_stream stream;
    unsigned char *input = NULL, *output = NULL, *dictionary = NULL;
    uInt dictionary_length = 0;
    size_t since_boundary = 0;
    int ret, eof = 0;

    memset(&stream, 0, sizeof(stream));
    ret = deflateInit2(&stream, o->level, Z_DEFLATED, o->window_bits,
                       o->mem_level, o->strategy);
    if (ret != Z_OK) {
        fprintf(stderr, "deflateInit2 failed: %d\n", ret);
        return 1;
    }

    if (o->dictionary_path != NULL) {
        dictionary = read_file(o->dictionary_path, &dictionary_length);
        ret = deflateSetDictionary(&stream, dictionary, dictionary_length);
        if (ret != Z_OK) {
            fprintf(stderr, "deflateSetDictionary failed: %d\n", ret);
            deflateEnd(&stream);
            free(dictionary);
            return 1;
        }
    }

    input = (unsigned char *)malloc(o->input_chunk);
    output = (unsigned char *)malloc(o->output_chunk);
    if (input == NULL || output == NULL) {
        fprintf(stderr, "out of memory\n");
        ret = 1;
        goto done;
    }

    for (;;) {
        size_t want = o->input_chunk;
        size_t got;
        int action;

        if (o->block_size != 0 && o->flush_mode != Z_NO_FLUSH) {
            size_t remaining = o->block_size - since_boundary;
            if (want > remaining) want = remaining;
        }

        got = fread(input, 1, want, stdin);
        if (got < want) {
            if (ferror(stdin)) {
                fprintf(stderr, "input read failed\n");
                ret = 1;
                goto done;
            }
            eof = feof(stdin);
        }

        stream.next_in = input;
        stream.avail_in = (uInt)got;

        if (eof) {
            action = Z_FINISH;
        } else if (o->block_size != 0 && o->flush_mode != Z_NO_FLUSH &&
                   since_boundary + got == o->block_size) {
            action = o->flush_mode;
        } else {
            action = Z_NO_FLUSH;
        }

        {
            int call_action = action;
            do {
                size_t produced;
                stream.next_out = output;
                stream.avail_out = (uInt)o->output_chunk;
                ret = deflate(&stream, call_action);
                if (ret != Z_OK && ret != Z_STREAM_END) {
                    fprintf(stderr, "deflate failed: %d (%s)\n", ret,
                            stream.msg != NULL ? stream.msg : "no message");
                    ret = 1;
                    goto done;
                }
                produced = o->output_chunk - stream.avail_out;
                if (write_all(output, produced) != 0) {
                    fprintf(stderr, "output write failed\n");
                    ret = 1;
                    goto done;
                }
                /* A periodic flush only needs to be requested once: it
                 * queues its bytes internally regardless of how much
                 * output space is available. Further calls forced by a
                 * small output buffer should just drain that queue, not
                 * re-request the same flush, since with a tiny avail_out
                 * that can make deflate() manufacture a fresh (empty)
                 * flush block forever instead of ever reporting the
                 * flush complete. */
                if (call_action != Z_NO_FLUSH && call_action != Z_FINISH) {
                    call_action = Z_NO_FLUSH;
                }
            } while ((action == Z_FINISH && ret != Z_STREAM_END) ||
                     stream.avail_in != 0 || stream.avail_out == 0);
        }

        if (action == o->flush_mode && action != Z_NO_FLUSH) {
            since_boundary = 0;
        } else {
            since_boundary += got;
        }

        if (ret == Z_STREAM_END) {
            ret = 0;
            break;
        }
    }

done:
    deflateEnd(&stream);
    free(input);
    free(output);
    free(dictionary);
    return ret;
}

static int decompress_stream(const struct options *o) {
    z_stream stream;
    unsigned char *input = NULL, *output = NULL, *dictionary = NULL;
    uInt dictionary_length = 0;
    int ret, finished = 0;

    memset(&stream, 0, sizeof(stream));
    ret = inflateInit2(&stream, o->window_bits);
    if (ret != Z_OK) {
        fprintf(stderr, "inflateInit2 failed: %d\n", ret);
        return 1;
    }

    if (o->dictionary_path != NULL) {
        dictionary = read_file(o->dictionary_path, &dictionary_length);
        if (o->window_bits < 0) {
            ret = inflateSetDictionary(&stream, dictionary, dictionary_length);
            if (ret != Z_OK) {
                fprintf(stderr, "inflateSetDictionary(raw) failed: %d\n", ret);
                inflateEnd(&stream);
                free(dictionary);
                return 1;
            }
        }
    }

    input = (unsigned char *)malloc(o->input_chunk);
    output = (unsigned char *)malloc(o->output_chunk);
    if (input == NULL || output == NULL) {
        fprintf(stderr, "out of memory\n");
        ret = 1;
        goto done;
    }

    while (!finished) {
        size_t got = fread(input, 1, o->input_chunk, stdin);
        if (got == 0) {
            if (ferror(stdin)) {
                fprintf(stderr, "input read failed\n");
            } else {
                fprintf(stderr, "truncated compressed stream\n");
            }
            ret = 1;
            goto done;
        }

        stream.next_in = input;
        stream.avail_in = (uInt)got;

        do {
            size_t produced;
            uLong before_in = stream.total_in;
            uLong before_out = stream.total_out;

            stream.next_out = output;
            stream.avail_out = (uInt)o->output_chunk;
            ret = inflate(&stream, Z_NO_FLUSH);

            if (ret == Z_NEED_DICT && dictionary != NULL) {
                ret = inflateSetDictionary(&stream, dictionary, dictionary_length);
                if (ret != Z_OK) {
                    fprintf(stderr, "inflateSetDictionary failed: %d\n", ret);
                    ret = 1;
                    goto done;
                }
                continue;
            }

            produced = o->output_chunk - stream.avail_out;
            if (write_all(output, produced) != 0) {
                fprintf(stderr, "output write failed\n");
                ret = 1;
                goto done;
            }

            if (ret == Z_STREAM_END) {
                finished = 1;
                break;
            }
            if (ret != Z_OK && ret != Z_BUF_ERROR) {
                fprintf(stderr, "inflate failed: %d (%s)\n", ret,
                        stream.msg != NULL ? stream.msg : "no message");
                ret = 1;
                goto done;
            }
            if (ret == Z_BUF_ERROR && stream.total_in == before_in &&
                stream.total_out == before_out) {
                break;
            }
        } while (stream.avail_in != 0 || stream.avail_out == 0);
    }

    ret = 0;

done:
    inflateEnd(&stream);
    free(input);
    free(output);
    free(dictionary);
    return ret;
}

int main(int argc, char **argv) {
    struct options options = parse_options(argc, argv);
    if (options.compress) return compress_stream(&options);
    return decompress_stream(&options);
}
