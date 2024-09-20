import hljs, { LanguageFn } from "highlight.js";

export const rs4j: LanguageFn = () => {
    const KEYWORDS = [
        "fn",
        "mut",
        "optional",
        "class",
        "static",
        "consumed",
        "bound",
    ];

    const TYPES = [
        "String",
        "str",
        "bool",
        "i8",
        "i16",
        "i32",
        "i64",
        "i128",
        "u8",
        "u16",
        "u32",
        "u64",
        "u128",
        "f32",
        "f64",
    ];

    return {
        case_insensitive: true,

        keywords: {
            $pattern: hljs.IDENT_RE,

            keyword: KEYWORDS,
            type: TYPES,

            punctuation: [";", ".", ",", "&", "-", ">", "<"],
        },

        contains: [
            hljs.C_LINE_COMMENT_MODE,

            {
                scope: "string",
                begin: '"',
                end: '"',
            },

            {
                begin: [/fn/, /\s+/, hljs.UNDERSCORE_IDENT_RE],
                className: {
                    1: "keyword",
                    3: "title.function",
                } as any,
            },

            {
                begin: [/class/, /\s+/, hljs.UNDERSCORE_IDENT_RE],
                className: {
                    1: "keyword",
                    3: "title.class",
                } as any,
            },

            {
                begin: [/->/, /\s+/, hljs.UNDERSCORE_IDENT_RE],
                className: {
                    1: "code",
                    3: "type",
                } as any,
            },

            {
                begin: hljs.IDENT_RE + "::",
                keywords: {
                    keyword: "Self",
                    type: TYPES,
                },
            },

            {
                begin: [
                    hljs.UNDERSCORE_IDENT_RE,
                    /:/,
                    /\s+/,
                    /(#into)?(\s+)?(&(mut)?)?/,
                    /(\s+)?/,
                    hljs.UNDERSCORE_IDENT_RE,
                ],
                className: {
                    1: "variable",
                    2: "code",
                    4: "meta",
                    6: "type",
                } as any,
            },
        ],
    };
};

export default rs4j;
