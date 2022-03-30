#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub enum PgBuiltInOids {
    BOOLOID = crate::BOOLOID as isize,
    BYTEAOID = crate::BYTEAOID as isize,
    CHAROID = crate::CHAROID as isize,
    NAMEOID = crate::NAMEOID as isize,
    INT8OID = crate::INT8OID as isize,
    INT2OID = crate::INT2OID as isize,
    INT2VECTOROID = crate::INT2VECTOROID as isize,
    INT4OID = crate::INT4OID as isize,
    REGPROCOID = crate::REGPROCOID as isize,
    TEXTOID = crate::TEXTOID as isize,
    OIDOID = crate::OIDOID as isize,
    TIDOID = crate::TIDOID as isize,
    XIDOID = crate::XIDOID as isize,
    CIDOID = crate::CIDOID as isize,
    OIDVECTOROID = crate::OIDVECTOROID as isize,
    JSONOID = crate::JSONOID as isize,
    XMLOID = crate::XMLOID as isize,
    PGNODETREEOID = crate::PGNODETREEOID as isize,
    PGNDISTINCTOID = crate::PGNDISTINCTOID as isize,
    PGDEPENDENCIESOID = crate::PGDEPENDENCIESOID as isize,
    PGMCVLISTOID = crate::PGMCVLISTOID as isize,
    PGDDLCOMMANDOID = crate::PGDDLCOMMANDOID as isize,
    XID8OID = crate::XID8OID as isize,
    POINTOID = crate::POINTOID as isize,
    LSEGOID = crate::LSEGOID as isize,
    PATHOID = crate::PATHOID as isize,
    BOXOID = crate::BOXOID as isize,
    POLYGONOID = crate::POLYGONOID as isize,
    LINEOID = crate::LINEOID as isize,
    FLOAT4OID = crate::FLOAT4OID as isize,
    FLOAT8OID = crate::FLOAT8OID as isize,
    UNKNOWNOID = crate::UNKNOWNOID as isize,
    CIRCLEOID = crate::CIRCLEOID as isize,
    CASHOID = crate::CASHOID as isize,
    MACADDROID = crate::MACADDROID as isize,
    INETOID = crate::INETOID as isize,
    CIDROID = crate::CIDROID as isize,
    MACADDR8OID = crate::MACADDR8OID as isize,
    ACLITEMOID = crate::ACLITEMOID as isize,
    BPCHAROID = crate::BPCHAROID as isize,
    VARCHAROID = crate::VARCHAROID as isize,
    DATEOID = crate::DATEOID as isize,
    TIMEOID = crate::TIMEOID as isize,
    TIMESTAMPOID = crate::TIMESTAMPOID as isize,
    TIMESTAMPTZOID = crate::TIMESTAMPTZOID as isize,
    INTERVALOID = crate::INTERVALOID as isize,
    TIMETZOID = crate::TIMETZOID as isize,
    BITOID = crate::BITOID as isize,
    VARBITOID = crate::VARBITOID as isize,
    NUMERICOID = crate::NUMERICOID as isize,
    REFCURSOROID = crate::REFCURSOROID as isize,
    REGPROCEDUREOID = crate::REGPROCEDUREOID as isize,
    REGOPEROID = crate::REGOPEROID as isize,
    REGOPERATOROID = crate::REGOPERATOROID as isize,
    REGCLASSOID = crate::REGCLASSOID as isize,
    REGCOLLATIONOID = crate::REGCOLLATIONOID as isize,
    REGTYPEOID = crate::REGTYPEOID as isize,
    REGROLEOID = crate::REGROLEOID as isize,
    REGNAMESPACEOID = crate::REGNAMESPACEOID as isize,
    UUIDOID = crate::UUIDOID as isize,
    LSNOID = crate::LSNOID as isize,
    TSVECTOROID = crate::TSVECTOROID as isize,
    GTSVECTOROID = crate::GTSVECTOROID as isize,
    TSQUERYOID = crate::TSQUERYOID as isize,
    REGCONFIGOID = crate::REGCONFIGOID as isize,
    REGDICTIONARYOID = crate::REGDICTIONARYOID as isize,
    JSONBOID = crate::JSONBOID as isize,
    JSONPATHOID = crate::JSONPATHOID as isize,
    TXID_SNAPSHOTOID = crate::TXID_SNAPSHOTOID as isize,
    PG_SNAPSHOTOID = crate::PG_SNAPSHOTOID as isize,
    INT4RANGEOID = crate::INT4RANGEOID as isize,
    NUMRANGEOID = crate::NUMRANGEOID as isize,
    TSRANGEOID = crate::TSRANGEOID as isize,
    TSTZRANGEOID = crate::TSTZRANGEOID as isize,
    DATERANGEOID = crate::DATERANGEOID as isize,
    INT8RANGEOID = crate::INT8RANGEOID as isize,
    RECORDOID = crate::RECORDOID as isize,
    RECORDARRAYOID = crate::RECORDARRAYOID as isize,
    CSTRINGOID = crate::CSTRINGOID as isize,
    ANYOID = crate::ANYOID as isize,
    ANYARRAYOID = crate::ANYARRAYOID as isize,
    VOIDOID = crate::VOIDOID as isize,
    TRIGGEROID = crate::TRIGGEROID as isize,
    EVTTRIGGEROID = crate::EVTTRIGGEROID as isize,
    LANGUAGE_HANDLEROID = crate::LANGUAGE_HANDLEROID as isize,
    INTERNALOID = crate::INTERNALOID as isize,
    ANYELEMENTOID = crate::ANYELEMENTOID as isize,
    ANYNONARRAYOID = crate::ANYNONARRAYOID as isize,
    ANYENUMOID = crate::ANYENUMOID as isize,
    FDW_HANDLEROID = crate::FDW_HANDLEROID as isize,
    INDEX_AM_HANDLEROID = crate::INDEX_AM_HANDLEROID as isize,
    TSM_HANDLEROID = crate::TSM_HANDLEROID as isize,
    TABLE_AM_HANDLEROID = crate::TABLE_AM_HANDLEROID as isize,
    ANYRANGEOID = crate::ANYRANGEOID as isize,
    ANYCOMPATIBLEOID = crate::ANYCOMPATIBLEOID as isize,
    ANYCOMPATIBLEARRAYOID = crate::ANYCOMPATIBLEARRAYOID as isize,
    ANYCOMPATIBLENONARRAYOID = crate::ANYCOMPATIBLENONARRAYOID as isize,
    ANYCOMPATIBLERANGEOID = crate::ANYCOMPATIBLERANGEOID as isize,
    BOOLARRAYOID = crate::BOOLARRAYOID as isize,
    BYTEAARRAYOID = crate::BYTEAARRAYOID as isize,
    CHARARRAYOID = crate::CHARARRAYOID as isize,
    NAMEARRAYOID = crate::NAMEARRAYOID as isize,
    INT8ARRAYOID = crate::INT8ARRAYOID as isize,
    INT2ARRAYOID = crate::INT2ARRAYOID as isize,
    INT2VECTORARRAYOID = crate::INT2VECTORARRAYOID as isize,
    INT4ARRAYOID = crate::INT4ARRAYOID as isize,
    REGPROCARRAYOID = crate::REGPROCARRAYOID as isize,
    TEXTARRAYOID = crate::TEXTARRAYOID as isize,
    OIDARRAYOID = crate::OIDARRAYOID as isize,
    TIDARRAYOID = crate::TIDARRAYOID as isize,
    XIDARRAYOID = crate::XIDARRAYOID as isize,
    CIDARRAYOID = crate::CIDARRAYOID as isize,
    OIDVECTORARRAYOID = crate::OIDVECTORARRAYOID as isize,
    JSONARRAYOID = crate::JSONARRAYOID as isize,
    XMLARRAYOID = crate::XMLARRAYOID as isize,
    XID8ARRAYOID = crate::XID8ARRAYOID as isize,
    POINTARRAYOID = crate::POINTARRAYOID as isize,
    LSEGARRAYOID = crate::LSEGARRAYOID as isize,
    PATHARRAYOID = crate::PATHARRAYOID as isize,
    BOXARRAYOID = crate::BOXARRAYOID as isize,
    POLYGONARRAYOID = crate::POLYGONARRAYOID as isize,
    LINEARRAYOID = crate::LINEARRAYOID as isize,
    FLOAT4ARRAYOID = crate::FLOAT4ARRAYOID as isize,
    FLOAT8ARRAYOID = crate::FLOAT8ARRAYOID as isize,
    CIRCLEARRAYOID = crate::CIRCLEARRAYOID as isize,
    MONEYARRAYOID = crate::MONEYARRAYOID as isize,
    MACADDRARRAYOID = crate::MACADDRARRAYOID as isize,
    INETARRAYOID = crate::INETARRAYOID as isize,
    CIDRARRAYOID = crate::CIDRARRAYOID as isize,
    MACADDR8ARRAYOID = crate::MACADDR8ARRAYOID as isize,
    ACLITEMARRAYOID = crate::ACLITEMARRAYOID as isize,
    BPCHARARRAYOID = crate::BPCHARARRAYOID as isize,
    VARCHARARRAYOID = crate::VARCHARARRAYOID as isize,
    DATEARRAYOID = crate::DATEARRAYOID as isize,
    TIMEARRAYOID = crate::TIMEARRAYOID as isize,
    TIMESTAMPARRAYOID = crate::TIMESTAMPARRAYOID as isize,
    TIMESTAMPTZARRAYOID = crate::TIMESTAMPTZARRAYOID as isize,
    INTERVALARRAYOID = crate::INTERVALARRAYOID as isize,
    TIMETZARRAYOID = crate::TIMETZARRAYOID as isize,
    BITARRAYOID = crate::BITARRAYOID as isize,
    VARBITARRAYOID = crate::VARBITARRAYOID as isize,
    NUMERICARRAYOID = crate::NUMERICARRAYOID as isize,
    REFCURSORARRAYOID = crate::REFCURSORARRAYOID as isize,
    REGPROCEDUREARRAYOID = crate::REGPROCEDUREARRAYOID as isize,
    REGOPERARRAYOID = crate::REGOPERARRAYOID as isize,
    REGOPERATORARRAYOID = crate::REGOPERATORARRAYOID as isize,
    REGCLASSARRAYOID = crate::REGCLASSARRAYOID as isize,
    REGCOLLATIONARRAYOID = crate::REGCOLLATIONARRAYOID as isize,
    REGTYPEARRAYOID = crate::REGTYPEARRAYOID as isize,
    REGROLEARRAYOID = crate::REGROLEARRAYOID as isize,
    REGNAMESPACEARRAYOID = crate::REGNAMESPACEARRAYOID as isize,
    UUIDARRAYOID = crate::UUIDARRAYOID as isize,
    PG_LSNARRAYOID = crate::PG_LSNARRAYOID as isize,
    TSVECTORARRAYOID = crate::TSVECTORARRAYOID as isize,
    GTSVECTORARRAYOID = crate::GTSVECTORARRAYOID as isize,
    TSQUERYARRAYOID = crate::TSQUERYARRAYOID as isize,
    REGCONFIGARRAYOID = crate::REGCONFIGARRAYOID as isize,
    REGDICTIONARYARRAYOID = crate::REGDICTIONARYARRAYOID as isize,
    JSONBARRAYOID = crate::JSONBARRAYOID as isize,
    JSONPATHARRAYOID = crate::JSONPATHARRAYOID as isize,
    TXID_SNAPSHOTARRAYOID = crate::TXID_SNAPSHOTARRAYOID as isize,
    PG_SNAPSHOTARRAYOID = crate::PG_SNAPSHOTARRAYOID as isize,
    INT4RANGEARRAYOID = crate::INT4RANGEARRAYOID as isize,
    NUMRANGEARRAYOID = crate::NUMRANGEARRAYOID as isize,
    TSRANGEARRAYOID = crate::TSRANGEARRAYOID as isize,
    TSTZRANGEARRAYOID = crate::TSTZRANGEARRAYOID as isize,
    DATERANGEARRAYOID = crate::DATERANGEARRAYOID as isize,
    INT8RANGEARRAYOID = crate::INT8RANGEARRAYOID as isize,
    CSTRINGARRAYOID = crate::CSTRINGARRAYOID as isize,
    HEAP_TABLE_AM_HANDLER_OID = crate::HEAP_TABLE_AM_HANDLER_OID as isize,
}
impl PgBuiltInOids {
    pub fn from(oid: crate::Oid) -> Option<PgBuiltInOids> {
        match oid {
            crate::BOOLOID => Some(crate::PgBuiltInOids::BOOLOID),
            crate::BYTEAOID => Some(crate::PgBuiltInOids::BYTEAOID),
            crate::CHAROID => Some(crate::PgBuiltInOids::CHAROID),
            crate::NAMEOID => Some(crate::PgBuiltInOids::NAMEOID),
            crate::INT8OID => Some(crate::PgBuiltInOids::INT8OID),
            crate::INT2OID => Some(crate::PgBuiltInOids::INT2OID),
            crate::INT2VECTOROID => Some(crate::PgBuiltInOids::INT2VECTOROID),
            crate::INT4OID => Some(crate::PgBuiltInOids::INT4OID),
            crate::REGPROCOID => Some(crate::PgBuiltInOids::REGPROCOID),
            crate::TEXTOID => Some(crate::PgBuiltInOids::TEXTOID),
            crate::OIDOID => Some(crate::PgBuiltInOids::OIDOID),
            crate::TIDOID => Some(crate::PgBuiltInOids::TIDOID),
            crate::XIDOID => Some(crate::PgBuiltInOids::XIDOID),
            crate::CIDOID => Some(crate::PgBuiltInOids::CIDOID),
            crate::OIDVECTOROID => Some(crate::PgBuiltInOids::OIDVECTOROID),
            crate::JSONOID => Some(crate::PgBuiltInOids::JSONOID),
            crate::XMLOID => Some(crate::PgBuiltInOids::XMLOID),
            crate::PGNODETREEOID => Some(crate::PgBuiltInOids::PGNODETREEOID),
            crate::PGNDISTINCTOID => Some(crate::PgBuiltInOids::PGNDISTINCTOID),
            crate::PGDEPENDENCIESOID => Some(crate::PgBuiltInOids::PGDEPENDENCIESOID),
            crate::PGMCVLISTOID => Some(crate::PgBuiltInOids::PGMCVLISTOID),
            crate::PGDDLCOMMANDOID => Some(crate::PgBuiltInOids::PGDDLCOMMANDOID),
            crate::XID8OID => Some(crate::PgBuiltInOids::XID8OID),
            crate::POINTOID => Some(crate::PgBuiltInOids::POINTOID),
            crate::LSEGOID => Some(crate::PgBuiltInOids::LSEGOID),
            crate::PATHOID => Some(crate::PgBuiltInOids::PATHOID),
            crate::BOXOID => Some(crate::PgBuiltInOids::BOXOID),
            crate::POLYGONOID => Some(crate::PgBuiltInOids::POLYGONOID),
            crate::LINEOID => Some(crate::PgBuiltInOids::LINEOID),
            crate::FLOAT4OID => Some(crate::PgBuiltInOids::FLOAT4OID),
            crate::FLOAT8OID => Some(crate::PgBuiltInOids::FLOAT8OID),
            crate::UNKNOWNOID => Some(crate::PgBuiltInOids::UNKNOWNOID),
            crate::CIRCLEOID => Some(crate::PgBuiltInOids::CIRCLEOID),
            crate::CASHOID => Some(crate::PgBuiltInOids::CASHOID),
            crate::MACADDROID => Some(crate::PgBuiltInOids::MACADDROID),
            crate::INETOID => Some(crate::PgBuiltInOids::INETOID),
            crate::CIDROID => Some(crate::PgBuiltInOids::CIDROID),
            crate::MACADDR8OID => Some(crate::PgBuiltInOids::MACADDR8OID),
            crate::ACLITEMOID => Some(crate::PgBuiltInOids::ACLITEMOID),
            crate::BPCHAROID => Some(crate::PgBuiltInOids::BPCHAROID),
            crate::VARCHAROID => Some(crate::PgBuiltInOids::VARCHAROID),
            crate::DATEOID => Some(crate::PgBuiltInOids::DATEOID),
            crate::TIMEOID => Some(crate::PgBuiltInOids::TIMEOID),
            crate::TIMESTAMPOID => Some(crate::PgBuiltInOids::TIMESTAMPOID),
            crate::TIMESTAMPTZOID => Some(crate::PgBuiltInOids::TIMESTAMPTZOID),
            crate::INTERVALOID => Some(crate::PgBuiltInOids::INTERVALOID),
            crate::TIMETZOID => Some(crate::PgBuiltInOids::TIMETZOID),
            crate::BITOID => Some(crate::PgBuiltInOids::BITOID),
            crate::VARBITOID => Some(crate::PgBuiltInOids::VARBITOID),
            crate::NUMERICOID => Some(crate::PgBuiltInOids::NUMERICOID),
            crate::REFCURSOROID => Some(crate::PgBuiltInOids::REFCURSOROID),
            crate::REGPROCEDUREOID => Some(crate::PgBuiltInOids::REGPROCEDUREOID),
            crate::REGOPEROID => Some(crate::PgBuiltInOids::REGOPEROID),
            crate::REGOPERATOROID => Some(crate::PgBuiltInOids::REGOPERATOROID),
            crate::REGCLASSOID => Some(crate::PgBuiltInOids::REGCLASSOID),
            crate::REGCOLLATIONOID => Some(crate::PgBuiltInOids::REGCOLLATIONOID),
            crate::REGTYPEOID => Some(crate::PgBuiltInOids::REGTYPEOID),
            crate::REGROLEOID => Some(crate::PgBuiltInOids::REGROLEOID),
            crate::REGNAMESPACEOID => Some(crate::PgBuiltInOids::REGNAMESPACEOID),
            crate::UUIDOID => Some(crate::PgBuiltInOids::UUIDOID),
            crate::LSNOID => Some(crate::PgBuiltInOids::LSNOID),
            crate::TSVECTOROID => Some(crate::PgBuiltInOids::TSVECTOROID),
            crate::GTSVECTOROID => Some(crate::PgBuiltInOids::GTSVECTOROID),
            crate::TSQUERYOID => Some(crate::PgBuiltInOids::TSQUERYOID),
            crate::REGCONFIGOID => Some(crate::PgBuiltInOids::REGCONFIGOID),
            crate::REGDICTIONARYOID => Some(crate::PgBuiltInOids::REGDICTIONARYOID),
            crate::JSONBOID => Some(crate::PgBuiltInOids::JSONBOID),
            crate::JSONPATHOID => Some(crate::PgBuiltInOids::JSONPATHOID),
            crate::TXID_SNAPSHOTOID => Some(crate::PgBuiltInOids::TXID_SNAPSHOTOID),
            crate::PG_SNAPSHOTOID => Some(crate::PgBuiltInOids::PG_SNAPSHOTOID),
            crate::INT4RANGEOID => Some(crate::PgBuiltInOids::INT4RANGEOID),
            crate::NUMRANGEOID => Some(crate::PgBuiltInOids::NUMRANGEOID),
            crate::TSRANGEOID => Some(crate::PgBuiltInOids::TSRANGEOID),
            crate::TSTZRANGEOID => Some(crate::PgBuiltInOids::TSTZRANGEOID),
            crate::DATERANGEOID => Some(crate::PgBuiltInOids::DATERANGEOID),
            crate::INT8RANGEOID => Some(crate::PgBuiltInOids::INT8RANGEOID),
            crate::RECORDOID => Some(crate::PgBuiltInOids::RECORDOID),
            crate::RECORDARRAYOID => Some(crate::PgBuiltInOids::RECORDARRAYOID),
            crate::CSTRINGOID => Some(crate::PgBuiltInOids::CSTRINGOID),
            crate::ANYOID => Some(crate::PgBuiltInOids::ANYOID),
            crate::ANYARRAYOID => Some(crate::PgBuiltInOids::ANYARRAYOID),
            crate::VOIDOID => Some(crate::PgBuiltInOids::VOIDOID),
            crate::TRIGGEROID => Some(crate::PgBuiltInOids::TRIGGEROID),
            crate::EVTTRIGGEROID => Some(crate::PgBuiltInOids::EVTTRIGGEROID),
            crate::LANGUAGE_HANDLEROID => Some(crate::PgBuiltInOids::LANGUAGE_HANDLEROID),
            crate::INTERNALOID => Some(crate::PgBuiltInOids::INTERNALOID),
            crate::ANYELEMENTOID => Some(crate::PgBuiltInOids::ANYELEMENTOID),
            crate::ANYNONARRAYOID => Some(crate::PgBuiltInOids::ANYNONARRAYOID),
            crate::ANYENUMOID => Some(crate::PgBuiltInOids::ANYENUMOID),
            crate::FDW_HANDLEROID => Some(crate::PgBuiltInOids::FDW_HANDLEROID),
            crate::INDEX_AM_HANDLEROID => Some(crate::PgBuiltInOids::INDEX_AM_HANDLEROID),
            crate::TSM_HANDLEROID => Some(crate::PgBuiltInOids::TSM_HANDLEROID),
            crate::TABLE_AM_HANDLEROID => Some(crate::PgBuiltInOids::TABLE_AM_HANDLEROID),
            crate::ANYRANGEOID => Some(crate::PgBuiltInOids::ANYRANGEOID),
            crate::ANYCOMPATIBLEOID => Some(crate::PgBuiltInOids::ANYCOMPATIBLEOID),
            crate::ANYCOMPATIBLEARRAYOID => Some(crate::PgBuiltInOids::ANYCOMPATIBLEARRAYOID),
            crate::ANYCOMPATIBLENONARRAYOID => Some(crate::PgBuiltInOids::ANYCOMPATIBLENONARRAYOID),
            crate::ANYCOMPATIBLERANGEOID => Some(crate::PgBuiltInOids::ANYCOMPATIBLERANGEOID),
            crate::BOOLARRAYOID => Some(crate::PgBuiltInOids::BOOLARRAYOID),
            crate::BYTEAARRAYOID => Some(crate::PgBuiltInOids::BYTEAARRAYOID),
            crate::CHARARRAYOID => Some(crate::PgBuiltInOids::CHARARRAYOID),
            crate::NAMEARRAYOID => Some(crate::PgBuiltInOids::NAMEARRAYOID),
            crate::INT8ARRAYOID => Some(crate::PgBuiltInOids::INT8ARRAYOID),
            crate::INT2ARRAYOID => Some(crate::PgBuiltInOids::INT2ARRAYOID),
            crate::INT2VECTORARRAYOID => Some(crate::PgBuiltInOids::INT2VECTORARRAYOID),
            crate::INT4ARRAYOID => Some(crate::PgBuiltInOids::INT4ARRAYOID),
            crate::REGPROCARRAYOID => Some(crate::PgBuiltInOids::REGPROCARRAYOID),
            crate::TEXTARRAYOID => Some(crate::PgBuiltInOids::TEXTARRAYOID),
            crate::OIDARRAYOID => Some(crate::PgBuiltInOids::OIDARRAYOID),
            crate::TIDARRAYOID => Some(crate::PgBuiltInOids::TIDARRAYOID),
            crate::XIDARRAYOID => Some(crate::PgBuiltInOids::XIDARRAYOID),
            crate::CIDARRAYOID => Some(crate::PgBuiltInOids::CIDARRAYOID),
            crate::OIDVECTORARRAYOID => Some(crate::PgBuiltInOids::OIDVECTORARRAYOID),
            crate::JSONARRAYOID => Some(crate::PgBuiltInOids::JSONARRAYOID),
            crate::XMLARRAYOID => Some(crate::PgBuiltInOids::XMLARRAYOID),
            crate::XID8ARRAYOID => Some(crate::PgBuiltInOids::XID8ARRAYOID),
            crate::POINTARRAYOID => Some(crate::PgBuiltInOids::POINTARRAYOID),
            crate::LSEGARRAYOID => Some(crate::PgBuiltInOids::LSEGARRAYOID),
            crate::PATHARRAYOID => Some(crate::PgBuiltInOids::PATHARRAYOID),
            crate::BOXARRAYOID => Some(crate::PgBuiltInOids::BOXARRAYOID),
            crate::POLYGONARRAYOID => Some(crate::PgBuiltInOids::POLYGONARRAYOID),
            crate::LINEARRAYOID => Some(crate::PgBuiltInOids::LINEARRAYOID),
            crate::FLOAT4ARRAYOID => Some(crate::PgBuiltInOids::FLOAT4ARRAYOID),
            crate::FLOAT8ARRAYOID => Some(crate::PgBuiltInOids::FLOAT8ARRAYOID),
            crate::CIRCLEARRAYOID => Some(crate::PgBuiltInOids::CIRCLEARRAYOID),
            crate::MONEYARRAYOID => Some(crate::PgBuiltInOids::MONEYARRAYOID),
            crate::MACADDRARRAYOID => Some(crate::PgBuiltInOids::MACADDRARRAYOID),
            crate::INETARRAYOID => Some(crate::PgBuiltInOids::INETARRAYOID),
            crate::CIDRARRAYOID => Some(crate::PgBuiltInOids::CIDRARRAYOID),
            crate::MACADDR8ARRAYOID => Some(crate::PgBuiltInOids::MACADDR8ARRAYOID),
            crate::ACLITEMARRAYOID => Some(crate::PgBuiltInOids::ACLITEMARRAYOID),
            crate::BPCHARARRAYOID => Some(crate::PgBuiltInOids::BPCHARARRAYOID),
            crate::VARCHARARRAYOID => Some(crate::PgBuiltInOids::VARCHARARRAYOID),
            crate::DATEARRAYOID => Some(crate::PgBuiltInOids::DATEARRAYOID),
            crate::TIMEARRAYOID => Some(crate::PgBuiltInOids::TIMEARRAYOID),
            crate::TIMESTAMPARRAYOID => Some(crate::PgBuiltInOids::TIMESTAMPARRAYOID),
            crate::TIMESTAMPTZARRAYOID => Some(crate::PgBuiltInOids::TIMESTAMPTZARRAYOID),
            crate::INTERVALARRAYOID => Some(crate::PgBuiltInOids::INTERVALARRAYOID),
            crate::TIMETZARRAYOID => Some(crate::PgBuiltInOids::TIMETZARRAYOID),
            crate::BITARRAYOID => Some(crate::PgBuiltInOids::BITARRAYOID),
            crate::VARBITARRAYOID => Some(crate::PgBuiltInOids::VARBITARRAYOID),
            crate::NUMERICARRAYOID => Some(crate::PgBuiltInOids::NUMERICARRAYOID),
            crate::REFCURSORARRAYOID => Some(crate::PgBuiltInOids::REFCURSORARRAYOID),
            crate::REGPROCEDUREARRAYOID => Some(crate::PgBuiltInOids::REGPROCEDUREARRAYOID),
            crate::REGOPERARRAYOID => Some(crate::PgBuiltInOids::REGOPERARRAYOID),
            crate::REGOPERATORARRAYOID => Some(crate::PgBuiltInOids::REGOPERATORARRAYOID),
            crate::REGCLASSARRAYOID => Some(crate::PgBuiltInOids::REGCLASSARRAYOID),
            crate::REGCOLLATIONARRAYOID => Some(crate::PgBuiltInOids::REGCOLLATIONARRAYOID),
            crate::REGTYPEARRAYOID => Some(crate::PgBuiltInOids::REGTYPEARRAYOID),
            crate::REGROLEARRAYOID => Some(crate::PgBuiltInOids::REGROLEARRAYOID),
            crate::REGNAMESPACEARRAYOID => Some(crate::PgBuiltInOids::REGNAMESPACEARRAYOID),
            crate::UUIDARRAYOID => Some(crate::PgBuiltInOids::UUIDARRAYOID),
            crate::PG_LSNARRAYOID => Some(crate::PgBuiltInOids::PG_LSNARRAYOID),
            crate::TSVECTORARRAYOID => Some(crate::PgBuiltInOids::TSVECTORARRAYOID),
            crate::GTSVECTORARRAYOID => Some(crate::PgBuiltInOids::GTSVECTORARRAYOID),
            crate::TSQUERYARRAYOID => Some(crate::PgBuiltInOids::TSQUERYARRAYOID),
            crate::REGCONFIGARRAYOID => Some(crate::PgBuiltInOids::REGCONFIGARRAYOID),
            crate::REGDICTIONARYARRAYOID => Some(crate::PgBuiltInOids::REGDICTIONARYARRAYOID),
            crate::JSONBARRAYOID => Some(crate::PgBuiltInOids::JSONBARRAYOID),
            crate::JSONPATHARRAYOID => Some(crate::PgBuiltInOids::JSONPATHARRAYOID),
            crate::TXID_SNAPSHOTARRAYOID => Some(crate::PgBuiltInOids::TXID_SNAPSHOTARRAYOID),
            crate::PG_SNAPSHOTARRAYOID => Some(crate::PgBuiltInOids::PG_SNAPSHOTARRAYOID),
            crate::INT4RANGEARRAYOID => Some(crate::PgBuiltInOids::INT4RANGEARRAYOID),
            crate::NUMRANGEARRAYOID => Some(crate::PgBuiltInOids::NUMRANGEARRAYOID),
            crate::TSRANGEARRAYOID => Some(crate::PgBuiltInOids::TSRANGEARRAYOID),
            crate::TSTZRANGEARRAYOID => Some(crate::PgBuiltInOids::TSTZRANGEARRAYOID),
            crate::DATERANGEARRAYOID => Some(crate::PgBuiltInOids::DATERANGEARRAYOID),
            crate::INT8RANGEARRAYOID => Some(crate::PgBuiltInOids::INT8RANGEARRAYOID),
            crate::CSTRINGARRAYOID => Some(crate::PgBuiltInOids::CSTRINGARRAYOID),
            crate::HEAP_TABLE_AM_HANDLER_OID => {
                Some(crate::PgBuiltInOids::HEAP_TABLE_AM_HANDLER_OID)
            }
            _ => None,
        }
    }
}
