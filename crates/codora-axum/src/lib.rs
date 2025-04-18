macro_rules! create_status_code {
    ($($ident:ident),*) => {
        $(
            pub const $ident: axum::http::StatusCode = axum::http::StatusCode::$ident;
        )*
    };
}

create_status_code!(CONTINUE, SWITCHING_PROTOCOLS, PROCESSING);
create_status_code!(
    OK, CREATED, ACCEPTED, NON_AUTHORITATIVE_INFORMATION, NO_CONTENT, RESET_CONTENT, PARTIAL_CONTENT, MULTI_STATUS,
    ALREADY_REPORTED, IM_USED
);
create_status_code!(
    MULTIPLE_CHOICES, MOVED_PERMANENTLY, FOUND, SEE_OTHER, NOT_MODIFIED, USE_PROXY, TEMPORARY_REDIRECT,
    PERMANENT_REDIRECT
);
create_status_code!(
    BAD_REQUEST, UNAUTHORIZED, PAYMENT_REQUIRED, FORBIDDEN, NOT_FOUND, METHOD_NOT_ALLOWED, NOT_ACCEPTABLE,
    PROXY_AUTHENTICATION_REQUIRED, REQUEST_TIMEOUT, CONFLICT, GONE, LENGTH_REQUIRED, PRECONDITION_FAILED,
    PAYLOAD_TOO_LARGE, URI_TOO_LONG, UNSUPPORTED_MEDIA_TYPE, RANGE_NOT_SATISFIABLE, EXPECTATION_FAILED, IM_A_TEAPOT,
    MISDIRECTED_REQUEST, UNPROCESSABLE_ENTITY, LOCKED, FAILED_DEPENDENCY, UPGRADE_REQUIRED, PRECONDITION_REQUIRED,
    TOO_MANY_REQUESTS, REQUEST_HEADER_FIELDS_TOO_LARGE, UNAVAILABLE_FOR_LEGAL_REASONS
);
create_status_code!(
    INTERNAL_SERVER_ERROR, NOT_IMPLEMENTED, BAD_GATEWAY, SERVICE_UNAVAILABLE, GATEWAY_TIMEOUT,
    HTTP_VERSION_NOT_SUPPORTED, VARIANT_ALSO_NEGOTIATES, INSUFFICIENT_STORAGE, LOOP_DETECTED, NOT_EXTENDED,
    NETWORK_AUTHENTICATION_REQUIRED
);
