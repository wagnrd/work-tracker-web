let initialPageLoad = true;
let ws;

function createWebSocket() {
    setTimeout(() => {
        try {
            connect();
        } catch (_) {
            createWebSocket();
        }
    }, 100)
}

function connect() {
    ws = new WebSocket('ws://localhost:8888/dev/autorefresh');

    ws.onopen = () => {
        if (initialPageLoad) {
            console.log('auto refresh initialized');
            return;
        }

        console.log('Server has rebuilt - reloading page');
        location.reload();
    }

    ws.onclose = () => {
        console.log('Server rebuilding...')
        initialPageLoad = false;
        createWebSocket();
    }
}

createWebSocket();