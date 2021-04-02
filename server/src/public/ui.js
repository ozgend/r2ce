const _version = 'v0.9';
const _identifierName = 'web';
let _id = '';
//

const _actions = { listjoins: 'listjoins', join: 'join', forward: 'forward', respond: 'respond', leave: 'leave', id: 'id' };
const _cliItemTypes = ['input', 'output', 'error'];
const _cliHistory = {};
const _historySize = 100;
let _historyCursor = -1;

let $summaryTargetId, $summaryWebId, $summaryConnected;
let $clientList, $commandInput, $commandHistory;
let _socket, _isConnected = false;

const __addCliHistory = (id, item) => {
    if (!_cliHistory[id]) {
        _cliHistory[id] = [];
    }

    if (_cliHistory[id].length >= _historySize) {
        _cliHistory[id].splice(0, 1);
    }

    _cliHistory[id].push(item);

    localStorage.setItem(id, JSON.stringify(_cliHistory[id]));
};

const __renderCliHistory = (id) => {
    $commandHistory.replaceChildren();

    if (!_cliHistory[id]) {
        return;
    }

    let $child;

    _cliHistory[id].forEach(item => {
        _cliItemTypes.forEach(type => {
            if ((item[type] || '').length > 0) {
                $child = document.createElement('span');
                $child.innerText = `${item[type].trim()}`;
                $child.classList.add(type);
                $commandHistory.appendChild($child);
            }
        });
    });

    $commandHistory.scroll({ top: $commandHistory.scrollHeight, behavior: 'smooth' });
    $commandInput.focus();
};

const __loadCliHistory = (id) => {
    _cliHistory[id] = JSON.parse(localStorage.getItem(id));
};

const updateCli = (id, item) => {
    __addCliHistory(id, item);
    __renderCliHistory(id);
};

const publishMessage = (action, target, data) => {
    if (!_isConnected) {
        return;
    }
    _socket.send(`${action}<<<${target}<<<${data}`);
};

const sendCommand = (e, text) => {
    if (e.keyCode === 38 || e.keyCode === 40) {
        let inputHistory = _cliHistory[$clientList.value].filter(cli => cli.input).reverse()

        if (e.keyCode === 38 && _historyCursor < inputHistory.length - 1) {
            _historyCursor++;
        }
        if (e.keyCode === 40 && _historyCursor >= 0) {
            _historyCursor--;
        }

        $commandInput.value = _historyCursor === -1 ? '' : inputHistory[_historyCursor].input.split('] $ ')[1];
        return;
    }

    if ((e.ctrlKey || e.metaKey) && (e.keyCode === 13 || e.keyCode === 10) && text.trim().length > 0 && $clientList.value.length > 0) {
        publishMessage(_actions.forward, $clientList.value, text.trim());
        updateCli($clientList.value, { input: `[${$clientList.value}] $ ${text}` });
        $commandInput.value = '';
        _historyCursor = 0;
    }
};

const addClientList = (client) => {
    let option = document.createElement('option');
    option.value = client.id;
    option.innerHTML = `${client.id} :: ${client.name}`;
    option.disabled = client.name === _identifierName;
    $clientList.appendChild(option);
    $summaryConnected.innerText = $clientList.childElementCount - 1;
};

const removeClientList = (id) => {
    for (let i = 0; i < $clientList.length; i++) {
        if ($clientList.options[i].value == id)
            $clientList.remove(i);
    }
};

const handleSocketMessage = (data) => {
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
            updateCli(target, JSON.parse(payload));
            break;

        case _actions.id:
            _id = payload;
            $summaryWebId.innerText = payload;
            break;

        default:
            console.log(`invalid message: ${data}`);
            break;
    }
};

const setTarget = (id) => {
    __loadCliHistory(id);
    __renderCliHistory(id);
    $summaryTargetId.innerText = id;
}

const init = () => {
    $clientList = document.querySelector('select.client-list');
    $summaryConnected = document.querySelector('div.client-summary > span.connected');
    $summaryTargetId = document.querySelector('div.client-summary > span.target-id');
    $summaryWebId = document.querySelector('div.client-summary > span.web-id');
    $commandInput = document.querySelector('input.command-input');
    $commandHistory = document.querySelector('div.command-history');

    try {
        _socket = new WebSocket(`ws://${location.hostname}:${location.port}/`, ['r2ce']);

        _socket.onopen = function (a) {
            console.log('ws connect');
            _isConnected = true;
            publishMessage(_actions.join, 'server', _identifierName);
        };

        _socket.onerror = function (err) {
            _isConnected = false;
            console.log('ws error ', err);
        };

        _socket.onmessage = function (e) {
            console.log('ws message: ', e.data);
            handleSocketMessage(e.data);
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