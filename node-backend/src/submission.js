'use strict';

function processPayment(payment) {
    const total = calculateTotal(payment);
    const message = `${total} USD has been successfully wired to ${payment.name}`;

    console.log(message); // <- actual transfer occurs here

    return message;
}

function calculateTotal(payment) {
    return payment.q1 + payment.q2 + payment.q3 + payment.q4;
}


module.exports = {
    processPayment,
    calculateTotal
};