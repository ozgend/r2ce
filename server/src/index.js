const express = require('express');
const app = express();
const port = 6022;

app.use(express.json());

app.use((req, res, next) => {
    console.log(`@(${new Date().toISOString()}) | [${req.ip}] ${req.method} ${req.url}`);
    next();
});

app.get('/', (req, res) => {
    res.send('r2ce-server');
});

app.post('/pulse', (req, res) => {
    console.log(`>> incoming pulse from ${req.body.COMPUTERNAME} @ ${req.body.HOMEPATH}`);
    res.sendStatus(204);
});

app.listen(port, () => {
    console.log(`r2ce-server @ http://localhost:${port}`)
});