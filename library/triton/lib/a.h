#pragma once

/*
 * Headers
*/

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>


/*
 * Initialisation
*/

struct futhark_context_config ;
struct futhark_context_config *futhark_context_config_new(void);
void futhark_context_config_free(struct futhark_context_config *cfg);
void futhark_context_config_set_debugging(struct futhark_context_config *cfg,
                                          int flag);
void futhark_context_config_set_logging(struct futhark_context_config *cfg,
                                        int flag);
struct futhark_context ;
struct futhark_context *futhark_context_new(struct futhark_context_config *cfg);
void futhark_context_free(struct futhark_context *ctx);
int futhark_context_sync(struct futhark_context *ctx);
char *futhark_context_get_error(struct futhark_context *ctx);
void futhark_context_pause_profiling(struct futhark_context *ctx);
void futhark_context_unpause_profiling(struct futhark_context *ctx);

/*
 * Arrays
*/

struct futhark_i64_1d ;
struct futhark_i64_1d *futhark_new_i64_1d(struct futhark_context *ctx,
                                          int64_t *data, int64_t dim0);
struct futhark_i64_1d *futhark_new_raw_i64_1d(struct futhark_context *ctx,
                                              char *data, int offset,
                                              int64_t dim0);
int futhark_free_i64_1d(struct futhark_context *ctx,
                        struct futhark_i64_1d *arr);
int futhark_values_i64_1d(struct futhark_context *ctx,
                          struct futhark_i64_1d *arr, int64_t *data);
char *futhark_values_raw_i64_1d(struct futhark_context *ctx,
                                struct futhark_i64_1d *arr);
int64_t *futhark_shape_i64_1d(struct futhark_context *ctx,
                              struct futhark_i64_1d *arr);
struct futhark_i64_2d ;
struct futhark_i64_2d *futhark_new_i64_2d(struct futhark_context *ctx,
                                          int64_t *data, int64_t dim0,
                                          int64_t dim1);
struct futhark_i64_2d *futhark_new_raw_i64_2d(struct futhark_context *ctx,
                                              char *data, int offset,
                                              int64_t dim0, int64_t dim1);
int futhark_free_i64_2d(struct futhark_context *ctx,
                        struct futhark_i64_2d *arr);
int futhark_values_i64_2d(struct futhark_context *ctx,
                          struct futhark_i64_2d *arr, int64_t *data);
char *futhark_values_raw_i64_2d(struct futhark_context *ctx,
                                struct futhark_i64_2d *arr);
int64_t *futhark_shape_i64_2d(struct futhark_context *ctx,
                              struct futhark_i64_2d *arr);
struct futhark_u64_1d ;
struct futhark_u64_1d *futhark_new_u64_1d(struct futhark_context *ctx,
                                          uint64_t *data, int64_t dim0);
struct futhark_u64_1d *futhark_new_raw_u64_1d(struct futhark_context *ctx,
                                              char *data, int offset,
                                              int64_t dim0);
int futhark_free_u64_1d(struct futhark_context *ctx,
                        struct futhark_u64_1d *arr);
int futhark_values_u64_1d(struct futhark_context *ctx,
                          struct futhark_u64_1d *arr, uint64_t *data);
char *futhark_values_raw_u64_1d(struct futhark_context *ctx,
                                struct futhark_u64_1d *arr);
int64_t *futhark_shape_u64_1d(struct futhark_context *ctx,
                              struct futhark_u64_1d *arr);
struct futhark_u64_2d ;
struct futhark_u64_2d *futhark_new_u64_2d(struct futhark_context *ctx,
                                          uint64_t *data, int64_t dim0,
                                          int64_t dim1);
struct futhark_u64_2d *futhark_new_raw_u64_2d(struct futhark_context *ctx,
                                              char *data, int offset,
                                              int64_t dim0, int64_t dim1);
int futhark_free_u64_2d(struct futhark_context *ctx,
                        struct futhark_u64_2d *arr);
int futhark_values_u64_2d(struct futhark_context *ctx,
                          struct futhark_u64_2d *arr, uint64_t *data);
char *futhark_values_raw_u64_2d(struct futhark_context *ctx,
                                struct futhark_u64_2d *arr);
int64_t *futhark_shape_u64_2d(struct futhark_context *ctx,
                              struct futhark_u64_2d *arr);
struct futhark_u64_3d ;
struct futhark_u64_3d *futhark_new_u64_3d(struct futhark_context *ctx,
                                          uint64_t *data, int64_t dim0,
                                          int64_t dim1, int64_t dim2);
struct futhark_u64_3d *futhark_new_raw_u64_3d(struct futhark_context *ctx,
                                              char *data, int offset,
                                              int64_t dim0, int64_t dim1,
                                              int64_t dim2);
int futhark_free_u64_3d(struct futhark_context *ctx,
                        struct futhark_u64_3d *arr);
int futhark_values_u64_3d(struct futhark_context *ctx,
                          struct futhark_u64_3d *arr, uint64_t *data);
char *futhark_values_raw_u64_3d(struct futhark_context *ctx,
                                struct futhark_u64_3d *arr);
int64_t *futhark_shape_u64_3d(struct futhark_context *ctx,
                              struct futhark_u64_3d *arr);

/*
 * Opaque values
*/

struct futhark_opaque_8fa02cab ;
int futhark_free_opaque_8fa02cab(struct futhark_context *ctx,
                                 struct futhark_opaque_8fa02cab *obj);

/*
 * Entry points
*/

int futhark_entry_finalize(struct futhark_context *ctx,
                           struct futhark_u64_2d **out0, const
                           struct futhark_opaque_8fa02cab *in0);
int futhark_entry_simple11(struct futhark_context *ctx,
                           struct futhark_u64_2d **out0, const int32_t in0);
int futhark_entry_simple8(struct futhark_context *ctx,
                          struct futhark_u64_2d **out0, const int32_t in0);
int futhark_entry_simple2(struct futhark_context *ctx,
                          struct futhark_u64_2d **out0, const int32_t in0);
int futhark_entry_add_columns(struct futhark_context *ctx,
                              struct futhark_opaque_8fa02cab **out0, const
                              struct futhark_opaque_8fa02cab *in0, const
                              int32_t in1, const struct futhark_u64_1d *in2);
int futhark_entry_init(struct futhark_context *ctx,
                       struct futhark_opaque_8fa02cab **out0, const
                       struct futhark_u64_1d *in0, const
                       struct futhark_u64_2d *in1, const
                       struct futhark_u64_3d *in2, const
                       struct futhark_u64_3d *in3, const
                       struct futhark_u64_3d *in4, const
                       struct futhark_u64_1d *in5, const
                       struct futhark_u64_2d *in6, const
                       struct futhark_u64_3d *in7, const
                       struct futhark_u64_3d *in8, const
                       struct futhark_u64_3d *in9);

/*
 * Miscellaneous
*/

void futhark_debugging_report(struct futhark_context *ctx);
