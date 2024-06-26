#[allow(unused)]
#[repr(u16)]
pub enum HttpStatus {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    ImUsed = 226,
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    #[deprecated = "in-band configuration of proxy is of dubious security"]
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableContent = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

impl HttpStatus {
    pub fn into_status_line(self) -> &'static str {
        match self {
            Self::Continue => "HTTP/1.1 100 Continue\r\n",
            Self::SwitchingProtocols => "HTTP/1.1 101 Switching Protocols\r\n",
            Self::Processing => "HTTP/1.1 102 Processing\r\n",
            Self::EarlyHints => "HTTP/1.1 103 Early Hints\r\n",
            Self::Ok => "HTTP/1.1 200 OK\r\n",
            Self::Created => "HTTP/1.1 201 Created\r\n",
            Self::Accepted => "HTTP/1.1 202 Accepted\r\n",
            Self::NonAuthoritativeInformation => "HTTP/1.1 203 Non-Authoritative Information\r\n",
            Self::NoContent => "HTTP/1.1 204 No Content\r\n",
            Self::ResetContent => "HTTP/1.1 205 Reset Content\r\n",
            Self::PartialContent => "HTTP/1.1 206 Partial Content\r\n",
            Self::MultiStatus => "HTTP/1.1 207 Multi-Status\r\n",
            Self::AlreadyReported => "HTTP/1.1 208 Already Reported\r\n",
            Self::ImUsed => "HTTP/1.1 226 IM Used\r\n",
            Self::MultipleChoices => "HTTP/1.1 300 Multiple Choices\r\n",
            Self::MovedPermanently => "HTTP/1.1 301 Moved Permanently\r\n",
            Self::Found => "HTTP/1.1 302 Found\r\n",
            Self::SeeOther => "HTTP/1.1 303 See Other\r\n",
            Self::NotModified => "HTTP/1.1 304 Not Modified\r\n",
            #[allow(deprecated)]
            Self::UseProxy => "HTTP/1.1 305 Use Proxy\r\n",
            Self::TemporaryRedirect => "HTTP/1.1 307 Temporary Redirect\r\n",
            Self::PermanentRedirect => "HTTP/1.1 308 Permanent Redirect\r\n",
            Self::BadRequest => "HTTP/1.1 400 Bad Request\r\n",
            Self::Unauthorized => "HTTP/1.1 401 Unauthorized\r\n",
            Self::PaymentRequired => "HTTP/1.1 402 Payment Required\r\n",
            Self::Forbidden => "HTTP/1.1 403 Forbidden\r\n",
            Self::NotFound => "HTTP/1.1 404 Not Found\r\n",
            Self::MethodNotAllowed => "HTTP/1.1 405 Method Not Allowed\r\n",
            Self::NotAcceptable => "HTTP/1.1 406 Not Acceptable\r\n",
            Self::ProxyAuthenticationRequired => "HTTP/1.1 407 Proxy Authentication Required\r\n",
            Self::RequestTimeout => "HTTP/1.1 408 Request Timeout\r\n",
            Self::Conflict => "HTTP/1.1 409 Conflict\r\n",
            Self::Gone => "HTTP/1.1 410 Gone\r\n",
            Self::LengthRequired => "HTTP/1.1 411 Length Required\r\n",
            Self::PreconditionFailed => "HTTP/1.1 412 Precondition Failed\r\n",
            Self::PayloadTooLarge => "HTTP/1.1 413 Payload Too Large\r\n",
            Self::UriTooLong => "HTTP/1.1 414 URI Too Long\r\n",
            Self::UnsupportedMediaType => "HTTP/1.1 415 Unsupported Media Type\r\n",
            Self::RangeNotSatisfiable => "HTTP/1.1 416 Range Not Satisfiable\r\n",
            Self::ExpectationFailed => "HTTP/1.1 417 Expectation Failed\r\n",
            Self::ImATeapot => "HTTP/1.1 418 I'm A Teapot\r\n",
            Self::MisdirectedRequest => "HTTP/1.1 421 Misdirected Request\r\n",
            Self::UnprocessableContent => "HTTP/1.1 422 Unprocessable Content\r\n",
            Self::Locked => "HTTP/1.1 423 Locked\r\n",
            Self::FailedDependency => "HTTP/1.1 424 Failed Dependency\r\n",
            Self::TooEarly => "HTTP/1.1 425 Too Early\r\n",
            Self::UpgradeRequired => "HTTP/1.1 426 Upgrade Required\r\n",
            Self::PreconditionRequired => "HTTP/1.1 428 Precondition Required\r\n",
            Self::TooManyRequests => "HTTP/1.1 429 Too Many Requests\r\n",
            Self::RequestHeaderFieldsTooLarge => "HTTP/1.1 431 Request Header Fields Too Large\r\n",
            Self::UnavailableForLegalReasons => "HTTP/1.1 451 Unavailable For Legal Reasons\r\n",
            Self::InternalServerError => "HTTP/1.1 500 Internal Server Error\r\n",
            Self::NotImplemented => "HTTP/1.1 501 Not Implemented\r\n",
            Self::BadGateway => "HTTP/1.1 502 Bad Gateway\r\n",
            Self::ServiceUnavailable => "HTTP/1.1 503 Service Unavailable\r\n",
            Self::GatewayTimeout => "HTTP/1.1 504 Gateway Timeout\r\n",
            Self::HttpVersionNotSupported => "HTTP/1.1 505 HTTP Version Not Supported\r\n",
            Self::VariantAlsoNegotiates => "HTTP/1.1 506 Variant Also Negotiates\r\n",
            Self::InsufficientStorage => "HTTP/1.1 507 Insufficient Storage\r\n",
            Self::LoopDetected => "HTTP/1.1 508 Loop Detected\r\n",
            Self::NotExtended => "HTTP/1.1 510 Not Extended\r\n",
            Self::NetworkAuthenticationRequired => {
                "HTTP/1.1 511 Network Authentication Required\r\n"
            }
        }
    }
}
