module.exports = {
    docs: {
        'Introduction': [
            'welcome',
            'whatis'
        ],
        'Basics': [
            'getting-started',
            'application',
            'server',
            'extractors',
            'handlers',
        ],
        'Advanced': [
            'errors',
            'url-dispatch',
            'request',
            'response',
            'testing',
            'middleware',
            'static-files',
        ],
        'Protocols': [
            'websockets',
            'http2',
        ],
        'Patterns': [
            'autoreload',
            'databases',
        ],
        'Diagrams': [
            'http_server_init',
            'conn_lifecycle',
        ],
        'API Documentation': [
            {
                type: 'link',
                label: 'actix',
                href: 'https://docs.rs/actix/latest/actix/'
            },
            {
                type: 'link',
                label: 'actix-web',
                href: 'https://docs.rs/actix-web/latest/actix_web/'
            }
        ],
    },
};
