const version = 'v0.9';
//

let _socket, _isConnected = false;

const _publishData = (data) => {
    if (!_isConnected) {
        return;
    }
    _socket.send(data);
}

const init = () => {
    try {
        _socket = new WebSocket(`ws://${location.hostname}:${location.port}/`, ['r2ce']);

        _socket.onopen = function (a) {
            console.log('ws connect');
            _isConnected = true;
            _publishData(`iam-web-client`);
        };

        _socket.onerror = function (err) {
            _isConnected = false;
            console.log('ws error ', err);
        };

        _socket.onmessage = function (e) {
            console.log('ws message: ', e.data);
        };

        _socket.onclose = function () {
            _isConnected = false;
            console.log('ws close');
        };
    } catch (err) {
        console.err(err);
    }

    console.log('init done');
};

window.onload = () => init();