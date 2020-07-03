#pragma once

// Headers

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#define CL_TARGET_OPENCL_VERSION 120
#define CL_USE_DEPRECATED_OPENCL_1_2_APIS
#ifdef __APPLE__
#define CL_SILENCE_DEPRECATION
#include <OpenCL/cl.h>
#else
#include <CL/cl.h>
#endif


// Initialisation

int futhark_get_num_sizes(void);
const char *futhark_get_size_name(int);
const char *futhark_get_size_class(int);
struct futhark_context_config ;
struct futhark_context_config *futhark_context_config_new(void);
void futhark_context_config_free(struct futhark_context_config *cfg);
void futhark_context_config_add_build_option(struct futhark_context_config *cfg,
                                             const char *opt);
void futhark_context_config_set_debugging(struct futhark_context_config *cfg,
                                          int flag);
void futhark_context_config_set_profiling(struct futhark_context_config *cfg,
                                          int flag);
void futhark_context_config_set_logging(struct futhark_context_config *cfg,
                                        int flag);
void futhark_context_config_set_device(struct futhark_context_config *cfg, const
                                       char *s);
void futhark_context_config_set_platform(struct futhark_context_config *cfg,
                                         const char *s);
void
futhark_context_config_select_device_interactively(struct futhark_context_config *cfg);
void futhark_context_config_dump_program_to(struct futhark_context_config *cfg,
                                            const char *path);
void
futhark_context_config_load_program_from(struct futhark_context_config *cfg,
                                         const char *path);
void futhark_context_config_dump_binary_to(struct futhark_context_config *cfg,
                                           const char *path);
void futhark_context_config_load_binary_from(struct futhark_context_config *cfg,
                                             const char *path);
void
futhark_context_config_set_default_group_size(struct futhark_context_config *cfg,
                                              int size);
void
futhark_context_config_set_default_num_groups(struct futhark_context_config *cfg,
                                              int num);
void
futhark_context_config_set_default_tile_size(struct futhark_context_config *cfg,
                                             int num);
void
futhark_context_config_set_default_threshold(struct futhark_context_config *cfg,
                                             int num);
int futhark_context_config_set_size(struct futhark_context_config *cfg, const
                                    char *size_name, size_t size_value);
struct futhark_context ;
struct futhark_context *futhark_context_new(struct futhark_context_config *cfg);
struct futhark_context
*futhark_context_new_with_command_queue(struct futhark_context_config *cfg,
                                        cl_command_queue queue);
void futhark_context_free(struct futhark_context *ctx);
cl_command_queue futhark_context_get_command_queue(struct futhark_context *ctx);

// Arrays

struct futhark_i64_1d ;
struct futhark_i64_1d *futhark_new_i64_1d(struct futhark_context *ctx,
                                          int64_t *data, int64_t dim0);
struct futhark_i64_1d *futhark_new_raw_i64_1d(struct futhark_context *ctx,
                                              cl_mem data, int offset,
                                              int64_t dim0);
int futhark_free_i64_1d(struct futhark_context *ctx,
                        struct futhark_i64_1d *arr);
int futhark_values_i64_1d(struct futhark_context *ctx,
                          struct futhark_i64_1d *arr, int64_t *data);
cl_mem futhark_values_raw_i64_1d(struct futhark_context *ctx,
                                 struct futhark_i64_1d *arr);
const int64_t *futhark_shape_i64_1d(struct futhark_context *ctx,
                                    struct futhark_i64_1d *arr);
struct futhark_i64_2d ;
struct futhark_i64_2d *futhark_new_i64_2d(struct futhark_context *ctx,
                                          int64_t *data, int64_t dim0,
                                          int64_t dim1);
struct futhark_i64_2d *futhark_new_raw_i64_2d(struct futhark_context *ctx,
                                              cl_mem data, int offset,
                                              int64_t dim0, int64_t dim1);
int futhark_free_i64_2d(struct futhark_context *ctx,
                        struct futhark_i64_2d *arr);
int futhark_values_i64_2d(struct futhark_context *ctx,
                          struct futhark_i64_2d *arr, int64_t *data);
cl_mem futhark_values_raw_i64_2d(struct futhark_context *ctx,
                                 struct futhark_i64_2d *arr);
const int64_t *futhark_shape_i64_2d(struct futhark_context *ctx,
                                    struct futhark_i64_2d *arr);
struct futhark_u64_1d ;
struct futhark_u64_1d *futhark_new_u64_1d(struct futhark_context *ctx,
                                          uint64_t *data, int64_t dim0);
struct futhark_u64_1d *futhark_new_raw_u64_1d(struct futhark_context *ctx,
                                              cl_mem data, int offset,
                                              int64_t dim0);
int futhark_free_u64_1d(struct futhark_context *ctx,
                        struct futhark_u64_1d *arr);
int futhark_values_u64_1d(struct futhark_context *ctx,
                          struct futhark_u64_1d *arr, uint64_t *data);
cl_mem futhark_values_raw_u64_1d(struct futhark_context *ctx,
                                 struct futhark_u64_1d *arr);
const int64_t *futhark_shape_u64_1d(struct futhark_context *ctx,
                                    struct futhark_u64_1d *arr);
struct futhark_u64_2d ;
struct futhark_u64_2d *futhark_new_u64_2d(struct futhark_context *ctx,
                                          uint64_t *data, int64_t dim0,
                                          int64_t dim1);
struct futhark_u64_2d *futhark_new_raw_u64_2d(struct futhark_context *ctx,
                                              cl_mem data, int offset,
                                              int64_t dim0, int64_t dim1);
int futhark_free_u64_2d(struct futhark_context *ctx,
                        struct futhark_u64_2d *arr);
int futhark_values_u64_2d(struct futhark_context *ctx,
                          struct futhark_u64_2d *arr, uint64_t *data);
cl_mem futhark_values_raw_u64_2d(struct futhark_context *ctx,
                                 struct futhark_u64_2d *arr);
const int64_t *futhark_shape_u64_2d(struct futhark_context *ctx,
                                    struct futhark_u64_2d *arr);
struct futhark_u64_3d ;
struct futhark_u64_3d *futhark_new_u64_3d(struct futhark_context *ctx,
                                          uint64_t *data, int64_t dim0,
                                          int64_t dim1, int64_t dim2);
struct futhark_u64_3d *futhark_new_raw_u64_3d(struct futhark_context *ctx,
                                              cl_mem data, int offset,
                                              int64_t dim0, int64_t dim1,
                                              int64_t dim2);
int futhark_free_u64_3d(struct futhark_context *ctx,
                        struct futhark_u64_3d *arr);
int futhark_values_u64_3d(struct futhark_context *ctx,
                          struct futhark_u64_3d *arr, uint64_t *data);
cl_mem futhark_values_raw_u64_3d(struct futhark_context *ctx,
                                 struct futhark_u64_3d *arr);
const int64_t *futhark_shape_u64_3d(struct futhark_context *ctx,
                                    struct futhark_u64_3d *arr);

// Opaque values

struct futhark_opaque_p11_state ;
int futhark_free_opaque_p11_state(struct futhark_context *ctx,
                                  struct futhark_opaque_p11_state *obj);
struct futhark_opaque_p2_state ;
int futhark_free_opaque_p2_state(struct futhark_context *ctx,
                                 struct futhark_opaque_p2_state *obj);
struct futhark_opaque_p8_state ;
int futhark_free_opaque_p8_state(struct futhark_context *ctx,
                                 struct futhark_opaque_p8_state *obj);
struct futhark_opaque_s11_state ;
int futhark_free_opaque_s11_state(struct futhark_context *ctx,
                                  struct futhark_opaque_s11_state *obj);
struct futhark_opaque_s2_state ;
int futhark_free_opaque_s2_state(struct futhark_context *ctx,
                                 struct futhark_opaque_s2_state *obj);
struct futhark_opaque_s8_state ;
int futhark_free_opaque_s8_state(struct futhark_context *ctx,
                                 struct futhark_opaque_s8_state *obj);
struct futhark_opaque_t8_64m_state ;
int futhark_free_opaque_t8_64m_state(struct futhark_context *ctx,
                                     struct futhark_opaque_t8_64m_state *obj);

// Entry points

int futhark_entry_build_tree8_64m(struct futhark_context *ctx,
                                  struct futhark_u64_2d **out0, const
                                  struct futhark_opaque_t8_64m_state *in0, const
                                  struct futhark_u64_1d *in1);
int futhark_entry_hash8(struct futhark_context *ctx,
                        struct futhark_u64_1d **out0,
                        struct futhark_opaque_p8_state **out1, const
                        struct futhark_opaque_p8_state *in0, const
                        struct futhark_u64_1d *in1);
int futhark_entry_init11(struct futhark_context *ctx,
                         struct futhark_opaque_p11_state **out0, const
                         struct futhark_u64_1d *in0, const
                         struct futhark_u64_2d *in1, const
                         struct futhark_u64_3d *in2, const
                         struct futhark_u64_3d *in3, const
                         struct futhark_u64_3d *in4);
int futhark_entry_init11s(struct futhark_context *ctx,
                          struct futhark_opaque_s11_state **out0, const
                          struct futhark_u64_1d *in0, const
                          struct futhark_u64_2d *in1, const
                          struct futhark_u64_3d *in2, const
                          struct futhark_u64_3d *in3, const
                          struct futhark_u64_3d *in4);
int futhark_entry_init2(struct futhark_context *ctx,
                        struct futhark_opaque_p2_state **out0, const
                        struct futhark_u64_1d *in0, const
                        struct futhark_u64_2d *in1, const
                        struct futhark_u64_3d *in2, const
                        struct futhark_u64_3d *in3, const
                        struct futhark_u64_3d *in4);
int futhark_entry_init2s(struct futhark_context *ctx,
                         struct futhark_opaque_s2_state **out0, const
                         struct futhark_u64_1d *in0, const
                         struct futhark_u64_2d *in1, const
                         struct futhark_u64_3d *in2, const
                         struct futhark_u64_3d *in3, const
                         struct futhark_u64_3d *in4);
int futhark_entry_init8(struct futhark_context *ctx,
                        struct futhark_opaque_p8_state **out0, const
                        struct futhark_u64_1d *in0, const
                        struct futhark_u64_2d *in1, const
                        struct futhark_u64_3d *in2, const
                        struct futhark_u64_3d *in3, const
                        struct futhark_u64_3d *in4);
int futhark_entry_init8s(struct futhark_context *ctx,
                         struct futhark_opaque_s8_state **out0, const
                         struct futhark_u64_1d *in0, const
                         struct futhark_u64_2d *in1, const
                         struct futhark_u64_3d *in2, const
                         struct futhark_u64_3d *in3, const
                         struct futhark_u64_3d *in4);
int futhark_entry_init_t8_64m(struct futhark_context *ctx,
                              struct futhark_opaque_t8_64m_state **out0, const
                              struct futhark_u64_1d *in0, const
                              struct futhark_u64_2d *in1, const
                              struct futhark_u64_3d *in2, const
                              struct futhark_u64_3d *in3, const
                              struct futhark_u64_3d *in4);
int futhark_entry_mbatch_hash11(struct futhark_context *ctx,
                                struct futhark_u64_2d **out0,
                                struct futhark_opaque_p11_state **out1, const
                                struct futhark_opaque_p11_state *in0, const
                                struct futhark_u64_1d *in1);
int futhark_entry_mbatch_hash11s(struct futhark_context *ctx,
                                 struct futhark_u64_2d **out0,
                                 struct futhark_opaque_s11_state **out1, const
                                 struct futhark_opaque_s11_state *in0, const
                                 struct futhark_u64_1d *in1);
int futhark_entry_mbatch_hash2(struct futhark_context *ctx,
                               struct futhark_u64_2d **out0,
                               struct futhark_opaque_p2_state **out1, const
                               struct futhark_opaque_p2_state *in0, const
                               struct futhark_u64_1d *in1);
int futhark_entry_mbatch_hash2s(struct futhark_context *ctx,
                                struct futhark_u64_2d **out0,
                                struct futhark_opaque_s2_state **out1, const
                                struct futhark_opaque_s2_state *in0, const
                                struct futhark_u64_1d *in1);
int futhark_entry_mbatch_hash8(struct futhark_context *ctx,
                               struct futhark_u64_2d **out0,
                               struct futhark_opaque_p8_state **out1, const
                               struct futhark_opaque_p8_state *in0, const
                               struct futhark_u64_1d *in1);
int futhark_entry_mbatch_hash8s(struct futhark_context *ctx,
                                struct futhark_u64_2d **out0,
                                struct futhark_opaque_s8_state **out1, const
                                struct futhark_opaque_s8_state *in0, const
                                struct futhark_u64_1d *in1);
int futhark_entry_simple8(struct futhark_context *ctx,
                          struct futhark_u64_2d **out0, const int32_t in0);

// Miscellaneous

int futhark_context_sync(struct futhark_context *ctx);
int futhark_context_clear_caches(struct futhark_context *ctx);
char *futhark_context_report(struct futhark_context *ctx);
char *futhark_context_get_error(struct futhark_context *ctx);
void futhark_context_pause_profiling(struct futhark_context *ctx);
void futhark_context_unpause_profiling(struct futhark_context *ctx);
#define FUTHARK_BACKEND_opencl
