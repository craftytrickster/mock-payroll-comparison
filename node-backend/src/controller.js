'use strict';

const express = require('express');
const bodyParser = require('body-parser');
const cors = require('cors');
const processPayment = require('./submission').processPayment;

const app = express();

app.use(bodyParser.json());
app.use(cors());

app.get('/', indexHandler);
app.post('/payment', paymentHandler)

app.listen(3010, () => {
    console.log('Node Application Started');
});


function indexHandler(req, res) {
    res.send('Node Payment System Online');
}

function paymentHandler(req, res) {
    console.log('Received payment request');
    const paymentInfo = req.body;
    console.log(req.body);

    processPayment(paymentInfo);

    res.sendStatus(200);
}
