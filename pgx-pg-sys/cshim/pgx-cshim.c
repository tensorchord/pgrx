/*
 * Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
 * governed by the MIT license that can be found in the LICENSE file.
 */

#include "postgres.h"

#define IS_PG_10 (PG_VERSION_NUM >= 100000 && PG_VERSION_NUM < 110000)
#define IS_PG_11 (PG_VERSION_NUM >= 110000 && PG_VERSION_NUM < 120000)
#define IS_PG_12 (PG_VERSION_NUM >= 120000 && PG_VERSION_NUM < 130000)

#include "access/htup.h"
#include "access/htup_details.h"
#include "catalog/pg_type.h"
#if IS_PG_10 || IS_PG_11
#include "nodes/relation.h"
#else
#include "nodes/pathnodes.h"
#endif
#include "parser/parsetree.h"
#include "utils/memutils.h"
#include "utils/builtins.h"


PGDLLEXPORT MemoryContext pgx_GetMemoryContextChunk(void *ptr);
MemoryContext pgx_GetMemoryContextChunk(void *ptr) {
    return GetMemoryChunkContext(ptr);
}

PGDLLEXPORT void pgx_elog(int32 level, char *message);
void pgx_elog(int32 level, char *message) {
    elog(level, "%s", message);
}

PGDLLEXPORT void pgx_elog_error(char *message);
void pgx_elog_error(char *message) {
    elog(ERROR, "%s", message);
}

PGDLLEXPORT void pgx_ereport(int level, int code, char *message, char *file, int lineno, int colno);
void pgx_ereport(int level, int code, char *message, char *file, int lineno, int colno) {
    ereport(level,
            (errcode(code),
                    errmsg("%s", message), errcontext_msg("%s:%d:%d", file, lineno, colno)));
}

PGDLLEXPORT void pgx_SET_VARSIZE(struct varlena *ptr, int size);
void pgx_SET_VARSIZE(struct varlena *ptr, int size) {
    SET_VARSIZE(ptr, size);
}

PGDLLEXPORT void pgx_SET_VARSIZE_SHORT(struct varlena *ptr, int size);
void pgx_SET_VARSIZE_SHORT(struct varlena *ptr, int size) {
    SET_VARSIZE_SHORT(ptr, size);
}

PGDLLEXPORT Datum pgx_heap_getattr(HeapTupleData *tuple, int attnum, TupleDesc tupdesc, bool *isnull);
Datum pgx_heap_getattr(HeapTupleData *tuple, int attnum, TupleDesc tupdesc, bool *isnull) {
    return heap_getattr(tuple, attnum, tupdesc, isnull);
}

PGDLLEXPORT TransactionId pgx_HeapTupleHeaderGetXmin(HeapTupleHeader htup_header);
TransactionId pgx_HeapTupleHeaderGetXmin(HeapTupleHeader htup_header) {
    return HeapTupleHeaderGetXmin(htup_header);
}

PGDLLEXPORT CommandId pgx_HeapTupleHeaderGetRawCommandId(HeapTupleHeader htup_header);
CommandId pgx_HeapTupleHeaderGetRawCommandId(HeapTupleHeader htup_header) {
    return HeapTupleHeaderGetRawCommandId(htup_header);
}

PGDLLEXPORT bool pgx_HeapTupleHeaderIsHeapOnly(HeapTupleHeader htup_header);
bool pgx_HeapTupleHeaderIsHeapOnly(HeapTupleHeader htup_header) {
    return HeapTupleHeaderIsHeapOnly(htup_header);
}

PGDLLEXPORT RangeTblEntry *pgx_planner_rt_fetch(Index index, PlannerInfo *plannerInfo);
RangeTblEntry *pgx_planner_rt_fetch(Index index, PlannerInfo *root) {
    return planner_rt_fetch(index, root);
}

#if IS_PG_10 || IS_PG_11
PGDLLEXPORT Oid pgx_HeapTupleHeaderGetOid(HeapTupleHeader htup_header);
Oid pgx_HeapTupleHeaderGetOid(HeapTupleHeader htup_header) {
    return HeapTupleHeaderGetOid(htup_header);
}
#endif

PGDLLEXPORT char *pgx_GETSTRUCT(HeapTuple tuple);
char *pgx_GETSTRUCT(HeapTuple tuple) {
    return GETSTRUCT(tuple);
}

PGDLLEXPORT void pgx_deconstruct_row_type(TupleDesc tupdesc, Datum row, Datum **columns, bool **nulls);
void pgx_deconstruct_row_type(TupleDesc tupdesc, Datum row, Datum **columns, bool **nulls) {
    HeapTupleHeader td;
    HeapTupleData   tmptup;
    HeapTupleData   *tuple;
    int             natts;

    td = DatumGetHeapTupleHeader(row);

    /* Build a temporary HeapTuple control structure */
    tmptup.t_len  = HeapTupleHeaderGetDatumLength(td);
    tmptup.t_data = td;
    tuple = &tmptup;
    natts = tupdesc->natts;

    Datum *cols = palloc(natts * sizeof(Datum));
    bool *ns = palloc(natts * sizeof(bool));

    for (int i=0; i<natts; i++) {
        Form_pg_attribute att = TupleDescAttr(tupdesc, i);

        if (att->attisdropped) {
            cols[i] = 0;
            ns[i] = true;
            continue;
        }

        cols[i] = heap_getattr(tuple, i + 1, tupdesc, &ns[i]);
    }

    *columns = cols;
    *nulls = ns;
}