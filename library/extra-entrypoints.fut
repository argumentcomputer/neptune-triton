import "poseidon"

entry hash11 (s: p11_state) (preimage_u64s: [44]u64) : ([p11.Field.LIMBS]u64, p11_state) =
  let preimage = map p11.Field.mont_from_u64s (even_chunks p11.Field.LIMBS preimage_u64s) in
  let hash = p11.Field.mont_to_u64s (p11.hash_preimage s (preimage :> [p11.arity]p11.Field.t)) in
  (hash, s)

entry test2 (input_u64s: [12]u64) : [p2.Field.LIMBS]u64 =
  let input = map p2.Field.mont_from_u64s (even_chunks p2.Field.LIMBS input_u64s) in
  let res = p2.Field.(input[0] * input[1] + input[2]) in
  p2.Field.mont_to_u64s res

let x2 = p2.init p2.blank_constants
entry simple2 n = tabulate n (\i -> p2.Field.mont_to_u64s
                                    (p2.hash_preimage x2 ((replicate 2 (p2.Field.mont_from_u32 (u32.i32 i)) :> [p2.arity]p2.Field.t))))

let x8 = p8.init p8.blank_constants
entry simple8 n = tabulate n (\i -> p8.Field.mont_to_u64s
                                    (p8.hash_preimage x8 ((replicate 8 (p8.Field.mont_from_u32 (u32.i32 i)) :> [p8.arity]p8.Field.t))))

let x11 = p11.init p11.blank_constants
entry simple11 n = tabulate n (\i -> p11.Field.mont_to_u64s
                                     (p11.hash_preimage x11 ((replicate 11 (p11.Field.mont_from_u32 (u32.i32 i)) :> [p11.arity]p11.Field.t))))

entry batch_hash2 [u64_count] (s: p2_state) (u64s: [u64_count]u64): ([][p2.Field.LIMBS]u64, p2_state) = p2.hash_preimages_u64s s u64s
entry batch_hash8 [u64_count] (s: p8_state) (u64s: [u64_count]u64): ([][p8.Field.LIMBS]u64, p8_state) = p8.hash_preimages_u64s s u64s
entry batch_hash11 [u64_count] (s: p11_state) (u64s: [u64_count]u64): ([][p11.Field.LIMBS]u64, p11_state) = p11.hash_preimages_u64s s u64s

  -- 4 GiB trees
-- This hardcodes:
-- Column arity = 11
-- Tree arity   = 8
-- Tree size    = 4 GiB

module ctb_4g = make_column_tree_builder p11 t8_4g
type ctb_4g_state = ctb_4g.state

module colhasher_4g = ctb_4g.ColumnHasher
module treehasher_4g = ctb_4g.TreeBuilder.Hasher

entry init_4g (treehasher_arity_tag: ([treehasher_4g.Field.LIMBS]u64))
           (treehasher_round_keys: [treehasher_4g.rk_count]([treehasher_4g.Field.LIMBS]u64))
           (treehasher_mds_matrix: matrix ([treehasher_4g.Field.LIMBS]u64) [treehasher_4g.width])
           (treehasher_pre_sparse_matrix: matrix ([treehasher_4g.Field.LIMBS]u64) [treehasher_4g.width])
           (treehasher_sparse_matrixes: [treehasher_4g.sparse_count][treehasher_4g.sparse_matrix_size]([treehasher_4g.Field.LIMBS]u64))
           (colhasher_arity_tag: ([colhasher_4g.Field.LIMBS]u64))
           (colhasher_round_keys: [colhasher_4g.rk_count]([colhasher_4g.Field.LIMBS]u64))
           (colhasher_mds_matrix: matrix ([colhasher_4g.Field.LIMBS]u64) [colhasher_4g.width])
           (colhasher_pre_sparse_matrix: matrix ([colhasher_4g.Field.LIMBS]u64) [colhasher_4g.width])
           (colhasher_sparse_matrixes: [colhasher_4g.sparse_count][colhasher_4g.sparse_matrix_size]([colhasher_4g.Field.LIMBS]u64))
           : ctb_4g_state =
  let treehasher_constants = treehasher_4g.make_constants treehasher_arity_tag treehasher_round_keys treehasher_mds_matrix treehasher_pre_sparse_matrix treehasher_sparse_matrixes in
  let colhasher_constants =   colhasher_4g.make_constants colhasher_arity_tag colhasher_round_keys colhasher_mds_matrix colhasher_pre_sparse_matrix colhasher_sparse_matrixes in
  ctb_4g.init (colhasher_4g.init colhasher_constants) (treehasher_4g.init treehasher_constants)

entry add_columns_4g [u64_count] (s: ctb_4g_state) (columns: [u64_count]u64): ctb_4g_state =
  let column_count = div_evenly u64_count colhasher_4g.Field.LIMBS in
  ctb_4g.add_columns s (map colhasher_4g.Field.mont_from_u64s (unflatten column_count colhasher_4g.Field.LIMBS columns))

entry finalize_4g (s: ctb_4g_state): ([ctb_4g.TreeBuilder.tree_size][treehasher_4g.Field.LIMBS]u64, ctb_4g_state) =
  ctb_4g.finalize s


-- -- 2 KiB trees
-- -- This hardcodes:
-- -- Column arity = 11
-- -- Tree arity   = 8
-- -- Tree size    = 2 KiB

module ctb_2k = make_column_tree_builder p11 t8_2k
type ctb_2k_state = ctb_2k.state

module colhasher_2k = ctb_2k.ColumnHasher
module treehasher_2k = ctb_2k.TreeBuilder.Hasher

entry init_2k (treehasher_arity_tag: ([treehasher_2k.Field.LIMBS]u64))
           (treehasher_round_keys: [treehasher_2k.rk_count]([treehasher_2k.Field.LIMBS]u64))
           (treehasher_mds_matrix: matrix ([treehasher_2k.Field.LIMBS]u64) [treehasher_2k.width])
           (treehasher_pre_sparse_matrix: matrix ([treehasher_2k.Field.LIMBS]u64) [treehasher_2k.width])
           (treehasher_sparse_matrixes: [treehasher_2k.sparse_count][treehasher_2k.sparse_matrix_size]([treehasher_2k.Field.LIMBS]u64))
           (colhasher_arity_tag: ([colhasher_2k.Field.LIMBS]u64))
           (colhasher_round_keys: [colhasher_2k.rk_count]([colhasher_2k.Field.LIMBS]u64))
           (colhasher_mds_matrix: matrix ([colhasher_2k.Field.LIMBS]u64) [colhasher_2k.width])
           (colhasher_pre_sparse_matrix: matrix ([colhasher_2k.Field.LIMBS]u64) [colhasher_2k.width])
           (colhasher_sparse_matrixes: [colhasher_2k.sparse_count][colhasher_2k.sparse_matrix_size]([colhasher_2k.Field.LIMBS]u64))
           : ctb_2k_state =
  let treehasher_constants = treehasher_2k.make_constants treehasher_arity_tag treehasher_round_keys treehasher_mds_matrix treehasher_pre_sparse_matrix treehasher_sparse_matrixes in
  let colhasher_constants = colhasher_2k.make_constants colhasher_arity_tag colhasher_round_keys colhasher_mds_matrix colhasher_pre_sparse_matrix colhasher_sparse_matrixes in
  ctb_2k.init (colhasher_2k.init colhasher_constants) (treehasher_2k.init treehasher_constants)

entry add_columns_2k [u64_count] (s: ctb_2k_state) (columns: [u64_count]u64): ctb_2k_state =
  let column_count = div_evenly u64_count colhasher_2k.Field.LIMBS in
  ctb_2k.add_columns s (map colhasher_2k.Field.mont_from_u64s (unflatten column_count colhasher_2k.Field.LIMBS columns))

entry finalize_2k (s: ctb_2k_state): ([ctb_2k.TreeBuilder.tree_size][treehasher_2k.Field.LIMBS]u64, ctb_2k_state) =
  ctb_2k.finalize s

module ctb_512m = make_column_tree_builder p11 t8_512m
type ctb_512m_state = ctb_512m.state

module colhasher_512m = ctb_512m.ColumnHasher
module treehasher_512m = ctb_512m.TreeBuilder.Hasher

entry init_512m (treehasher_arity_tag: ([treehasher_512m.Field.LIMBS]u64))
           (treehasher_round_keys: [treehasher_512m.rk_count]([treehasher_512m.Field.LIMBS]u64))
           (treehasher_mds_matrix: matrix ([treehasher_512m.Field.LIMBS]u64) [treehasher_512m.width])
           (treehasher_pre_sparse_matrix: matrix ([treehasher_512m.Field.LIMBS]u64) [treehasher_512m.width])
           (treehasher_sparse_matrixes: [treehasher_512m.sparse_count][treehasher_512m.sparse_matrix_size]([treehasher_512m.Field.LIMBS]u64))
           (colhasher_arity_tag: ([colhasher_512m.Field.LIMBS]u64))
           (colhasher_round_keys: [colhasher_512m.rk_count]([colhasher_512m.Field.LIMBS]u64))
           (colhasher_mds_matrix: matrix ([colhasher_512m.Field.LIMBS]u64) [colhasher_512m.width])
           (colhasher_pre_sparse_matrix: matrix ([colhasher_512m.Field.LIMBS]u64) [colhasher_512m.width])
           (colhasher_sparse_matrixes: [colhasher_512m.sparse_count][colhasher_512m.sparse_matrix_size]([colhasher_512m.Field.LIMBS]u64))
           : ctb_512m_state =
  let treehasher_constants = treehasher_512m.make_constants treehasher_arity_tag treehasher_round_keys treehasher_mds_matrix treehasher_pre_sparse_matrix treehasher_sparse_matrixes in
  let colhasher_constants = colhasher_512m.make_constants colhasher_arity_tag colhasher_round_keys colhasher_mds_matrix colhasher_pre_sparse_matrix colhasher_sparse_matrixes in
  ctb_512m.init (colhasher_512m.init colhasher_constants) (treehasher_512m.init treehasher_constants)

entry add_columns_512m [u64_count] (s: ctb_512m_state) (columns: [u64_count]u64): ctb_512m_state =
  let column_count = div_evenly u64_count colhasher_512m.Field.LIMBS in
  ctb_512m.add_columns s (map colhasher_512m.Field.mont_from_u64s (unflatten column_count colhasher_512m.Field.LIMBS columns))

entry finalize_512m (s: ctb_512m_state): ([ctb_512m.TreeBuilder.tree_size][treehasher_512m.Field.LIMBS]u64, ctb_512m_state) =
  ctb_512m.finalize s
