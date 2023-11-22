import http from 'k6/http';

export default function () {
    const url = 'http://localhost:8080/';

    const params = {
        headers: {
            'Content-Type': 'application/text',
        },
    };

    http.get(url)
}